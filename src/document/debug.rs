//! Implementation of the `Debug` trait for `Doc` and `DocCommand`.
//! Port of functions from debug.js

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use super::{Align, Break, Doc, DocCommand, LineType};
use crate::common::Symbol;

impl Debug for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printed_symbols: HashMap<Symbol, String> = HashMap::new();
        let mut used_keys_for_symbols: HashSet<String> = HashSet::new();

        write!(
            f,
            "{}",
            print_doc(
                &flatten_doc(self),
                &mut printed_symbols,
                &mut used_keys_for_symbols
            )
        )
    }
}

fn print_group_id(
    id: &Symbol,
    printed_symbols: &mut HashMap<Symbol, String>,
    used_keys_for_symbols: &mut HashSet<String>,
) -> String {
    if let Symbol::String(s) = id {
        return s.clone();
    }

    if printed_symbols.contains_key(id) {
        return printed_symbols[id].clone();
    }

    // const prefix = id.description || "symbol";
    // for (let counter = 0; ; counter++) {
    //     const key = prefix + (counter > 0 ? ` #${counter}` : "");
    //     if (!usedKeysForSymbols.has(key)) {
    //       usedKeysForSymbols.add(key);
    //       return (printedSymbols[id] = `Symbol.for(${JSON.stringify(key)})`);
    //     }
    // }
    let counter = used_keys_for_symbols.len();
    let key = format!(
        "symbol{}",
        if counter > 0 {
            format!(" #{counter}")
        } else {
            String::new()
        }
    );
    used_keys_for_symbols.insert(key.clone());
    printed_symbols.insert(id.clone(), format!("Symbol.for({key})"));
    printed_symbols[&id].clone()
}

