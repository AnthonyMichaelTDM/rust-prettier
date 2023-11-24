use std::{cell::RefCell, collections::HashMap, rc::Rc};

use unicode_width::UnicodeWidthStr;

use crate::{
    common::{end_of_line::EndLine, Symbol},
    document::{
        builders::hardline_without_break_parent, utils::propagate_breaks, DocCommand, LineType,
    },
    PrettyPrinter,
};

use super::{Align, Doc};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Flat,
    Break,
}

#[derive(Clone)]
struct Indent {
    value: String,
    length: usize,
    queue: Vec<Part>,
    root: Option<Box<Indent>>,
}

#[derive(Clone)]
enum Part {
    Dedent,
    Indent,
    StringAlign(String),
    NumberAlign(usize),
}

#[derive(Clone)]
struct Command {
    indent: Rc<RefCell<Indent>>,
    mode: Mode,
    doc: Doc,
}

enum OutputItem {
    Content(String),
    Cursor,
}

#[derive(thiserror::Error, Debug)]
pub enum FormattingError {
    #[error("There are too many 'cursor' in doc.")]
    TooManyCursors,
}

impl Doc {
    /// This function is used to render a Doc to a String
    ///
    /// Note: before getting to this point, `options.end_of_line` should have been infered (if it was "auto", it should've been changed to match the first newline in the input text),
    /// if this is not the case, we will change it to the default here
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_prettier::document::{Doc, builders::*};
    ///
    /// let doc = concat(vec!["foo".into(), line_suffix(";".into())  ]);
    ///
    ///
    /// ```
    ///
    /// # Errors
    /// - if the tree rooted as `self` has more than 1 cursor
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    pub fn format(&self, options: &PrettyPrinter) -> Result<String, FormattingError> {
        let mut group_mode_map: HashMap<Symbol, Mode> = HashMap::new();

        #[allow(clippy::cast_possible_wrap)]
        let width = options.print_width.clamp(0, isize::MAX as usize) as isize;
        let new_line = match options.end_of_line {
            crate::config::EndOfLine::Auto => EndLine::default(),
            crate::config::EndOfLine::Lf => EndLine::Lf,
            crate::config::EndOfLine::CrLf => EndLine::CrLf,
            crate::config::EndOfLine::Cr => EndLine::Cr,
        };
        let mut pos = 0;

        let mut doc = self.clone();

        propagate_breaks(&mut doc);

        // cmds is basically a stack. We've turned a recursive call into a
        // while loop which is much faster. The while loop below adds new
        // cmds to the array instead of recursively calling `print`.
        let mut cmds = vec![Command {
            indent: Rc::new(RefCell::new(Indent::root())),
            mode: Mode::Break,
            doc,
        }];
        let mut out: Vec<OutputItem> = Vec::new();
        let mut should_remeasure = false;
        let mut line_suffix: Vec<Command> = Vec::new();
        let mut printed_cursor_count = 0;

        while let Some(Command { indent, mode, doc }) = cmds.pop() {
            match doc {
                Self::String(s) => {
                    let formatted = if new_line == EndLine::Lf {
                        s
                    } else {
                        s.replace('\n', &new_line.to_string())
                    };
                    // Plugins may print single string, should skip measure the width
                    if !cmds.is_empty() {
                        pos += UnicodeWidthStr::width(formatted.as_str());
                    }
                    out.push(OutputItem::Content(formatted));
                }
                Self::Array(parts) => {
                    for part in parts.iter().rev() {
                        cmds.push(Command {
                            indent: indent.clone(),
                            mode,
                            doc: part.clone(),
                        });
                    }
                }

                Self::DocCommand(DocCommand::Cursor) => {
                    if printed_cursor_count > 1 {
                        return Err(FormattingError::TooManyCursors);
                    }
                    out.push(OutputItem::Cursor);
                    printed_cursor_count += 1;
                }

                Self::DocCommand(DocCommand::Indent { contents }) => {
                    *indent.borrow_mut() = make_indent(indent.clone(), options);
                    // indent.replace_with(|old| make_indent(old, options));
                    cmds.push(Command {
                        indent,
                        mode,
                        doc: contents.as_ref().clone(),
                    })
                }

                Self::DocCommand(DocCommand::Align {
                    contents,
                    alignment,
                }) => {
                    *indent.borrow_mut() = make_align(indent.clone(), options, alignment);
                    // indent.replace_with(|old| make_align(old, options, alignment));
                    cmds.push(Command {
                        indent,
                        mode,
                        doc: contents.as_ref().clone(),
                    })
                }

                Self::DocCommand(DocCommand::Trim) => {
                    pos -= trim(&mut out);
                }

                Self::DocCommand(DocCommand::Group {
                    id,
                    contents,
                    break_: should_break,
                    expanded_states,
                }) => {
                    match mode {
                        Mode::Flat if !should_remeasure => {
                            cmds.push(Command {
                                indent,
                                mode: if should_break.should_break() {
                                    Mode::Break
                                } else {
                                    Mode::Flat
                                },
                                doc: contents.as_ref().clone(),
                            });
                        }
                        _ => {
                            should_remeasure = false;

                            let next = Command {
                                indent: indent.clone(),
                                mode: Mode::Flat,
                                doc: contents.as_ref().clone(),
                            };
                            #[allow(clippy::cast_possible_wrap)]
                            let rem = width - pos.clamp(0, isize::MAX as usize) as isize;
                            let has_line_suffix = !line_suffix.is_empty();

                            if !should_break.should_break()
                                && fits(&next, &cmds, rem, has_line_suffix, &group_mode_map, false)
                            {
                                cmds.push(next);
                            } else if expanded_states.is_some()
                                && !expanded_states.as_deref().unwrap_or(&[]).is_empty()
                            {
                                // Expanded states are a rare case where a document
                                // can manually provide multiple representations of
                                // itself. It provides an array of documents
                                // going from the least expanded (most flattened)
                                // representation first to the most expanded. If a
                                // group has these, we need to manually go through
                                // these states and find the first one that fits.
                                // eslint-disable-next-line no-lonely-if
                                let Some(expanded_states) = expanded_states else {
                                    // safe because we checked`` for none
                                    unreachable!()
                                };
                                #[allow(clippy::unwrap_used)]
                                let Some(most_expanded) = expanded_states.last() else {
                                    // safe because we check for empty above
                                    unreachable!()
                                };

                                if should_break.should_break() {
                                    cmds.push(Command {
                                        indent,
                                        mode: Mode::Break,
                                        doc: most_expanded.clone(),
                                    });
                                } else {
                                    for (i, state) in expanded_states.iter().enumerate() {
                                        if i + 1 >= expanded_states.len() {
                                            cmds.push(Command {
                                                indent,
                                                mode: Mode::Break,
                                                doc: state.clone(),
                                            });
                                            break;
                                        }
                                        let command = Command {
                                            indent: indent.clone(),
                                            mode: Mode::Flat,
                                            doc: state.clone(),
                                        };

                                        if fits(
                                            &command,
                                            &cmds,
                                            rem,
                                            has_line_suffix,
                                            &group_mode_map,
                                            false,
                                        ) {
                                            cmds.push(command);
                                            break;
                                        }
                                    }
                                }
                            } else {
                                cmds.push(Command {
                                    indent,
                                    mode: Mode::Break,
                                    doc: contents.as_ref().clone(),
                                });
                            }
                        }
                    }

                    if let (Some(id), Some(cmd)) = (id, cmds.last()) {
                        group_mode_map.insert(id, cmd.mode);
                    }
                }
                // # Original Implementation Notes
                // Fills each line with as much code as possible before moving to a new
                // line with the same indentation.
                //
                // Expects doc.parts to be an array of alternating content and
                // whitespace. The whitespace contains the linebreaks.
                //
                // For example:
                //   ["I", line, "love", line, "monkeys"]
                // or
                //   [{ type: group, ... }, softline, { type: group, ... }]
                //
                // It uses this parts structure to handle three main layout cases:
                // * The first two content items fit on the same line without
                //   breaking
                //   -> output the first content item and the whitespace "flat".
                // * Only the first content item fits on the line without breaking
                //   -> output the first content item "flat" and the whitespace with
                //   "break".
                // * Neither content item fits on the line without breaking
                //   -> output the first content item and the whitespace with "break".
                Self::DocCommand(DocCommand::Fill { mut parts }) => {
                    #[allow(clippy::cast_possible_wrap)]
                    let rem = width - pos.clamp(0, isize::MAX as usize) as isize;

                    let Some(content) = parts.pop_front() else {
                        continue;
                    };
                    let whitespace = parts.pop_front();

                    let content_flat_cmd = Command {
                        indent: indent.clone(),
                        mode: Mode::Flat,
                        doc: content.clone(),
                    };
                    let content_break_cmd = Command {
                        indent: indent.clone(),
                        mode: Mode::Break,
                        doc: content.clone(),
                    };
                    let content_fits = fits(
                        &content_flat_cmd,
                        &[],
                        rem,
                        !line_suffix.is_empty(),
                        &group_mode_map,
                        true,
                    );

                    let Some(whitespace) = whitespace else {
                        // equivalent to the original implementation's `if (parts.length === 1)`
                        debug_assert!(parts.is_empty());
                        if content_fits {
                            cmds.push(content_flat_cmd);
                        } else {
                            cmds.push(content_break_cmd);
                        }
                        continue;
                    };

                    let whitespace_flat_cmd = Command {
                        indent: indent.clone(),
                        mode: Mode::Flat,
                        doc: whitespace.clone(),
                    };
                    let whitespace_break_cmd = Command {
                        indent: indent.clone(),
                        mode: Mode::Break,
                        doc: whitespace.clone(),
                    };

                    // At this point we've handled the first pair (context, separator)
                    // and will create a new fill doc for the rest of the content.
                    // Ideally we wouldn't mutate the array here but copying all the
                    // elements to a new array would make this algorithm quadratic,
                    // which is unusable for large arrays (e.g. large texts in JSX).
                    let Some(second_content) = parts.front() else {
                        // equivalent to the original implementation's `if (parts.length === 2)`
                        if content_fits {
                            cmds.push(whitespace_flat_cmd);
                            cmds.push(content_flat_cmd);
                        } else {
                            cmds.push(whitespace_break_cmd);
                            cmds.push(content_break_cmd);
                        }
                        continue;
                    };
                    let remaining_cmd = Command {
                        indent: indent.clone(),
                        mode,
                        doc: Self::DocCommand(DocCommand::Fill {
                            parts: parts.clone(),
                        }),
                    };

                    let first_and_second_fit = fits(
                        &Command {
                            indent: indent.clone(),
                            mode: Mode::Flat,
                            doc: Self::from(vec![
                                content.clone(),
                                whitespace.clone(),
                                second_content.clone(),
                            ]),
                        },
                        &[],
                        rem,
                        !line_suffix.is_empty(),
                        &group_mode_map,
                        true,
                    );

                    cmds.push(remaining_cmd);
                    if first_and_second_fit {
                        cmds.push(whitespace_flat_cmd);
                        cmds.push(content_flat_cmd);
                    } else if content_fits {
                        cmds.push(whitespace_break_cmd);
                        cmds.push(content_flat_cmd);
                    } else {
                        cmds.push(whitespace_break_cmd);
                        cmds.push(content_break_cmd);
                    }
                }

                Self::DocCommand(DocCommand::IfBreak {
                    break_contents,
                    flat_contents,
                    group_id,
                }) => {
                    let group_mode = group_id
                        .and_then(|id| group_mode_map.get(&id).copied())
                        .unwrap_or(mode);
                    match group_mode {
                        Mode::Break => {
                            cmds.push(Command {
                                indent,
                                mode, // TODO: it's like this in original implementation, but shouldn't this be Mode::Break?
                                doc: break_contents.as_ref().clone(),
                            });
                        }
                        Mode::Flat => {
                            cmds.push(Command {
                                indent,
                                mode,
                                doc: flat_contents.as_ref().clone(),
                            });
                        }
                    }
                }
                Self::DocCommand(DocCommand::IndentIfBreak {
                    contents,
                    group_id,
                    negate,
                }) => {
                    let group_mode = group_id
                        .and_then(|id| group_mode_map.get(&id).copied())
                        .unwrap_or(mode);
                    match group_mode {
                        Mode::Break => cmds.push(Command {
                            indent,
                            mode,
                            doc: if negate {
                                contents.as_ref().clone()
                            } else {
                                Self::DocCommand(DocCommand::Indent {
                                    contents: Box::new(contents.as_ref().clone()),
                                })
                            },
                        }),
                        Mode::Flat => cmds.push(Command {
                            indent,
                            mode,
                            doc: if negate {
                                Self::DocCommand(DocCommand::Indent {
                                    contents: Box::new(contents.as_ref().clone()),
                                })
                            } else {
                                contents.as_ref().clone()
                            },
                        }),
                    }
                }

                Self::DocCommand(DocCommand::LineSuffix { contents }) => {
                    line_suffix.push(Command {
                        indent,
                        mode,
                        doc: contents.as_ref().clone(),
                    });
                }
                Self::DocCommand(DocCommand::LineSuffixBoundary) => {
                    if !line_suffix.is_empty() {
                        cmds.push(Command {
                            indent,
                            mode,
                            doc: hardline_without_break_parent(),
                        });
                    }
                }

                Self::DocCommand(DocCommand::Line(line_type)) => {
                    match mode {
                        Mode::Flat if line_type == LineType::None => {
                            out.push(OutputItem::Content(" ".into()));
                            pos += 1;
                        }
                        Mode::Flat if line_type == LineType::Soft => {}
                        _ => {
                            if mode == Mode::Flat {
                                // This line was forced into the output even if we
                                // were in flattened mode, so we need to tell the next
                                // group that no matter what, it needs to remeasure
                                // because the previous measurement didn't accurately
                                // capture the entire expression (this is necessary
                                // for nested groups)
                                should_remeasure = true;
                            }

                            if !line_suffix.is_empty() {
                                cmds.push(Command {
                                    indent,
                                    mode,
                                    doc: hardline_without_break_parent(),
                                });
                                #[allow(clippy::iter_with_drain)]
                                // we want to consume the vector without moving it
                                cmds.extend(line_suffix.drain(..).rev());
                            } else if line_type == LineType::Literal {
                                out.push(OutputItem::Content(new_line.to_string()));
                                if let Some(root_indent) = indent.borrow().to_owned().root {
                                    out.push(OutputItem::Content(root_indent.value.clone()));
                                    pos = root_indent.length;
                                } else {
                                    pos = 0;
                                }
                            } else {
                                // pos -= pos.checked_sub(trim(&mut out)).unwrap_or_default();
                                trim(&mut out);
                                out.push(OutputItem::Content(
                                    new_line.to_string() + &indent.borrow().value,
                                ));
                                pos = indent.borrow().length;
                            }
                        }
                    }
                }

                Self::DocCommand(DocCommand::Label { contents, .. }) => {
                    cmds.push(Command {
                        indent,
                        mode,
                        doc: contents.as_ref().clone(),
                    });
                }

                Self::DocCommand(DocCommand::BreakParent) => {} // do nothing
            }

            // Flush remaining line-suffix contents at the end of the document, in case
            // there is no new line after the line-suffix.
            if cmds.is_empty() && !line_suffix.is_empty() {
                #[allow(clippy::iter_with_drain)]
                // this is indeed what we want to do, we want to consume and empty `line_suffix` w/o moving it
                cmds.extend(line_suffix.drain(..).rev());
            }
        }

        // remove cursors
        return Ok(out
            .iter()
            .filter_map(|o| match o {
                OutputItem::Content(s) => Some(s.clone()),
                OutputItem::Cursor => None,
            })
            .collect::<String>());
    }
}

