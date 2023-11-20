//! TODO: a lot of stuff in this file uses more allocations than necessary, refactor into member functions on Doc and optimize w/ mutable references where applicable

mod traverse_docs;

use std::collections::{HashSet, VecDeque};

pub use traverse_docs::{traverse_doc, traverse_doc_mut};

use super::{join, Break, Doc, DocCommand, LineType};

/// Applies the given function `f` to each node in the `doc` tree, returning a new `Doc` tree with the results.
///
/// If the same subtree is found multiple times within the `doc` tree, the mapping results are cached in a map and reused.
///
/// # Arguments
///
/// * `doc` - The `Doc` tree to map over.
/// * `f` - The function to apply to each node in the `doc` tree.
///
/// # Returns
///
/// A new `Doc` tree with the results of applying the given function `f` to each node in the `doc` tree.
pub fn map_doc(doc: &Doc, f: impl Fn(&Doc) -> Doc) -> Doc {
    if let Doc::String(_) = doc {
        return f(doc);
    }

    // this is an example of TopDown memoization

    // Within a doc tree, the same subtrees can be found multiple times.
    // E.g., often this happens in conditional groups.
    // As an optimization (those subtrees can be huge) and to maintain the
    // reference structure of the tree, the mapping results are cached in
    // a map and reused.
    let mut cache = std::collections::HashMap::new();

    return map_doc_recursive(doc, &f, &mut cache);

    fn map_doc_recursive(
        doc: &Doc,
        f: &impl Fn(&Doc) -> Doc,
        cache: &mut std::collections::HashMap<Doc, Doc>,
    ) -> Doc {
        if let Some(cached) = cache.get(&doc) {
            return cached.clone();
        }

        let result = process(doc, f, cache);
        cache.insert(doc.clone(), result.clone());
        result
    }

    fn process(
        doc: &Doc,
        f: &impl Fn(&Doc) -> Doc,
        cache: &mut std::collections::HashMap<Doc, Doc>,
    ) -> Doc {
        match doc {
            Doc::Array(parts) => f(&Doc::Array(
                parts
                    .into_iter()
                    .map(|d| map_doc_recursive(d, f, cache).into())
                    .collect::<Vec<_>>(),
            )),
            Doc::DocCommand(DocCommand::Fill { parts }) => f(&Doc::DocCommand(DocCommand::Fill {
                parts: parts
                    .into_iter()
                    .map(|d| map_doc_recursive(d, f, cache).into())
                    .collect::<VecDeque<_>>(),
            })),
            Doc::DocCommand(DocCommand::IfBreak {
                break_contents,
                flat_contents,
                group_id,
            }) => f(&Doc::DocCommand(DocCommand::IfBreak {
                break_contents: Box::new(map_doc_recursive(break_contents, f, cache)),
                flat_contents: Box::new(map_doc_recursive(flat_contents, f, cache)),
                group_id: group_id.clone(),
            })),
            Doc::DocCommand(DocCommand::Group {
                expanded_states,
                contents,
                id,
                should_break,
            }) => {
                let (expanded_states, contents) = match expanded_states {
                    Some(states) => (
                        Some(
                            states
                                .into_iter()
                                .map(|d| map_doc_recursive(d, f, cache).into())
                                .collect::<Vec<_>>(),
                        ),
                        map_doc_recursive(contents, f, cache),
                    ),
                    None => (None, map_doc_recursive(contents, f, cache)),
                };
                f(&Doc::DocCommand(DocCommand::Group {
                    expanded_states,
                    contents: Box::new(contents),
                    id: id.clone(),
                    should_break: should_break.to_owned(),
                }))
            }
            Doc::DocCommand(DocCommand::Align {
                contents,
                alignment,
            }) => f(&Doc::DocCommand(DocCommand::Align {
                contents: Box::new(map_doc_recursive(contents, f, cache)),
                alignment: alignment.to_owned(),
            })),
            Doc::DocCommand(DocCommand::IndentIfBreak {
                contents,
                group_id,
                negate,
            }) => f(&Doc::DocCommand(DocCommand::IndentIfBreak {
                contents: Box::new(map_doc_recursive(contents, f, cache)),
                group_id: group_id.clone(),
                negate: negate.to_owned(),
            })),
            Doc::DocCommand(DocCommand::Label { contents, label }) => {
                f(&Doc::DocCommand(DocCommand::Label {
                    contents: Box::new(map_doc_recursive(contents, f, cache)),
                    label: label.to_owned(),
                }))
            }
            Doc::DocCommand(DocCommand::Indent { contents }) => {
                f(&Doc::DocCommand(DocCommand::Indent {
                    contents: Box::new(map_doc_recursive(contents, f, cache)),
                }))
            }
            Doc::DocCommand(DocCommand::LineSuffix { contents }) => {
                f(&Doc::DocCommand(DocCommand::LineSuffix {
                    contents: Box::new(map_doc_recursive(contents, f, cache)),
                }))
            }
            Doc::String(_)
            | Doc::DocCommand(DocCommand::Cursor)
            | Doc::DocCommand(DocCommand::Trim)
            | Doc::DocCommand(DocCommand::LineSuffixBoundary)
            | Doc::DocCommand(DocCommand::Line(_))
            | Doc::DocCommand(DocCommand::BreakParent) => f(doc),
        }
    }
}
/// Finds a node in the document tree that satisfies the given predicate function.
///
/// # Arguments
///
/// * `doc` - The root node of the document tree to search in.
/// * `f` - A closure that takes a reference to a `Doc` node and returns an `Option<Doc>`.
///          The closure should return `true` if the node satisfies the search criteria,
///          or `false` otherwise.
///
/// # Returns
///
/// Returns an `Option<Doc>` representing the first node in the document tree that satisfies
/// the search criteria, or `None` if no such node is found.
pub fn find_in_doc<State>(
    doc: &Doc,
    state: &mut State,
    f: impl Fn(&Doc, &mut State) -> bool,
) -> Option<Doc> {
    let mut result = None;
    let mut should_skip_further_processing = false;

    traverse_doc(
        &mut doc.clone(),
        &mut (&mut should_skip_further_processing, &mut result, state),
        |doc, (should_skip_further_processing, result, state)| {
            if **should_skip_further_processing {
                return false;
            }

            if f(doc, state) {
                **result = Some(doc.to_owned());
                **should_skip_further_processing = true;
                return false;
            }

            true
        },
        None,
        None,
    );

    result
}

