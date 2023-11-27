mod function;
mod loops;

use anyhow::Result;
use biome_js_syntax::{AnyJsStatement, JsSyntaxKind};
use biome_rowan::AstNode as _;

use crate::{
    document::{builders::*, Doc},
    PrettyPrinter,
};

use self::{function::print_function_declaration, loops::print_for_of_statement};

use super::{expression::print_expression, Cache};

pub fn print_statement(
    statement: &AnyJsStatement,
    options: &PrettyPrinter,
    cache: &mut Cache,
) -> Result<Doc> {
    let needs_leading_newline = statement_needs_leading_newline(statement);

    if let Some(doc) = cache.get(statement.syntax()) {
        let doc = if needs_leading_newline {
            concat([hardline(), doc.clone()])
        } else {
            doc.clone()
        };
        return Ok(doc);
    }
    let doc = match statement {
        AnyJsStatement::JsBlockStatement(block) => block
            .statements()
            .into_iter()
            .map(|stmt| print_statement(&stmt, options, cache))
            .collect::<Result<Vec<_>>>()
            .and_then(|statements| {
                if statements.is_empty() {
                    Ok(concat(["{".into(), softline(), "}".into()]))
                } else {
                    Ok(concat([
                        "{".into(),
                        indent(hardline()),
                        join(&hardline(), statements),
                        dedent(hardline()),
                        "}".into(),
                    ]))
                }
            })?,
        AnyJsStatement::JsBogusStatement(_) => todo!(),
        AnyJsStatement::JsBreakStatement(_) => todo!(),
        AnyJsStatement::JsClassDeclaration(_) => todo!(),
        AnyJsStatement::JsContinueStatement(_) => todo!(),
        AnyJsStatement::JsDebuggerStatement(_) => todo!(),
        AnyJsStatement::JsDoWhileStatement(_) => todo!(),
        AnyJsStatement::JsEmptyStatement(_) => todo!(),
        AnyJsStatement::JsExpressionStatement(expr) => concat([
            print_expression(&expr.expression().unwrap(), options, cache)?,
            ";".into(),
        ]),
        AnyJsStatement::JsForInStatement(_) => todo!(),
        AnyJsStatement::JsForOfStatement(statement) => {
            print_for_of_statement(statement, options, cache)?
        }
        AnyJsStatement::JsForStatement(_) => todo!(),
        AnyJsStatement::JsFunctionDeclaration(function) => {
            print_function_declaration(function, options, cache)?
        }
        AnyJsStatement::JsIfStatement(_) => todo!(),
        AnyJsStatement::JsLabeledStatement(_) => todo!(),
        AnyJsStatement::JsReturnStatement(stmt) => concat([
            "return ".into(),
            stmt.argument()
                .and_then(|expr| print_expression(&expr, options, cache).ok())
                .unwrap_or_else(|| "".into()),
            ";".into(),
        ]),
        AnyJsStatement::JsSwitchStatement(_) => todo!(),
        AnyJsStatement::JsThrowStatement(_) => todo!(),
        AnyJsStatement::JsTryFinallyStatement(_) => todo!(),
        AnyJsStatement::JsTryStatement(_) => todo!(),
        AnyJsStatement::JsVariableStatement(stmt) => {
            let stmt = stmt.declaration()?;
            let prefix: Doc = format! {"{async_}{kind} ",async_ = if stmt.await_token().is_some() { "await " } else { "" }, kind = stmt.kind()?.text_trimmed()}.into();
            let declarations = stmt
                .declarators()
                .into_iter()
                .filter_map(|decl| decl.ok())
                .map(|decl| {
                    let name: Doc = decl.id()?.trim_trivia().unwrap().text().into();

                    Ok((name, decl))
                })
                .collect::<Result<Vec<_>>>()
                .and_then(|declarations| {
                    if declarations.is_empty() {
                        Ok("".into())
                    } else if declarations.len() == 1 {
                        Ok(fill([
                            concat([declarations.first().unwrap().0.clone(), " =".into()]),
                            if_break(indent(softline()), Some(" ".into()), None),
                            print_expression(
                                &declarations
                                    .first()
                                    .unwrap()
                                    .1
                                    .initializer()
                                    .unwrap()
                                    .expression()?,
                                options,
                                cache,
                            )?,
                        ]))
                    } else {
                        Ok(group(
                            concat(
                                declarations
                                    .iter()
                                    .map(|(name, decl)| {
                                        Ok(concat([
                                            name.clone(),
                                            " = ".into(),
                                            print_expression(
                                                &decl.initializer().unwrap().expression()?,
                                                options,
                                                cache,
                                            )?,
                                        ]))
                                    })
                                    .try_fold(
                                        Vec::with_capacity(declarations.len() * 2),
                                        |mut acc, doc: Result<Doc>| match doc {
                                            Ok(doc) => {
                                                if !acc.is_empty() {
                                                    acc.push(concat([
                                                        ",".into(),
                                                        if_break(
                                                            (acc.len() == 1)
                                                                .then(|| indent(softline()))
                                                                .unwrap_or_else(|| softline()),
                                                            Some(" ".into()),
                                                            None,
                                                        ),
                                                    ]));
                                                }
                                                acc.push(doc.clone());
                                                Ok(acc)
                                            }
                                            Err(err) => Err(err),
                                        },
                                    )?,
                            ),
                            None,
                            false,
                            None,
                        ))
                    }
                })?;

            let doc = group(
                concat([
                    prefix,
                    declarations,
                    if_break(dedent(";".into()), Some(";".into()), None),
                ]),
                None,
                false,
                None,
            );

            doc
        }
        AnyJsStatement::JsWhileStatement(_) => todo!(),
        AnyJsStatement::JsWithStatement(_) => todo!(),
        AnyJsStatement::TsDeclareFunctionDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsDeclareStatement(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsEnumDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsExternalModuleDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsGlobalDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsImportEqualsDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsInterfaceDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsModuleDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsTypeAliasDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
    };

    cache.insert(statement.syntax().clone(), doc.clone());

    if needs_leading_newline {
        Ok(concat([hardline(), doc.clone()]))
    } else {
        Ok(doc)
    }
}

fn statement_needs_leading_newline(statement: &AnyJsStatement) -> bool {
    // if statement's leading trivia has 2 or more NewLine's, and it is not the first element of a statement list
    // return true, otherwise false.
    let num_newlines = || {
        statement
            .syntax()
            .first_token()
            .and_then(|token| {
                Some(
                    token
                        .leading_trivia()
                        .pieces()
                        .filter(|trivia| trivia.is_newline())
                        .count(),
                )
            })
            .unwrap_or_default()
    };

    let is_first_element_of_parent_list = statement
        .syntax()
        .parent()
        .and_then(|parent| {
            Some(
                parent.kind() == JsSyntaxKind::JS_STATEMENT_LIST
                    && parent.first_child()? == *statement.syntax(),
            )
        })
        .unwrap_or(false);

    return !is_first_element_of_parent_list && num_newlines() >= 2;
}

/// test that it preserves leading newlines when necessary, and removes them when not (i.e. the start of a block)
#[cfg(test)]
mod tests {
    use crate::PrettyPrinter;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_statement_needs_leading_newline() {
        let input = r#"
function foo() {
  const a = 50;
  return a;

  
  foo();
}



function bar() {

    const a = 50;

    return a;
    foo();
}"#;
        let printer = PrettyPrinter::default();

        assert_str_eq!(
            printer.format(input).unwrap(),
            r#"function foo() {
  const a = 50;
  return a;

  foo();
}

function bar() {
  const a = 50;

  return a;
  foo();
}"#
        );
    }
}