#[allow(clippy::too_many_lines)]
fn print_doc(doc: &Doc, ps: &mut HashMap<Symbol, String>, uk: &mut HashSet<String>) -> String {
    match doc {
        Doc::String(s) => s.clone(),
        Doc::Array(a) => {
            if a.len() == 1 {
                print_doc(a.first().unwrap(), ps, uk)
            } else {
                format!(
                    "[{}]",
                    a.iter()
                        .map(|d| print_doc(d, ps, uk))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            }
        }
        Doc::DocCommand(DocCommand::Line(line_type)) => match line_type {
            LineType::Hard => "hardline".into(),
            LineType::Soft => "softline".into(),
            LineType::Literal => "literalline".into(),
            LineType::None => "line".into(),
        },
        Doc::DocCommand(DocCommand::BreakParent) => "breakParent".into(),
        Doc::DocCommand(DocCommand::Trim) => "trim".into(),
        Doc::DocCommand(DocCommand::Indent { contents }) => {
            format!("indent({})", print_doc(contents, ps, uk))
        }
        Doc::DocCommand(DocCommand::Align {
            contents,
            alignment,
        }) => match alignment {
            Align::By(n) if *n < 0 => {
                format!("dedent({})", print_doc(contents, ps, uk))
            }
            Align::By(n) => format!("align({n}, {})", print_doc(contents, ps, uk)),
            Align::With(s) => {
                format!("align(\"{s}\", {})", print_doc(contents, ps, uk))
            }
            Align::AsRoot => {
                format!("markAsRoot({})", print_doc(contents, ps, uk))
            }
            Align::ToRoot => {
                format!("dedentToRoot({})", print_doc(contents, ps, uk))
            }
        },
        Doc::DocCommand(DocCommand::IfBreak {
            break_contents,
            flat_contents,
            group_id,
        }) => {
            let options = group_id.as_ref().map_or_else(String::new, |group_id| {
                format!(", {{ groupId: {} }}", print_group_id(group_id, ps, uk))
            });

            if flat_contents.is_empty() {
                format!("ifBreak({}{})", print_doc(break_contents, ps, uk), options,)
            } else {
                format!(
                    "ifBreak({}, {}{})",
                    print_doc(break_contents, ps, uk),
                    print_doc(flat_contents, ps, uk),
                    options,
                )
            }
        }
        Doc::DocCommand(DocCommand::IndentIfBreak {
            contents,
            group_id,
            negate,
        }) => {
            let mut options_parts = Vec::new();
            if *negate {
                options_parts.push("negate: true".into());
            }
            if let Some(group_id) = group_id {
                options_parts.push(format!("groupId: {}", print_group_id(group_id, ps, uk)));
            }

            let options = if options_parts.is_empty() {
                String::new()
            } else {
                format!(", {{ {} }}", options_parts.join(", "))
            };

            format!("indentIfBreak({}{options})", print_doc(contents, ps, uk))
        }
        Doc::DocCommand(DocCommand::Group {
            id,
            contents,
            should_break,
            expanded_states,
        }) => {
            let mut options_parts = Vec::new();
            if matches!(should_break, Break::Yes) {
                options_parts.push("shouldBreak: true".into());
            }
            if let Some(id) = id {
                options_parts.push(format!("id: {}", print_group_id(id, ps, uk)));
            }

            let options = if options_parts.is_empty() {
                String::new()
            } else {
                format!(", {{ {} }}", options_parts.join(", "))
            };

            if let Some(expanded_states) = expanded_states {
                format!(
                    "conditionalGroup([{}]{options})",
                    expanded_states
                        .iter()
                        .map(|d| print_doc(d, ps, uk))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            } else {
                format!("group({}{options})", print_doc(contents, ps, uk))
            }
        }
        Doc::DocCommand(DocCommand::Fill { parts }) => {
            format!(
                "fill([{}])",
                parts
                    .iter()
                    .map(|doc| print_doc(doc, ps, uk))
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        }
        Doc::DocCommand(DocCommand::LineSuffix { contents }) => {
            format!("lineSuffix({})", print_doc(contents, ps, uk))
        }
        Doc::DocCommand(DocCommand::LineSuffixBoundary) => "lineSuffixBoundary".into(),
        Doc::DocCommand(DocCommand::Label { contents, label }) => {
            format!("label({label}, {})", print_doc(contents, ps, uk))
        }
        Doc::DocCommand(DocCommand::Cursor) => "cursor".into(),
    }
}

fn flatten_doc(doc: &Doc) -> Doc {
    if doc.is_empty() {
        return Doc::String(String::new());
    }

    match doc {
        Doc::Array(parts) => {
            let mut flattened = Vec::new();
            for part in parts {
                match flatten_doc(part) {
                    Doc::Array(flattened_parts) => {
                        flattened.extend(flattened_parts);
                    }
                    flattened_part => {
                        if !flattened_part.is_empty() {
                            flattened.push(flattened_part);
                        }
                    }
                }
            }
            Doc::Array(flattened)
        }
        Doc::DocCommand(DocCommand::IfBreak {
            break_contents,
            flat_contents,
            group_id,
        }) => Doc::DocCommand(DocCommand::IfBreak {
            break_contents: Box::new(flatten_doc(break_contents)),
            flat_contents: Box::new(flatten_doc(flat_contents)),
            group_id: group_id.clone(),
        }),
        Doc::DocCommand(DocCommand::Group {
            contents,
            id,
            should_break,
            expanded_states,
        }) => Doc::DocCommand(DocCommand::Group {
            id: id.clone(),
            contents: flatten_doc(contents).into(),
            should_break: should_break.clone(),
            expanded_states: expanded_states.clone(),
        }),
        Doc::DocCommand(DocCommand::Fill { parts }) => Doc::DocCommand(DocCommand::Fill {
            parts: parts.clone(),
        }),

        Doc::DocCommand(DocCommand::Indent { contents }) => Doc::DocCommand(DocCommand::Indent {
            contents: Box::new(flatten_doc(contents)),
        }),
        Doc::DocCommand(DocCommand::IndentIfBreak {
            contents,
            group_id,
            negate,
        }) => Doc::DocCommand(DocCommand::IndentIfBreak {
            contents: Box::new(flatten_doc(contents)),
            group_id: group_id.clone(),
            negate: negate.to_owned(),
        }),
        Doc::DocCommand(DocCommand::Align {
            contents,
            alignment,
        }) => Doc::DocCommand(DocCommand::Align {
            contents: Box::new(flatten_doc(contents)),
            alignment: alignment.clone(),
        }),
        Doc::DocCommand(DocCommand::LineSuffix { contents }) => {
            Doc::DocCommand(DocCommand::LineSuffix {
                contents: Box::new(flatten_doc(contents)),
            })
        }
        Doc::DocCommand(DocCommand::Label { contents, label }) => {
            Doc::DocCommand(DocCommand::Label {
                contents: Box::new(flatten_doc(contents)),
                label: *label,
            })
        }

        Doc::String(_)
        | Doc::DocCommand(
            DocCommand::Cursor
            | DocCommand::Trim
            | DocCommand::LineSuffixBoundary
            | DocCommand::Line(_)
            | DocCommand::BreakParent,
        ) => doc.clone(),
    }
}