pub fn will_break(doc: &Doc) -> bool {
    find_in_doc(doc, &mut (), |d, _| match d {
        Doc::DocCommand(DocCommand::Group {
            should_break: Break::Yes,
            ..
        }) => true,
        Doc::DocCommand(DocCommand::Line(LineType::Hard)) => true,
        Doc::DocCommand(DocCommand::BreakParent) => true,
        _ => false,
    })
    .is_some()
}

fn break_parent_group(group_stack: &mut [Doc]) {
    if !group_stack.is_empty() {
        let parent_group = group_stack.last_mut().unwrap();
        // Breaks are not propagated through conditional groups because
        // the user is expected to manually handle what breaks.
        if let Doc::DocCommand(DocCommand::Group {
            expanded_states,
            should_break,
            ..
        }) = parent_group
        {
            if (expanded_states.is_none() || expanded_states.as_ref().unwrap().is_empty())
                && *should_break == Break::Never
            {
                // An alternative truthy value allows to distinguish propagated group breaks
                // and not to print them as `group(..., { break: true })` in `--debug-print-doc`.
                *should_break = Break::Propagated;
            }
        }
    }
}

pub fn propagate_breaks(doc: &mut Doc) {
    let mut already_visited = HashSet::new();
    let mut group_stack = Vec::new();

    traverse_doc_mut(
        doc,
        &mut (&mut group_stack, &mut already_visited),
        |d, (group_stack, already_visited)| {
            if let Doc::DocCommand(DocCommand::BreakParent) = d {
                break_parent_group(group_stack);
            }
            if let Doc::DocCommand(DocCommand::Group { .. }) = d {
                group_stack.push(d.clone());
                if already_visited.contains(&*d) {
                    return false;
                }
                already_visited.insert(d.clone());
            }
            true
        },
        Some(Box::new(|d, (group_stack, _)| {
            if let Doc::DocCommand(DocCommand::Group { should_break, .. }) = d {
                group_stack.pop();
                if *should_break != Break::Yes {
                    break_parent_group(group_stack);
                }
            }
        })),
        Some(true),
    );
}