impl Indent {
    fn root() -> Self {
        Self {
            value: String::new(),
            length: 0,
            queue: Vec::new(),
            root: None,
        }
    }
}

fn make_indent(indent: Rc<RefCell<Indent>>, options: &PrettyPrinter) -> Indent {
    generate_indent(indent, Part::Indent, options)
}

fn generate_indent(indent: Rc<RefCell<Indent>>, new_part: Part, options: &PrettyPrinter) -> Indent {
    let queue = {
        let mut q = indent.borrow().to_owned().queue.clone();
        if matches!(new_part, Part::Dedent) {
            q.pop();
        } else {
            q.push(new_part);
        }
        q
    };

    let mut value = String::new();
    let mut length = 0;
    let mut last_tabs = 0;
    let mut last_spaces = 0;

    for part in &queue {
        match part {
            Part::Indent => {
                if options.use_tabs {
                    if last_tabs > 0 {
                        value.push_str(&"\t".repeat(last_tabs));
                        length += last_tabs * options.tab_width;
                    }
                    value.push('\t');
                    length += 1;
                } else {
                    if last_spaces > 0 {
                        value.push_str(&" ".repeat(last_spaces));
                        length += last_spaces;
                    }
                    value.push_str(&" ".repeat(options.tab_width));
                    length += options.tab_width;
                }
                last_tabs = 0;
                last_spaces = 0;
            }
            Part::StringAlign(s) => {
                if options.use_tabs {
                    if last_tabs > 0 {
                        value.push_str(&"\t".repeat(last_tabs));
                        length += last_tabs * options.tab_width;
                    }
                } else if last_spaces > 0 {
                    value.push_str(&" ".repeat(last_spaces));
                    length += last_spaces;
                }
                last_tabs = 0;
                last_spaces = 0;
                value += &s;
                length += s.len();
            }
            Part::NumberAlign(n) => {
                last_tabs += 1;
                last_spaces += n;
            }
            Part::Dedent => {
                unreachable!("Dedent should have been handled above");
            }
        }
    }

    if last_spaces > 0 {
        value.push_str(&" ".repeat(last_spaces));
        length += last_spaces;
    }

    // original: return { ...ind, value, length, queue };
    Indent {
        value,
        length,
        queue,
        root: indent.borrow().to_owned().root,
    }
}

