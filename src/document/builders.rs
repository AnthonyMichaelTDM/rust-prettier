use std::{collections::VecDeque, num::NonZeroUsize};

use super::{Align, Doc, DocCommand, LineType};
use crate::common::Symbol;

/// create a `Doc::Array` from a list of `Doc`'s
#[must_use]
pub fn concat(docs: impl AsRef<[Doc]>) -> Doc {
    Doc::from(docs.as_ref())
}

/// increase the level of indentation
#[must_use]
pub fn indent(contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::Indent {
        contents: Box::new(contents),
    })
}

/// Increase the indentation by a fixed number of spaces or a string. A variant of indent.
///
/// When `use_tabs` is enabled, trailing alignments in indentation are still spaces, but middle ones are transformed one tab per align. In a whitespace-sensitive context (e.g., Markdown), you should pass spaces to align as strings to prevent their replacement with tabs.
/// note: the above isn't implemented yet
#[must_use]
pub fn align(contents: Doc, alignment: Align) -> Doc {
    Doc::DocCommand(DocCommand::Align {
        contents: Box::new(contents),
        alignment,
    })
}

/// Decrease the current indentation to the root marked by `mark_as_root`.
#[must_use]
pub fn dedent_to_root(contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::Align {
        contents: Box::new(contents),
        alignment: Align::ToRoot,
    })
}

/// Mark the current indentation as root for `dedent_to_root` and literallines.
#[must_use]
pub fn mark_as_root(contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::Align {
        contents: Box::new(contents),
        alignment: Align::AsRoot,
    })
}

/// Decrease the level of indentation. (Each align is considered one level of indentation.)
#[must_use]
pub fn dedent(contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::Align {
        contents: Box::new(contents),
        alignment: Align::By(-1),
    })
}

/// Mark a group of items which the printer should try to fit on one line. This is the basic command to tell the printer when to break.
/// Groups are usually nested, and the printer will try to fit everything on one line,
/// but if it doesn't fit it will break the outermost group first and try again.
/// It will continue breaking groups until everything fits (or there are no more groups to break).
///
/// A group is forced to break if it's created with the `should_break` option set to true or if it includes breakParent.
/// A hard and literal line breaks automatically include this so they always break parent groups.
/// Breaks are propagated to all parent groups, so if a deeply nested expression has a hard break, everything will break.
/// This only matters for "hard" breaks, i.e. newlines that are printed no matter what and can be statically analyzed.
///
/// For example, an array will try to fit on one line:
/// ```js
/// [1, "foo", { bar: 2 }];
/// ```
/// However, if any of the items inside the array have a hard break, the array will always break as well:
/// ```js
/// [
///   1,
///   function () {
///     return 2;
///   },
///   3,
/// ];
/// ```
///
/// Functions always break after the opening curly brace no matter what, so the array breaks as well for consistent formatting.
/// See the implementation of `ArrayExpression` for an example.
///
/// The `id` option can be used in `if_break` checks.
///
/// # Example
/// ```
/// use rust_prettier::{document::builders::*, PrettyPrinter};
///
/// let doc = group(
///     concat([
///         "[".into(),indent(group(concat([ softline(),
///             "1,".into(), softline(),concat([
///             "function () {".into(),indent(hardline()),
///                 "return 2;".into(),dedent(hardline()),
///             "},".into(),]), softline(),
///             "3,".into(),dedent(softline()),]),None,false,None,)),
///         "];".into(),
///     ]),
///     None,
///     false,
///     None,
/// );
///
/// let options = PrettyPrinter::default();
///
/// let formatted = doc.format(&options).unwrap();
///
/// assert_eq!(formatted,
/// r#"[
///     1,
///     function () {
///         return 2;
///     },
///     3,
/// ];"#);
/// ```
#[must_use]
pub fn group(
    contents: Doc,
    id: Option<Symbol>,
    should_break: bool,
    expanded_states: Option<Vec<Doc>>,
) -> Doc {
    Doc::DocCommand(DocCommand::Group {
        id,
        contents: Box::new(contents),
        break_: should_break.into(),
        expanded_states,
    })
}

/// This should be used as last resort as it triggers an exponential complexity when nested.
///
/// This will try to print the first alternative, if it fit use it, otherwise go to the next one and so on.
/// The alternatives is an array of documents going from the least expanded (most flattened) representation first to the most expanded.
///
/// # None
/// Returns none if states is empty.
#[must_use]
pub fn conditional_group(
    states: impl AsRef<[Doc]>,
    id: Option<Symbol>,
    should_break: bool,
) -> Option<Doc> {
    let states = states.as_ref();
    if states.is_empty() {
        return None;
    }
    #[allow(clippy::unwrap_used)] // safe because states is not empty
    Some(group(
        states.first()?.clone(),
        id,
        should_break,
        Some(states.to_owned()),
    ))
}