pub fn remove_lines(doc: &Doc) -> Doc {
    return map_doc(doc, |d| match d {
        // Force this doc into flat mode by statically converting all
        // lines into spaces (or soft lines into nothing). Hard lines
        // should still output because there's too great of a chance
        // of breaking existing assumptions otherwise.
        Doc::DocCommand(DocCommand::Line(LineType::Soft)) => Doc::String("".to_string()),
        Doc::DocCommand(DocCommand::Line(LineType::Literal)) => Doc::String(" ".to_string()),
        Doc::DocCommand(DocCommand::IfBreak { flat_contents, .. }) => {
            flat_contents.as_ref().clone()
        }
        Doc::DocCommand(DocCommand::Line(LineType::Hard)) | _ => d.clone(),
    });
}

fn strip_trailing_hardline_from_parts(parts: Vec<Box<Doc>>) -> Vec<Box<Doc>> {
    let mut parts = parts.to_vec();

    while parts.len() >= 2
        && matches!(
            *parts[parts.len() - 2],
            Doc::DocCommand(DocCommand::Line(_))
        )
        && *parts[parts.len() - 1] == Doc::DocCommand(DocCommand::BreakParent)
    {
        parts.pop();
        parts.pop();
    }

    if !parts.is_empty() {
        #[allow(clippy::unwrap_used)] // safe because we checked that parts is not empty
        let last_part = strip_trailing_hardline_from_doc(&parts.last().unwrap());
        *parts.last_mut().unwrap() = Box::new(last_part);
    }

    parts
}

fn strip_trailing_hardline_from_doc(doc: &Doc) -> Doc {
    match doc {
        Doc::DocCommand(DocCommand::Align {
            contents,
            alignment,
        }) => Doc::DocCommand(DocCommand::Align {
            contents: Box::new(strip_trailing_hardline_from_doc(&contents)),
            alignment: alignment.to_owned(),
        }),
        Doc::DocCommand(DocCommand::Indent { contents }) => Doc::DocCommand(DocCommand::Indent {
            contents: Box::new(strip_trailing_hardline_from_doc(&contents)),
        }),
        Doc::DocCommand(DocCommand::IndentIfBreak {
            contents,
            group_id,
            negate,
        }) => Doc::DocCommand(DocCommand::IndentIfBreak {
            contents: Box::new(strip_trailing_hardline_from_doc(&contents)),
            group_id: group_id.clone(),
            negate: negate.to_owned(),
        }),
        Doc::DocCommand(DocCommand::Group {
            id,
            contents,
            should_break,
            expanded_states,
        }) => Doc::DocCommand(DocCommand::Group {
            id: id.clone(),
            contents: Box::new(strip_trailing_hardline_from_doc(&contents)),
            should_break: should_break.to_owned(),
            expanded_states: expanded_states.clone(),
        }),
        Doc::DocCommand(DocCommand::LineSuffix { contents }) => {
            Doc::DocCommand(DocCommand::LineSuffix {
                contents: Box::new(strip_trailing_hardline_from_doc(&contents)),
            })
        }
        Doc::DocCommand(DocCommand::Label { contents, label }) => {
            Doc::DocCommand(DocCommand::Label {
                contents: Box::new(strip_trailing_hardline_from_doc(&contents)),
                label: label.to_owned(),
            })
        }
        Doc::DocCommand(DocCommand::IfBreak {
            break_contents,
            flat_contents,
            group_id,
        }) => Doc::DocCommand(DocCommand::IfBreak {
            break_contents: Box::new(strip_trailing_hardline_from_doc(&break_contents)),
            flat_contents: Box::new(strip_trailing_hardline_from_doc(&flat_contents)),
            group_id: group_id.clone(),
        }),
        Doc::DocCommand(DocCommand::Fill { parts }) => {
            let mut parts = parts.clone();
            parts.make_contiguous();
            Doc::DocCommand(DocCommand::Fill {
                parts: strip_trailing_hardline_from_parts(parts.as_slices().0.to_vec()).into(),
            })
        }
        Doc::Array(parts) => Doc::Array(strip_trailing_hardline_from_parts(parts.to_vec())),
        Doc::String(s) => s.trim_end_matches(['\n', '\r'].as_ref()).into(),
        Doc::DocCommand(DocCommand::Cursor)
        | Doc::DocCommand(DocCommand::Trim)
        | Doc::DocCommand(DocCommand::LineSuffixBoundary)
        | Doc::DocCommand(DocCommand::Line(_))
        | Doc::DocCommand(DocCommand::BreakParent) => doc.clone(),
    }
}

