use std::num::NonZeroUsize;

use super::{Align, Doc, DocCommand, Label, LineType, ID};

/// increase the level of indentation
pub fn indent(contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::Indent {
        contents: Box::new(contents),
    })
}

/// Increase the indentation by a fixed number of spaces or a string. A variant of indent.
///
/// When useTabs is enabled, trailing alignments in indentation are still spaces, but middle ones are transformed one tab per align. In a whitespace-sensitive context (e.g., Markdown), you should pass spaces to align as strings to prevent their replacement with tabs.
pub fn align(contents: Doc, alignment: Align) -> Doc {
    Doc::DocCommand(DocCommand::Align {
        contents: Box::new(contents),
        alignment,
    })
}

/// Mark a group of items which the printer should try to fit on one line. This is the basic command to tell the printer when to break. Groups are usually nested, and the printer will try to fit everything on one line, but if it doesn't fit it will break the outermost group first and try again. It will continue breaking groups until everything fits (or there are no more groups to break).
///
/// A group is forced to break if it's created with the should_break option set to true or if it includes breakParent. A hard and literal line breaks automatically include this so they always break parent groups. Breaks are propagated to all parent groups, so if a deeply nested expression has a hard break, everything will break. This only matters for "hard" breaks, i.e. newlines that are printed no matter what and can be statically analyzed.
///
/// For example, an array will try to fit on one line:
/// ```no_run
/// [1, "foo", { bar: 2 }];
/// ```
/// However, if any of the items inside the array have a hard break, the array will always break as well:
/// ```no_run
/// [
///   1,
///   function () {
///     return 2;
///   },
///   3,
/// ];
/// ```
/// Functions always break after the opening curly brace no matter what, so the array breaks as well for consistent formatting. See the implementation of ArrayExpression for an example.
///
/// The `id`` option can be used in if_break checks.
pub fn group(
    contents: Doc,
    id: Option<ID>,
    should_break: bool,
    expanded_states: Option<Vec<Box<Doc>>>,
) -> Doc {
    Doc::DocCommand(DocCommand::Group {
        id,
        contents: Box::new(contents),
        should_break: should_break.into(),
        expanded_states,
    })
}

/// Decrease the current indentation to the root marked by mark_as_root.
pub fn dedent_to_root(contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::Align {
        contents: Box::new(contents),
        alignment: Align::ToRoot,
    })
}

/// Mark the current indentation as root for dedent_to_root and literallines.
pub fn mark_as_root(contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::Align {
        contents: Box::new(contents),
        alignment: Align::AsRoot,
    })
}

/// Decrease the level of indentation. (Each align is considered one level of indentation.)
pub fn dedent(contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::Align {
        contents: Box::new(contents),
        alignment: Align::By(-1),
    })
}

/// This should be used as last resort as it triggers an exponential complexity when nested.
///
/// This will try to print the first alternative, if it fit use it, otherwise go to the next one and so on. The alternatives is an array of documents going from the least expanded (most flattened) representation first to the most expanded.
///
/// # None
/// Returns none if states is empty.
pub fn conditional_group(states: Vec<Box<Doc>>, id: Option<ID>, should_break: bool) -> Option<Doc> {
    if states.is_empty() {
        return None;
    }
    #[allow(clippy::unwrap_used)] // safe because states is not empty
    Some(group(
        states.first()?.as_ref().clone(),
        id,
        should_break,
        Some(states),
    ))
}

/// This is an alternative type of group which behaves like text layout: it's going to add a break whenever the next element doesn't fit in the line anymore. The difference with group is that it's not going to break all the separators, just the ones that are at the end of lines.
///
/// Expects the docs argument to be an array of alternating content and line breaks. In other words, elements with odd indices must be line breaks (e.g., softline).
pub fn fill(parts: Vec<Box<Doc>>) -> Doc {
    Doc::DocCommand(DocCommand::Fill {
        parts: parts.into(),
    })
}