fn make_align(indent: Rc<RefCell<Indent>>, options: &PrettyPrinter, alignment: Align) -> Indent {
    match alignment {
        Align::ToRoot => indent
            .borrow()
            .root
            .as_ref()
            .map_or_else(Indent::root, |root| root.as_ref().clone()),
        Align::AsRoot => Indent {
            root: Some(Box::new(indent.borrow().to_owned())),
            ..indent.borrow().to_owned()
        },
        Align::By(n) if n < 0 => generate_indent(indent, Part::Dedent, options),
        #[allow(clippy::cast_sign_loss)] // the previous case ensures that `n` is positive
        Align::By(n) => generate_indent(indent, Part::NumberAlign(n as usize), options),
        Align::With(s) => generate_indent(indent, Part::StringAlign(s), options),
    }
}

fn trim(out: &mut Vec<OutputItem>) -> usize {
    let mut trim_count = 0;
    let mut cursor_count = 0;
    let mut out_index = out.len();

    'outer: while out_index > 0 {
        out_index -= 1;

        match &out[out_index] {
            OutputItem::Content(last) => {
                for char_index in (0..last.len()).rev() {
                    let c = last.chars().nth(char_index).unwrap();
                    if c == ' ' || c == '\t' {
                        trim_count += 1;
                    } else {
                        out[out_index] = OutputItem::Content(last[..=char_index].to_string());
                        // *last = last[..char_index + 1].to_string();
                        break 'outer;
                    }
                }
                out[out_index] = OutputItem::Content(String::new());
            }
            OutputItem::Cursor => {
                cursor_count += 1;
            }
        }
    }

    // from original js, removed because it doesn't make sense
    //   if (trimCount > 0 || cursorCount > 0) {
    //     out.length = outIndex + 1;
    //     while (cursorCount-- > 0) {
    //       out.push(CURSOR_PLACEHOLDER);
    //     }
    //   }
    //   return trimCount;

    if trim_count > 0 || cursor_count > 0 {
        out.truncate(out_index + 1);
        for _ in 0..cursor_count {
            out.push(OutputItem::Cursor);
        }
    }

    trim_count
}