pub fn strip_trailing_hardline(doc: &Doc) -> Doc {
    // HACK remove ending hardline, original PR: prettier/prettier#1984
    return strip_trailing_hardline_from_doc(&clean_doc(doc));
}

/// A safer version of `normalizeDoc`
/// - `normalizeDoc` concat strings and flat array in `fill`, while `cleanDoc` don't
/// - On array, `normalizeDoc` always return object with `parts`, `cleanDoc` may return strings
/// - `cleanDoc` also remove nested `group`s and empty `fill`/`align`/`indent`/`line-suffix`/`if-break` if possible
pub fn clean_doc(doc: &Doc) -> Doc {
    map_doc(doc, |current_doc| {
        match current_doc {
            Doc::DocCommand(DocCommand::Fill { parts }) => {
                if parts
                    .iter()
                    .all(|d| matches!(d.as_ref(), Doc::String(s) if s.is_empty()))
                {
                    return Doc::String("".to_string());
                }
            }
            Doc::DocCommand(DocCommand::Group {
                id,
                contents,
                should_break,
                expanded_states,
            }) => {
                if contents.as_ref().is_empty()
                    && Break::Never == *should_break
                    && id.is_none()
                    && (expanded_states.is_none()
                        || matches!(expanded_states, Some(states) if states.is_empty() || states.iter().all(|d| d.is_empty())))
                {
                    return Doc::String("".to_string());
                }
                // Remove nested only group
                match contents.as_ref() {
                    Doc::DocCommand(DocCommand::Group {
                        id: inner_id,
                        contents: _,
                        should_break: inner_should_break,
                        expanded_states: inner_expanded_states,
                    }) if id == inner_id
                        && should_break == inner_should_break
                        && expanded_states == inner_expanded_states =>
                    {
                        return contents.as_ref().clone();
                    }
                    _ => { /* no op */ }
                }
            }

            Doc::DocCommand(DocCommand::Align { contents, .. })
            | Doc::DocCommand(DocCommand::Indent { contents })
            | Doc::DocCommand(DocCommand::IndentIfBreak { contents, .. })
            | Doc::DocCommand(DocCommand::LineSuffix { contents }) => {
                if contents.as_ref().is_empty() {
                    return Doc::String("".to_string());
                }
            }
            Doc::DocCommand(DocCommand::IfBreak {
                break_contents,
                flat_contents,
                ..
            }) => {
                if break_contents.as_ref().is_empty() && flat_contents.as_ref().is_empty() {
                    return Doc::String("".to_string());
                }
            }
            Doc::Array(parts) => {
                // Flat array, concat strings
                let mut result = Vec::new();

                for part in parts.iter().filter(|d| !d.is_empty()) {
                    match part.as_ref() {
                        Doc::Array(parts) => {
                            result.extend(parts.to_owned());
                        }
                        Doc::String(s) => {
                            if let Some(Doc::String(last)) = result.last_mut().map(|d| &mut **d) {
                                *last += &s;
                            } else {
                                result.push(Box::new(s.to_owned().into()));
                            }
                        }
                        _ => result.push(part.clone()),
                    }
                }

                if result.is_empty() {
                    return Doc::String("".to_string());
                } else if result.len() == 1 {
                    return result[0].as_ref().clone();
                } else {
                    return Doc::Array(result);
                }
            }
            Doc::String(_)
            | Doc::DocCommand(DocCommand::Cursor)
            | Doc::DocCommand(DocCommand::Trim)
            | Doc::DocCommand(DocCommand::LineSuffixBoundary)
            | Doc::DocCommand(DocCommand::Line(_))
            | Doc::DocCommand(DocCommand::Label { .. })
            | Doc::DocCommand(DocCommand::BreakParent) => { /* no op */ }
        }
        return current_doc.clone();
    })
}