/// Print something if the current group or the current element of fill breaks and something else if it doesn't.
///
/// ```no_run
/// if_break(Doc::from(";"), Doc::from(" "));
/// ```
///  
/// group_id can be used to check another *already printed* group instead of the current group.
pub fn if_break(break_contents: Doc, flat_contents: Option<Doc>, group_id: Option<ID>) -> Doc {
    Doc::DocCommand(DocCommand::IfBreak {
        break_contents: Box::new(break_contents),
        flat_contents: flat_contents.map(Box::new),
        group_id,
    })
}

/// An optimized version of if_break(indent(doc), doc, { groupId }).
///
/// With negate: true, corresponds to if_break(doc, indent(doc), { groupId })
///
/// It doesn't make sense to apply indentIfBreak to the current group because "indent if the current group is broken" is the normal behavior of indent. That's why groupId is required.
pub fn indent_if_break(contents: Doc, group_id: ID, negate: bool) -> Doc {
    Doc::DocCommand(DocCommand::IndentIfBreak {
        contents: Box::new(contents),

        group_id: Some(group_id),
        negate,
    })
}

/// This is used to implement trailing comments. It's not practical to constantly check where the line ends to avoid accidentally printing some code at the end of a comment. lineSuffix buffers docs passed to it and flushes them before any new line.
/// For example,
/// ```no_run
/// ["a", lineSuffix(" // comment"), ";", hardline];
/// ```
/// will output
/// ```no_run
/// a; // comment
/// ```
pub fn line_suffix(contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::LineSuffix {
        contents: Box::new(contents),
    })
}

pub const fn line_suffix_boundary() -> Doc {
    Doc::DocCommand(DocCommand::LineSuffixBoundary)
}

pub const fn break_parent() -> Doc {
    Doc::DocCommand(DocCommand::BreakParent)
}

pub const fn trim() -> Doc {
    Doc::DocCommand(DocCommand::Trim)
}

pub const fn hardline_without_break_parent() -> Doc {
    Doc::DocCommand(DocCommand::Line(LineType::Hard))
}

pub const fn literalline_without_break_parent() -> Doc {
    Doc::DocCommand(DocCommand::Line(LineType::Literal))
}

pub const fn softline() -> Doc {
    Doc::DocCommand(DocCommand::Line(LineType::Soft))
}

pub fn hardline() -> Doc {
    Doc::Array(vec![
        hardline_without_break_parent().into(),
        break_parent().into(),
    ])
}

pub fn literalline() -> Doc {
    Doc::Array(vec![
        literalline_without_break_parent().into(),
        break_parent().into(),
    ])
}

pub const fn line() -> Doc {
    Doc::DocCommand(DocCommand::Line(LineType::Soft))
}

pub const fn cursor() -> Doc {
    Doc::DocCommand(DocCommand::Cursor)
}

pub fn join(separator: Doc, docs: impl AsRef<[Doc]>) -> Doc {
    Doc::Array(docs.as_ref().into_iter().fold(Vec::new(), |mut acc, doc| {
        if !acc.is_empty() {
            acc.push(Box::new(separator.clone()));
        }
        acc.push(Box::new(doc.clone()));
        acc
    }))
}

pub fn add_alignment_to_doc(doc: Doc, size: isize, tab_width: NonZeroUsize) -> Doc {
    let mut aligned = doc;
    if size > 0 {
        // Use indent to add tabs for all the levels of tabs we need
        // casting is safe because we know size is positive
        for _ in 0..(size as usize / tab_width) {
            aligned = indent(aligned);
        }
        // Use align for all the spaces that are needed
        aligned = align(aligned, Align::By((size as usize % tab_width) as isize));
        // size is absolute from 0 and not relative to the current
        // indentation, so we use -Infinity to reset the indentation to 0
        aligned = align(aligned, Align::ToRoot);
    }
    aligned
}

pub fn label(label: Label, contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::Label {
        label,
        contents: Box::new(contents),
    })
}
