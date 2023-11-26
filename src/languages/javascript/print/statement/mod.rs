mod function;
mod loops;

use anyhow::Result;
use biome_js_syntax::AnyJsStatement;
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
    if let Some(doc) = cache.get(statement.syntax()) {
        return Ok(doc.clone());
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
        AnyJsStatement::JsReturnStatement(_) => todo!(),
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
                        Ok(" ".into())
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
                        Ok(indent(join(
                            &concat([",".into(), if_break(softline(), Some(" ".into()), None)]),
                            declarations
                                .iter()
                                .map(|(name, decl)| {
                                    Ok(group(
                                        concat([
                                            name.clone(),
                                            " = ".into(),
                                            print_expression(
                                                &decl.initializer().unwrap().expression()?,
                                                options,
                                                cache,
                                            )?,
                                        ]),
                                        None,
                                        false,
                                        None,
                                    ))
                                })
                                .collect::<Result<Vec<_>>>()?,
                        )))
                    }
                })?;

            let doc = group(
                concat([prefix, declarations, dedent(";".into())]),
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

    Ok(doc)
}
