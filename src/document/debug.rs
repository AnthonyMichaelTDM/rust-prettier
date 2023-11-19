//! Implementation of the `Debug` trait for `Doc` and `DocCommand`.
//! Port of functions from debug.js

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use super::{Align, Break, Doc, DocCommand, LineType, ID};

impl Debug for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printed_symbols: HashMap<ID, String> = HashMap::new();
        let mut used_keys_for_symbols: HashSet<String> = HashSet::new();

        return write!(
            f,
            "{}",
            print_doc(
                &flatten_doc(self),
                &mut printed_symbols,
                &mut used_keys_for_symbols
            )
        );

        fn print_group_id(
            id: &ID,
            printed_symbols: &mut HashMap<ID, String>,
            used_keys_for_symbols: &mut HashSet<String>,
        ) -> String {
            if let ID::Symbol(s) = id {
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
                    format!(" #{}", counter)
                } else {
                    String::new()
                }
            );
            used_keys_for_symbols.insert(key.clone());
            printed_symbols.insert(id.clone(), format!("Symbol.for({})", key));
            return printed_symbols[&id].clone();
        }

        fn print_doc(doc: &Doc, ps: &mut HashMap<ID, String>, uk: &mut HashSet<String>) -> String {
            match doc {
                Doc::String(s) => s.clone(),
                Doc::Array(a) => {
                    if a.len() == 1 {
                        print_doc(a[0].as_ref(), ps, uk)
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
                    let options = if let Some(group_id) = group_id {
                        format!(", {{ groupId: {} }}", print_group_id(group_id, ps, uk))
                    } else {
                        String::new()
                    };

                    if let Some(flat_contents) = flat_contents {
                        format!(
                            "ifBreak({}, {}{})",
                            print_doc(break_contents, ps, uk),
                            print_doc(flat_contents, ps, uk),
                            options,
                        )
                    } else {
                        format!("ifBreak({}{})", print_doc(break_contents, ps, uk), options,)
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
                        options_parts
                            .push(format!("groupId: {}", print_group_id(group_id, ps, uk)));
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
                    if let Break::Yes = should_break {
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
                            .map(|doc| format!("{}", print_doc(doc, ps, uk)))
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
    }
}

fn flatten_doc(doc: &Doc) -> Doc {
    if doc.is_empty() {
        return Doc::String("".into());
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
                            flattened.push(Box::new(flattened_part));
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
            flat_contents: flat_contents.as_ref().map(|doc| Box::new(flatten_doc(doc))),
            group_id: group_id.clone(),
        }),
        Doc::DocCommand(DocCommand::Group {
            contents,
            id,
            should_break,
            expanded_states,
        }) => Doc::DocCommand(DocCommand::Group {
            id: id.to_owned(),
            contents: flatten_doc(contents).into(),
            should_break: should_break.to_owned(),
            expanded_states: expanded_states
                .as_ref()
                .map(|v| v.iter().map(|d| Box::new(flatten_doc(d))).collect()),
        }),
        Doc::DocCommand(DocCommand::Fill { parts }) => Doc::DocCommand(DocCommand::Fill {
            parts: parts.iter().map(|d| Box::new(flatten_doc(d))).collect(),
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
            group_id: group_id.to_owned(),
            negate: negate.to_owned(),
        }),
        Doc::DocCommand(DocCommand::Align {
            contents,
            alignment,
        }) => Doc::DocCommand(DocCommand::Align {
            contents: Box::new(flatten_doc(contents)),
            alignment: alignment.to_owned(),
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
        | Doc::DocCommand(DocCommand::Cursor)
        | Doc::DocCommand(DocCommand::Trim)
        | Doc::DocCommand(DocCommand::LineSuffixBoundary)
        | Doc::DocCommand(DocCommand::Line(_))
        | Doc::DocCommand(DocCommand::BreakParent) => doc.clone(),
    }
}