// TODO: this has too many allocations, optimize it
fn normalize_parts(parts: &[Box<Doc>]) -> Vec<Box<Doc>> {
    let mut new_parts: Vec<Box<Doc>> = Vec::new();
    let mut rest_parts: Vec<&Box<Doc>> = parts.iter().rev().collect();

    for part in parts.iter().rev() {
        match *part.as_ref() {
            Doc::Array(ref parts) => {
                rest_parts.extend(parts.iter().rev().collect::<Vec<_>>());
                continue;
            }
            Doc::String(ref s) => {
                if let Some(last_part) = new_parts.last_mut() {
                    if let Doc::String(ref mut last_s) = **last_part {
                        last_s.push_str(s);
                        continue;
                    }
                }
            }
            _ => {}
        }

        new_parts.push(part.clone());
    }

    new_parts
}

/// Run `normalizeParts` on all `fill` and `align` commands in the tree.
pub fn normalize_doc(doc: &Doc) -> Doc {
    map_doc(doc, |current_doc| match current_doc {
        // first layer
        Doc::Array(parts) => Doc::Array(normalize_parts(&parts)),
        Doc::DocCommand(DocCommand::Fill { parts }) => {
            let mut parts = parts.clone();
            parts.make_contiguous();
            Doc::DocCommand(DocCommand::Fill {
                parts: normalize_parts(parts.as_slices().0).into(),
            })
        }
        // recursive cases
        Doc::DocCommand(DocCommand::Align {
            contents,
            alignment,
        }) => Doc::DocCommand(DocCommand::Align {
            contents: Box::new(normalize_doc(&contents)),
            alignment: alignment.to_owned(),
        }),
        Doc::DocCommand(DocCommand::Indent { contents }) => Doc::DocCommand(DocCommand::Indent {
            contents: Box::new(normalize_doc(&contents)),
        }),
        Doc::DocCommand(DocCommand::IndentIfBreak {
            contents,
            group_id,
            negate,
        }) => Doc::DocCommand(DocCommand::IndentIfBreak {
            contents: Box::new(normalize_doc(&contents)),
            group_id: group_id.clone(),
            negate: negate.to_owned(),
        }),
        Doc::DocCommand(DocCommand::Group {
            id,
            contents,
            should_break,
            expanded_states,
        }) => Doc::DocCommand(DocCommand::Group {
            id: id.clone(),
            contents: Box::new(normalize_doc(&contents)),
            should_break: should_break.to_owned(),
            expanded_states: expanded_states.clone(),
        }),
        Doc::DocCommand(DocCommand::LineSuffix { contents }) => {
            Doc::DocCommand(DocCommand::LineSuffix {
                contents: Box::new(normalize_doc(&contents)),
            })
        }
        Doc::DocCommand(DocCommand::IfBreak {
            break_contents,
            flat_contents,
            group_id,
        }) => Doc::DocCommand(DocCommand::IfBreak {
            break_contents: Box::new(normalize_doc(&break_contents)),
            flat_contents: Box::new(normalize_doc(&flat_contents)),
            group_id: group_id.clone(),
        }),
        // base cases
        Doc::String(_)
        | Doc::DocCommand(DocCommand::Cursor)
        | Doc::DocCommand(DocCommand::Trim)
        | Doc::DocCommand(DocCommand::LineSuffixBoundary)
        | Doc::DocCommand(DocCommand::Line(_))
        | Doc::DocCommand(DocCommand::Label { .. })
        | Doc::DocCommand(DocCommand::BreakParent) => current_doc.clone(),
    })
}

/// for `replacement`, default to `literalline`
pub fn replace_end_of_line(doc: &Doc, replacement: Doc) -> Doc {
    map_doc(doc, |current_doc| match current_doc {
        Doc::String(s) => join(
            replacement.clone(),
            s.split('\n')
                .map(|s| Doc::from(s.to_string()))
                .collect::<Vec<_>>(),
        ),
        _ => current_doc.clone(),
    })
}

pub fn can_break(doc: &Doc) -> bool {
    return find_in_doc(
        doc,
        &mut (),
        |d, _| matches! {d, Doc::DocCommand(DocCommand::Line(_))},
    )
    .map_or(false, |_| true);
}

pub fn inherit_label(doc: &Doc, f: impl Fn(&Doc) -> Doc) -> Doc {
    match doc {
        Doc::DocCommand(DocCommand::Label { contents, label }) => {
            Doc::DocCommand(DocCommand::Label {
                contents: Box::new(f(contents)),
                label: label.to_owned(),
            })
        }
        _ => f(doc),
    }
}