#[allow(clippy::too_many_lines)]
fn fits(
    next: &Command,
    rest_commands: &[Command],
    mut width: isize,
    mut has_line_suffix: bool,
    group_mode_map: &HashMap<Symbol, Mode>,
    must_be_flat: bool,
) -> bool {
    struct StrippedCommand<'a> {
        mode: Mode,
        doc: &'a Doc,
    }

    impl<'a> From<&'a Command> for StrippedCommand<'a> {
        fn from(command: &'a Command) -> Self {
            StrippedCommand {
                mode: command.mode,
                doc: &command.doc,
            }
        }
    }

    if width == isize::MAX {
        return true;
    }
    if width < 0 {
        return false;
    }

    let mut cmds: Vec<StrippedCommand> = vec![next.into()];
    // `out` is only used for width counting because `trim` requires to look
    // backwards for space characters.
    let mut out: Vec<OutputItem> = Vec::new();
    let mut rest_iter = rest_commands.iter().rev();

    while width >= 0 {
        if cmds.is_empty() {
            if let Some(command) = rest_iter.next() {
                cmds.push(command.into());
                continue;
            }
            return true;
        }
        #[allow(clippy::unwrap_used)] // safe because we check for empty above
        let StrippedCommand { mode, doc } = cmds.pop().unwrap();

        match doc {
            #[allow(clippy::cast_possible_wrap)]
            Doc::String(s) => {
                out.push(OutputItem::Content(s.clone()));
                width -= s.len().clamp(0, isize::MAX as usize) as isize;
            }
            Doc::Array(parts) => {
                for part in parts.iter().rev() {
                    cmds.push(StrippedCommand { mode, doc: part });
                }
            }
            Doc::DocCommand(DocCommand::Fill { parts }) => {
                for part in parts.iter().rev() {
                    cmds.push(StrippedCommand { mode, doc: part });
                }
            }
            Doc::DocCommand(
                DocCommand::Indent { contents }
                | DocCommand::Align { contents, .. }
                | DocCommand::IndentIfBreak { contents, .. }
                | DocCommand::Label { contents, .. },
            ) => {
                cmds.push(StrippedCommand {
                    mode,
                    doc: contents.as_ref(),
                });
            }
            #[allow(clippy::cast_possible_wrap)]
            Doc::DocCommand(DocCommand::Trim) => {
                width += trim(&mut out).clamp(0, isize::MAX as usize) as isize;
            }
            Doc::DocCommand(DocCommand::Group {
                id: _,
                contents,
                break_: should_break,
                expanded_states,
            }) => {
                if must_be_flat && should_break.should_break() {
                    return false;
                }

                let group_mode = if should_break.should_break() {
                    Mode::Break
                } else {
                    mode
                };
                // The most expanded state takes up the least space on the current line.
                let contents = expanded_states
                    .as_ref()
                    .and_then(|v| v.last())
                    .unwrap_or(contents);

                cmds.push(StrippedCommand {
                    mode: group_mode,
                    doc: contents,
                });
            }

            Doc::DocCommand(DocCommand::IfBreak {
                break_contents,
                flat_contents,
                group_id,
            }) => {
                let group_mode = group_id.as_ref().map_or(mode, |id| {
                    group_mode_map.get(id).copied().unwrap_or(Mode::Break)
                });
                let contents = if group_mode == Mode::Break {
                    break_contents.as_ref()
                } else {
                    flat_contents.as_ref()
                };

                if !contents.is_empty() {
                    cmds.push(StrippedCommand {
                        mode: group_mode,
                        doc: contents,
                    });
                }
            }

            Doc::DocCommand(DocCommand::Line(line_type)) => {
                if mode == Mode::Break || line_type.is_hard() {
                    return true;
                }
                if !line_type.is_soft() {
                    out.push(OutputItem::Content(" ".into()));
                    width -= 1;
                }
            }

            Doc::DocCommand(DocCommand::LineSuffix { .. }) => {
                has_line_suffix = true;
            }

            Doc::DocCommand(DocCommand::LineSuffixBoundary) => {
                if has_line_suffix {
                    return false;
                }
            }

            Doc::DocCommand(DocCommand::BreakParent | DocCommand::Cursor) => {}
        }
    }

    false
}