/// This is an alternative type of group which behaves like text layout:
/// it's going to add a break whenever the next element doesn't fit in the line anymore.
/// The difference with group is that it's not going to break all the separators, just the ones that are at the end of lines.
///
/// Expects the docs argument to be an array of alternating content and line breaks.
/// In other words, elements with odd indices must be line breaks (e.g., softline).
///
/// # Example
/// ```
/// use rust_prettier::{PrettyPrinterBuilder, document::builders::*};
///
/// let doc = fill([
///     "a".repeat(40).into(),
///     softline(),
///     "b".repeat(40).into(),
///     softline(),
///     "c".repeat(60).into(),
///     softline(),
///     "d".repeat(80).into(),
///     softline(), // trailing soft lines are ignored
/// ]);
///
/// let formatted = doc.format(&PrettyPrinterBuilder::default().print_width(80).build().unwrap()).unwrap();
///
/// assert_eq!(formatted, format!("{}{}\n{}\n{}", "a".repeat(40), "b".repeat(40),"c".repeat(60),"d".repeat(80)))
/// ```
///
#[must_use]
pub fn fill(parts: impl AsRef<[Doc]>) -> Doc {
    Doc::DocCommand(DocCommand::Fill {
        parts: parts.as_ref().iter().cloned().collect::<VecDeque<_>>(),
    })
}

/// Print something if the current group or the current element of fill breaks and something else if it doesn't.
///
/// ```no_run
/// use rust_prettier::document::{Doc, builders::if_break};
///
/// if_break(Doc::from(";"), Some(Doc::from(" ")), None);
/// ```
///  
/// `group_id` can be used to check another *already printed* group instead of the current group.
#[must_use]
pub fn if_break(break_contents: Doc, flat_contents: Option<Doc>, group_id: Option<Symbol>) -> Doc {
    Doc::DocCommand(DocCommand::IfBreak {
        break_contents: Box::new(break_contents),
        flat_contents: Box::new(flat_contents.unwrap_or(Doc::String(String::new()))),
        group_id,
    })
}

/// An optimized version of `if_break(indent(doc), doc, { groupId })`.
///
/// With negate: true, corresponds to `if_break(doc, indent(doc), { groupId })`
///
/// It doesn't make sense to apply `indentIfBreak` to the current group because "indent if the current group is broken" is the normal behavior of indent. That's why groupId is required.
#[must_use]
pub fn indent_if_break(contents: Doc, group_id: Symbol, negate: bool) -> Doc {
    Doc::DocCommand(DocCommand::IndentIfBreak {
        contents: Box::new(contents),

        group_id: Some(group_id),
        negate,
    })
}

/// This is used to implement trailing comments. It's not practical to constantly check where the line ends to avoid accidentally printing some code at the end of a comment. lineSuffix buffers docs passed to it and flushes them before any new line.
/// For example,
/// ```
/// use rust_prettier::document::{Doc, builders::{line_suffix, hardline}};
/// use rust_prettier::{PrettyPrinter};
///
/// let doc: Doc = Doc::from(vec!["a".into(), line_suffix(" // comment".into()), ";".into(), hardline()]);
///
/// assert_eq!(doc.format(&PrettyPrinter::default()).unwrap(), "a; // comment\n");
/// ```
#[must_use]
pub fn line_suffix(contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::LineSuffix {
        contents: Box::new(contents),
    })
}

/// In cases where you embed code inside of templates,
/// comments shouldn't be able to leave the code part.
/// `line_suffix_boundary` is an explicit marker you can use to flush the
/// `line_suffix` buffer in addition to line breaks.
///
/// For Example:
///
/// ```
/// use rust_prettier::{PrettyPrinter, document::builders::*};
///
/// let doc = concat(["{".into(), line_suffix(" // comment".into()), line_suffix_boundary(), "}".into(), hardline()]);
///
/// assert_eq!(doc.format(&PrettyPrinter::default()).unwrap(), "{ // comment\n}\n");
/// // not
/// assert_ne!(doc.format(&PrettyPrinter::default()).unwrap(), "{} // comment\n");
/// ```
#[must_use]
pub const fn line_suffix_boundary() -> Doc {
    Doc::DocCommand(DocCommand::LineSuffixBoundary)
}

