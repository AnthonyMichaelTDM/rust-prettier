use crate::document::DocCommand;

use super::super::Doc;

enum TraverseDoc<'a> {
    Doc(&'a Doc),
    ExitMarker(&'a Doc),
}

pub fn traverse_doc(
    doc: &Doc,
    mut on_enter: impl FnMut(&Doc) -> bool,
    on_exit: Option<Box<dyn Fn(&Doc) -> ()>>,
    should_traverse_conditional_groups: Option<bool>,
) {
    let mut stack = vec![TraverseDoc::Doc(doc)];

    while !stack.is_empty() {
        let doc = match stack.pop() {
            Some(TraverseDoc::Doc(doc)) => doc,
            Some(TraverseDoc::ExitMarker(doc)) => {
                if let Some(ref exit_callback) = on_exit {
                    exit_callback(doc);
                }
                continue;
            }
            None => unreachable!(),
        };

        // push the doc back onto the stack so that we can
        // call the exit callback next time we see it (which will happen after all of its children have been processed)
        if let Some(_) = on_exit {
            stack.push(TraverseDoc::ExitMarker(doc));
        }

        // should we recurse into this doc?
        if !on_enter(doc) {
            continue;
        }

        // When there are multiple parts to process,
        // the parts need to be pushed onto the stack in reverse order,
        // so that they are processed in the original order
        // when the stack is popped.

        match doc {
            Doc::String(_)
            | Doc::DocCommand(DocCommand::Cursor { .. })
            | Doc::DocCommand(DocCommand::Trim)
            | Doc::DocCommand(DocCommand::LineSuffixBoundary)
            | Doc::DocCommand(DocCommand::Line(_))
            | Doc::DocCommand(DocCommand::BreakParent) => {
                // no children
            }
            Doc::DocCommand(DocCommand::Fill { parts }) => {
                for part in parts.iter().rev() {
                    stack.push(TraverseDoc::Doc(part));
                }
            }
            Doc::Array(parts) => {
                for part in parts.iter().rev() {
                    stack.push(TraverseDoc::Doc(part));
                }
            }
            Doc::DocCommand(DocCommand::IfBreak {
                break_contents,
                flat_contents,
                ..
            }) => {
                if let Some(flat_contents) = flat_contents {
                    stack.push(TraverseDoc::Doc(flat_contents));
                }
                stack.push(TraverseDoc::Doc(break_contents));
            }
            Doc::DocCommand(DocCommand::Group {
                contents,
                expanded_states,
                ..
            }) => match (should_traverse_conditional_groups, expanded_states) {
                (Some(true), Some(expanded_states)) => {
                    for expanded_state in expanded_states.iter().rev() {
                        stack.push(TraverseDoc::Doc(expanded_state));
                    }
                }
                _ => {
                    stack.push(TraverseDoc::Doc(contents));
                }
            },

            Doc::DocCommand(DocCommand::Align { contents, .. })
            | Doc::DocCommand(DocCommand::Indent { contents })
            | Doc::DocCommand(DocCommand::IndentIfBreak { contents, .. })
            | Doc::DocCommand(DocCommand::Label { contents, .. })
            | Doc::DocCommand(DocCommand::LineSuffix { contents }) => {
                stack.push(TraverseDoc::Doc(contents))
            }
        }
    }
}
