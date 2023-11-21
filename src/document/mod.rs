pub mod builders;
pub mod debug;
pub mod printer;
pub mod utils;

use std::{collections::VecDeque, fmt::Display};

use crate::common::Symbol;

// pub use builders::{
//     add_alignment_to_doc, align, break_parent, conditional_group, cursor, dedent, dedent_to_root,
//     fill, group, hardline, hardline_without_break_parent, if_break, indent, indent_if_break, join,
//     label, line, line_suffix, line_suffix_boundary, literalline, literalline_without_break_parent,
//     mark_as_root, softline, trim,
// };
// pub use utils::{
//     can_break, find_in_doc, inherit_label, map_doc, normalize_doc, propagate_breaks, remove_lines,
//     replace_end_of_line, strip_trailing_hardline, traverse_doc, traverse_doc_mut, will_break,
// };

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Doc {
    String(String),
    Array(Vec<Doc>),
    DocCommand(DocCommand),
}

impl Doc {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::String(s) => s.is_empty(),
            Self::Array(a) => a.is_empty(),
            Self::DocCommand(_) => false,
        }
    }
}

impl From<&'_ str> for Doc {
    fn from(t: &'_ str) -> Self {
        Self::String(t.to_string())
    }
}

impl From<String> for Doc {
    fn from(t: String) -> Self {
        Self::String(t)
    }
}

impl From<Vec<Self>> for Doc {
    /// moves inner docs into the heap
    fn from(t: Vec<Self>) -> Self {
        Self::Array(t)
    }
}

impl From<&[Self]> for Doc {
    /// moves inner docs into the heap
    fn from(t: &[Self]) -> Self {
        Self::Array(t.to_vec())
    }
}

impl From<DocCommand> for Doc {
    fn from(t: DocCommand) -> Self {
        Self::DocCommand(t)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DocCommand {
    Align {
        contents: Box<Doc>,
        alignment: Align,
    },
    BreakParent,
    Cursor,
    Fill {
        parts: VecDeque<Doc>,
    },
    Group {
        id: Option<Symbol>,
        contents: Box<Doc>,
        break_: Break,
        expanded_states: Option<Vec<Doc>>,
    },
    IfBreak {
        break_contents: Box<Doc>,
        flat_contents: Box<Doc>,
        group_id: Option<Symbol>,
    },
    Indent {
        contents: Box<Doc>,
    },
    IndentIfBreak {
        contents: Box<Doc>,
        group_id: Option<Symbol>,
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
    AsRoot, // root
    ToRoot, // equivalent to original implementation's Number.NEGATIVE_INFINITY
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Break {
    Yes,
    Never,
    Propagated, // this is a weird relic, if it's not used just TODO: use bool instead of Break
}

impl Display for Break {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "{:?}", true),
            Self::Never => write!(f, "{:?}", false),
            Self::Propagated => write!(f, "propagated"),
        }
    }
}

impl From<bool> for Break {
    fn from(t: bool) -> Self {
        if t {
            Self::Yes
        } else {
            Self::Never
        }
    }
}

impl Break {
    #[must_use]
    pub const fn should_break(&self) -> bool {
        matches!(self, Self::Propagated | Self::Yes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LineType {
    Soft,
    Hard,
    Literal,
    None,
}

impl LineType {
    #[must_use]
    pub const fn is_hard(&self) -> bool {
        match self {
            Self::Hard | Self::Literal => true,
            Self::Soft | Self::None => false,
        }
    }

    #[must_use]
    pub const fn is_soft(&self) -> bool {
        match self {
            Self::Soft => true,
            Self::Hard | Self::Literal | Self::None => false,
        }
    }

    #[must_use]
    pub const fn is_literal(&self) -> bool {
        match self {
            Self::Literal => true,
            Self::Soft | Self::Hard | Self::None => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum Label {
    MethodChain,
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MethodChain => write!(f, "method-chain"),
        }
    }
}