/// Include this anywhere to force all parent groups to break. See [`group`] for more info.
/// # Example
/// ```no_run
/// use rust_prettier::document::{builders::*, Doc};
///
/// # let some_doc = Doc::from("some doc");
/// # let some_other_doc = Doc::from("some other doc");
///
/// let doc = group(
///     concat([
///         some_doc,
///         softline(),
///         some_other_doc,
///         softline(),
///         break_parent(),
///     ]),
///     None, false, None,
/// );
#[must_use]
pub const fn break_parent() -> Doc {
    Doc::DocCommand(DocCommand::BreakParent)
}

/// Trim all the indentation on the current line.
/// This can be used for preprocessor directives.
/// Should be placed after a line break.
#[must_use]
pub const fn trim() -> Doc {
    Doc::DocCommand(DocCommand::Trim)
}

/// This is used very rarely, for advanced formatting tricks.
/// Unlike its "normal" counterpart, it doesn't include an implicit `break_parent`.
#[must_use]
pub const fn hardline_without_break_parent() -> Doc {
    Doc::DocCommand(DocCommand::Line(LineType::Hard))
}

/// This is used very rarely, for advanced formatting tricks.
/// Unlike its "normal" counterpart, it doesn't include an implicit `break_parent`.
#[must_use]
pub const fn literalline_without_break_parent() -> Doc {
    Doc::DocCommand(DocCommand::Line(LineType::Literal))
}

#[must_use]
/// Specify a line break. The difference from `line` is that if the expression fits on one line,
/// it will be replaced with nothing.
pub const fn softline() -> Doc {
    Doc::DocCommand(DocCommand::Line(LineType::Soft))
}

#[must_use]
/// Specify a line break that is **always** included in the output,
/// no matter if the expression fits on one line or not.
pub fn hardline() -> Doc {
    Doc::Array(vec![hardline_without_break_parent(), break_parent()])
}

#[must_use]
/// Specify a line break that is **always** included in the output and doesn't indent the next line. Also, unlike `hardline`,
/// this kind of line break preserves trailing whitespace on the line it ends. This is used for template literals.
pub fn literalline() -> Doc {
    Doc::Array(vec![literalline_without_break_parent(), break_parent()])
}

#[must_use]
/// Specify a line break. If an expression fits on one line, the line break will be replaced with a space.
/// Line breaks always indent the next line with the current level of indentation.
pub const fn line() -> Doc {
    Doc::DocCommand(DocCommand::Line(LineType::Soft))
}

/// This is a placeholder value where the cursor is in the original input in order to find where it would be printed.
#[must_use]
pub const fn cursor() -> Doc {
    Doc::DocCommand(DocCommand::Cursor)
}

/// Join an array of docs with a separator.
pub fn join(separator: &Doc, docs: impl AsRef<[Doc]>) -> Doc {
    Doc::Array(docs.as_ref().iter().fold(Vec::new(), |mut acc, doc| {
        if !acc.is_empty() {
            acc.push(separator.clone());
        }
        acc.push(doc.clone());
        acc
    }))
}

/// add `size` spaces/tabs to the current indentation level, greadily using tabs first and then spaces with what is left over.
/// alignment is relative to root and doesn't stack with other alignments from `add_alignment_to_doc`
#[must_use]
pub fn add_alignment_to_doc(doc: Doc, size: usize, tab_width: NonZeroUsize) -> Doc {
    let mut aligned = doc;

    let size = size.clamp(0, isize::MAX as usize);

    // Use indent to add tabs for all the levels of tabs we need
    for _ in 0..size / tab_width.get() {
        aligned = indent(aligned);
    }
    // Use align for all the spaces that are needed
    aligned = align(aligned, Align::By((size % tab_width.get()) as isize));
    // size is absolute from 0 and not relative to the current
    // indentation, so we use ToRoot to reset the indentation to 0
    aligned = align(aligned, Align::ToRoot);

    aligned
}

/// Mark a doc with an arbitrary [`Symbol`].
/// This doesn't affect how the doc is printed, but can be useful for heuristics based on doc introspection.
///
/// E.g., to decide how to print an assignment expression,
/// we might want to know whether its right-hand side has been printed as a method call chain, not as a plain function call.
/// If the method chain printing code uses `label` to mark its result,
/// checking that condition can be as easy as:
///
/// ```ignore
/// matches!(right_hand_side_doc, Doc::DocCommand(DocCommand::Label{Symbol::String(s)}) if s == "method-chain")
/// ```
#[must_use]
pub fn label(label: Symbol, contents: Doc) -> Doc {
    Doc::DocCommand(DocCommand::Label {
        label,
        contents: Box::new(contents),
    })
}
