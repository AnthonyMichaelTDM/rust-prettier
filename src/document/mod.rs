mod builders;
pub mod debug;
mod printer;
mod utils;

use std::fmt::Display;

pub use builders::{
    add_alignment_to_doc, align, break_parent, conditional_group, cursor, dedent, dedent_to_root,
    fill, group, hardline, hardline_without_break_parent, if_break, indent, indent_if_break, join,
    label, line, line_suffix, line_suffix_boundary, literalline, literalline_without_break_parent,
    mark_as_root, softline, trim,
};
// pub use printer::print_doc_to_string;
pub use utils::{
    can_break, find_in_doc, inherit_label, map_doc, normalize_doc, remove_lines,
    replace_end_of_line, strip_trailing_hardline, traverse_doc, will_break,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Doc {
    String(String),
    Array(Vec<Box<Doc>>),
    DocCommand(DocCommand),
}

impl Doc {
    pub fn is_empty(&self) -> bool {
        match self {
            Doc::String(s) => s.is_empty(),
            Doc::Array(a) => a.is_empty(),
            Doc::DocCommand(_) => false,
        }
    }
}

impl From<&'_ str> for Doc {
    fn from(t: &'_ str) -> Doc {
        Doc::String(t.to_string())
    }
}

impl From<String> for Doc {
    fn from(t: String) -> Doc {
        Doc::String(t)
    }
}

impl From<Vec<Doc>> for Doc {
    /// moves inner docs into the heap
    fn from(t: Vec<Doc>) -> Doc {
        Doc::Array(t.into_iter().map(|d| Box::new(d)).collect())
    }
}

impl From<&'_ [Doc]> for Doc {
    /// moves inner docs into the heap
    fn from(t: &'_ [Doc]) -> Doc {
        Doc::Array(t.iter().map(|d| Box::new(d.clone())).collect())
    }
}

impl From<Vec<Box<Doc>>> for Doc {
    fn from(t: Vec<Box<Doc>>) -> Doc {
        Doc::Array(t)
    }
}

impl From<&'_ [Box<Doc>]> for Doc {
    fn from(t: &'_ [Box<Doc>]) -> Doc {
        Doc::Array(t.to_vec())
    }
}

impl From<DocCommand> for Doc {
    fn from(t: DocCommand) -> Doc {
        Doc::DocCommand(t)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum DocCommand {
    Align {
        contents: Box<Doc>,
        alignment: Align,
    },
    BreakParent,
    Cursor,
    Fill {
        parts: Vec<Box<Doc>>,
    },
    Group {
        id: Option<ID>,
        contents: Box<Doc>,
        should_break: Break,
        expanded_states: Option<Vec<Box<Doc>>>,
    },
    IfBreak {
        break_contents: Box<Doc>,
        flat_contents: Option<Box<Doc>>,
        group_id: Option<ID>,
    },
    Indent {
        contents: Box<Doc>,
    },
    IndentIfBreak {
        contents: Box<Doc>,
        group_id: Option<ID>,
        negate: bool,
    },
    Label {
        contents: Box<Doc>,
        label: Label,
    },
    Line(LineType),
    LineSuffix {
        contents: Box<Doc>,
    },
    LineSuffixBoundary,
    Trim,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Align {
    By(isize),
    With(String),
    AsRoot,
    ToRoot,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ID {
    // TODO: this is basically the Symbol type from the original implementation
    Symbol(String),
    Number(usize),
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ID::Symbol(s) => write!(f, "{}", s),
            ID::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Break {
    Yes,
    No,
    Propagated,
}

impl Display for Break {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Break::Yes => write!(f, "{:?}", true),
            Break::No => write!(f, "{:?}", false),
            Break::Propagated => write!(f, "propagated"),
        }
    }
}

impl From<bool> for Break {
    fn from(t: bool) -> Break {
        if t {
            Break::Yes
        } else {
            Break::No
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LineType {
    Soft,
    Hard,
    Literal,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum Label {
    MethodChain,
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Label::MethodChain => write!(f, "method-chain"),
        }
    }
}
