use std::{cell::RefCell, rc::Rc};

use crate::document::DocCommand;

use super::super::Doc;

enum TraverseDoc<'a> {
    Doc(&'a Doc),
    ExitMarker(&'a Doc),
}

pub fn traverse_doc<State, ExitCallbackFN>(
    doc: &Doc,
    state: &mut State,
    on_enter: impl Fn(&Doc, &mut State) -> bool,
    on_exit: Option<&ExitCallbackFN>,
    should_traverse_conditional_groups: Option<bool>,
) where
    ExitCallbackFN: Fn(&Doc, &mut State),
{
    let mut stack = vec![TraverseDoc::Doc(doc)];

    while !stack.is_empty() {
        let doc = match stack.pop() {
            Some(TraverseDoc::Doc(doc)) => doc,
            Some(TraverseDoc::ExitMarker(doc)) => {
                if let Some(ref exit_callback) = on_exit {
                    exit_callback(doc, state);
                }
                continue;
            }
            None => unreachable!(),
        };

        // push the doc back onto the stack so that we can
        // call the exit callback next time we see it (which will happen after all of its children have been processed)
        if on_exit.is_some() {
            stack.push(TraverseDoc::ExitMarker(doc));
        }

        // should we recurse into this doc?
        if !on_enter(doc, state) {
            continue;
        }

        // When there are multiple parts to process,
        // the parts need to be pushed onto the stack in reverse order,
        // so that they are processed in the original order
        // when the stack is popped.

        match doc {
            Doc::String(_)
            | Doc::DocCommand(
                DocCommand::Cursor { .. }
                | DocCommand::Trim
                | DocCommand::LineSuffixBoundary
                | DocCommand::Line(_)
                | DocCommand::BreakParent,
            ) => {
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
                stack.push(TraverseDoc::Doc(flat_contents));
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

            Doc::DocCommand(
                DocCommand::Align { contents, .. }
                | DocCommand::Indent { contents }
                | DocCommand::IndentIfBreak { contents, .. }
                | DocCommand::Label { contents, .. }
                | DocCommand::LineSuffix { contents },
            ) => {
                stack.push(TraverseDoc::Doc(contents));
            }
        }
    }
}

// TODO: refactor to use Rc<RefCell<Doc>> instead of Rc<RefCell<&mut Doc>>, the mutable reference is redundant
enum TraverseDocMut<'a> {
    Doc(Rc<RefCell<&'a mut Doc>>),
    ExitMarker(Rc<RefCell<&'a mut Doc>>),
}

impl<'a> TraverseDocMut<'a> {
    fn new(doc: &'a mut Doc) -> Self {
        // TraverseDocMut::Doc(Rc::new(doc))
        TraverseDocMut::Doc(Rc::new(RefCell::new(doc)))
    }
}

/// Panics if the doc is mutated while traversing.
///
/// This is a convenience function for traversing a doc and mutating it at the same time.
///
/// # Safety
///
/// To ensure the function executes without panicing, do the following
///
/// - Ensure that any given node will not deallocate before being fully explored (entered and exited)
/// - If your callbacks save the `Doc` they get as their first argument to some external state, ensure that the tree will outlive that state
///
/// # Panics
///
/// if any part of the doc tree is deallocated before being fully explored (entered and exited)
pub fn traverse_doc_mut<State, ExitCallbackFN>(
    doc: &mut Doc,
    state: &mut State,
    on_enter: impl Fn(&mut Doc, &mut State) -> bool,
    on_exit: Option<&ExitCallbackFN>,
    should_traverse_conditional_groups: Option<bool>,
) where
    ExitCallbackFN: Fn(&mut Doc, &mut State),
{
    // let mut current_doc;
    let mut stack = vec![TraverseDocMut::new(doc)];
    while !stack.is_empty() {
        let current_doc = match stack.pop() {
            Some(TraverseDocMut::Doc(doc)) => doc,
            Some(TraverseDocMut::ExitMarker(doc)) => {
                if let Some(ref exit_callback) = on_exit {
                    exit_callback(&mut doc.borrow_mut(), state);
                }
                continue;
            }
            None => unreachable!(),
        };

        // push the doc back onto the stack so that we can
        // call the exit callback next time we see it (which will happen after all of its children have been processed)
        if on_exit.is_some() {
            stack.push(TraverseDocMut::ExitMarker(current_doc.clone()));
            // unsafe { Rc::decrement_strong_count(&mut current_doc) };
        }

        // should we recurse into this doc?
        if !on_enter(&mut current_doc.borrow_mut(), state) {
            continue;
        }

        // When there are multiple parts to process,
        // the parts need to be pushed onto the stack in reverse order,
        // so that they are processed in the original order
        // when the stack is popped.
        match unsafe { current_doc.as_ptr().as_mut().unwrap() } {
            Doc::String(_)
            | Doc::DocCommand(
                DocCommand::Cursor { .. }
                | DocCommand::Trim
                | DocCommand::LineSuffixBoundary
                | DocCommand::Line(_)
                | DocCommand::BreakParent,
            ) => {
                // no children
            }
            Doc::DocCommand(DocCommand::Fill { parts }) => {
                for part in parts.iter_mut().rev() {
                    stack.push(TraverseDocMut::new(part));
                }
            }
            Doc::Array(parts) => {
                for part in parts.iter_mut().rev() {
                    stack.push(TraverseDocMut::new(part));
                }
            }
            Doc::DocCommand(DocCommand::IfBreak {
                break_contents,
                flat_contents,
                ..
            }) => {
                stack.push(TraverseDocMut::new(flat_contents));
                stack.push(TraverseDocMut::new(break_contents));
            }
            Doc::DocCommand(DocCommand::Group {
                contents,
                expanded_states,
                ..
            }) => match (should_traverse_conditional_groups, expanded_states) {
                (Some(true), Some(expanded_states)) => {
                    for expanded_state in expanded_states.iter_mut().rev() {
                        stack.push(TraverseDocMut::new(expanded_state));
                    }
                }
                _ => {
                    stack.push(TraverseDocMut::new(contents));
                }
            },

            Doc::DocCommand(
                DocCommand::Align { contents, .. }
                | DocCommand::Indent { contents }
                | DocCommand::IndentIfBreak { contents, .. }
                | DocCommand::Label { contents, .. }
                | DocCommand::LineSuffix { contents },
            ) => {
                stack.push(TraverseDocMut::new(contents));
            }
        };
    }
}

#[cfg(test)]
mod traverse_doc_mut_tests {
    use super::*;

    #[test]
    fn test_traverse_doc_mut_mutates() {
        let mut doc = Doc::Array(vec![
            Doc::String("hello".to_string()).into(),
            Doc::String("world".to_string()).into(),
        ]);

        let mut count = 0;
        traverse_doc_mut(
            &mut doc,
            &mut count,
            |doc: &mut Doc, state: &mut i32| {
                if let Doc::String(s) = doc {
                    *s = s.to_uppercase();
                }
                *state += 1;
                // println!("{:?} ", doc);
                true
            },
            Some(&|_: &mut _, state: &mut i32| {
                // println!("{:?} ", doc);
                *state += 1;
            }),
            // None,
            None,
        );

        assert_eq!(count, 6); // enter and exit for each doc
        assert_eq!(
            doc,
            Doc::Array(vec![
                Doc::String("HELLO".to_string()).into(),
                Doc::String("WORLD".to_string()).into(),
            ])
        );
    }

    #[test]
    fn test_traverse_doc_mut_mutates_enter_only() {
        let mut doc = Doc::Array(vec![
            Doc::String("hello".to_string()).into(),
            Doc::String("world".to_string()).into(),
        ]);

        let mut count = 0;
        traverse_doc_mut(
            &mut doc,
            &mut count,
            |doc, state| {
                if let Doc::String(s) = doc {
                    *s = s.to_uppercase();
                }
                *state += 1;
                // println!("{:?} ", doc);
                true
            },
            None::<&fn(&mut _, &mut _)>,
            // None,
            None,
        );

        assert_eq!(count, 3); // enter and exit for each doc
        assert_eq!(
            doc,
            Doc::Array(vec![
                Doc::String("HELLO".to_string()).into(),
                Doc::String("WORLD".to_string()).into(),
            ])
        );
    }
}
