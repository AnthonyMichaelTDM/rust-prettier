use std::collections::HashMap;

use anyhow::Result;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExportClause, AnyJsExpression, AnyJsFormalParameter, AnyJsImportClause,
    AnyJsModuleItem, AnyJsNamedImport, AnyJsNamedImportSpecifier, AnyJsParameter, AnyJsStatement,
    JsExport, JsImport, JsModule, JsScript, JsSyntaxNode,
};
use biome_rowan::{AstNode, AstNodeList};

use crate::{
    document::{
        builders::{concat, dedent, group, hardline, if_break, indent, join, softline},
        Doc,
    },
    PrettyPrinter,
};

fn print_export(
    export: &JsExport,
    _options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
) -> Result<Doc> {
    if let Some(doc) = cache.get(export.syntax()) {
        return Ok(doc.clone());
    }
    let doc: Doc = match export.export_clause()? {
        AnyJsExportClause::AnyJsDeclarationClause(_) => todo!(),
        AnyJsExportClause::JsExportDefaultDeclarationClause(_) => todo!(),
        AnyJsExportClause::JsExportDefaultExpressionClause(expr) => concat([
            "export default ".into(),
            print_expression(&expr.expression()?, _options, cache)?,
            ";".into(),
        ]),
        AnyJsExportClause::JsExportFromClause(_) => todo!(),
        AnyJsExportClause::JsExportNamedClause(_) => todo!(),
        AnyJsExportClause::JsExportNamedFromClause(_) => todo!(),
        AnyJsExportClause::TsExportAsNamespaceClause(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsExportClause::TsExportAssignmentClause(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsExportClause::TsExportDeclareClause(_) => {
            unimplemented!("TS, not JS")
        }
    };

    cache.insert(export.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_import(
    import: &JsImport,
    _options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
) -> Result<Doc> {
    if let Some(doc) = cache.get(import.syntax()) {
        return Ok(doc.clone());
    }
    let doc = match import.import_clause().unwrap() {
        AnyJsImportClause::JsImportBareClause(_) => todo!(),
        AnyJsImportClause::JsImportDefaultClause(_) => todo!(),
        AnyJsImportClause::JsImportNamedClause(named_import) => {
            let imports = match named_import.named_import().unwrap() {
                AnyJsNamedImport::JsNamedImportSpecifiers(imports) => imports
                    .specifiers()
                    .into_iter()
                    .filter_map(|specifier| specifier.ok())
                    .map(|specifier| match specifier {
                        AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => todo!(),
                        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(_) => todo!(),
                        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(name) => name
                            .local_name()
                            .unwrap()
                            .trim_trivia()
                            .unwrap()
                            .text()
                            .into(),
                    })
                    .collect::<Vec<_>>(),
                AnyJsNamedImport::JsNamespaceImportSpecifier(_) => todo!(),
            };

            let source: Doc = named_import.source()?.trim_trivia().unwrap().text().into();

            let doc = group(
                concat([
                    "import".into(),
                    " {".into(),
                    softline(),
                    indent(join(
                        &concat([",".into(), if_break(softline(), Some(" ".into()), None)]),
                        imports,
                    )),
                    dedent(softline()),
                    "}".into(),
                    " from ".into(),
                    source,
                    ";".into(),
                ]),
                None,
                false,
                None,
            );

            doc
        }
        AnyJsImportClause::JsImportNamespaceClause(_) => todo!(),
    };

    cache.insert(import.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_statement(
    statement: &AnyJsStatement,
    _options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
) -> Result<Doc> {
    if let Some(doc) = cache.get(statement.syntax()) {
        return Ok(doc.clone());
    }
    let doc = match statement {
        AnyJsStatement::JsBlockStatement(_) => todo!(),
        AnyJsStatement::JsBogusStatement(_) => todo!(),
        AnyJsStatement::JsBreakStatement(_) => todo!(),
        AnyJsStatement::JsClassDeclaration(_) => todo!(),
        AnyJsStatement::JsContinueStatement(_) => todo!(),
        AnyJsStatement::JsDebuggerStatement(_) => todo!(),
        AnyJsStatement::JsDoWhileStatement(_) => todo!(),
        AnyJsStatement::JsEmptyStatement(_) => todo!(),
        AnyJsStatement::JsExpressionStatement(expr) => concat([
            print_expression(&expr.expression().unwrap(), _options, cache)?,
            ";".into(),
        ]),
        AnyJsStatement::JsForInStatement(_) => todo!(),
        AnyJsStatement::JsForOfStatement(_) => todo!(),
        AnyJsStatement::JsForStatement(_) => todo!(),
        AnyJsStatement::JsFunctionDeclaration(function) => {
            // get params
            let params = function
                .parameters()?
                .items()
                .into_iter()
                .filter_map(|item| match item.ok()? {
                    AnyJsParameter::AnyJsFormalParameter(
                        AnyJsFormalParameter::JsBogusParameter(_),
                    ) => todo!(),
                    AnyJsParameter::AnyJsFormalParameter(
                        AnyJsFormalParameter::JsFormalParameter(param),
                    ) => {
                        let param_name = param.binding().ok()?.trim_trivia()?.text().into();

                        if let Some(initializer) =
                            param.initializer().and_then(|init| init.expression().ok())
                        {
                            Some(concat([
                                param_name,
                                " = ".into(),
                                print_expression(&initializer, _options, cache).ok()?,
                            ]))
                        } else {
                            Some(param_name)
                        }
                    }
                    AnyJsParameter::JsRestParameter(_) => todo!(),
                    AnyJsParameter::TsThisParameter(_) => {
                        unimplemented!("TS, not JS")
                    }
                })
                .collect::<Vec<_>>();

            let body = group(
                concat([
                    "{".into(),
                    function
                        .body()?
                        .statements()
                        .into_iter()
                        .map(|statement| print_statement(&statement, _options, cache))
                        .collect::<Result<Vec<_>>>()
                        .and_then(|statements| {
                            if statements.is_empty() {
                                Ok(" ".into())
                            } else {
                                Ok(concat([
                                    indent(hardline()),
                                    join(&hardline(), statements),
                                    dedent(hardline()),
                                ]))
                            }
                        })?,
                    "}".into(),
                ]),
                None,
                false,
                None,
            );
            let doc = concat([
                format!(
                    "{}function {name}",
                    function.async_token().map_or("", |_| "async "),
                    name = function.id()?.trim_trivia().unwrap().text()
                )
                .into(),
                if !params.is_empty() {
                    group(
                        concat([
                            "(".into(),
                            indent(softline()),
                            join(
                                &concat([",".into(), if_break(softline(), Some(" ".into()), None)]),
                                params,
                            ),
                            dedent(softline()),
                            ")".into(),
                        ]),
                        None,
                        false,
                        None,
                    )
                } else {
                    "()".into()
                },
                " ".into(),
                body,
            ]);

            doc
        }
        AnyJsStatement::JsIfStatement(_) => todo!(),
        AnyJsStatement::JsLabeledStatement(_) => todo!(),
        AnyJsStatement::JsReturnStatement(_) => todo!(),
        AnyJsStatement::JsSwitchStatement(_) => todo!(),
        AnyJsStatement::JsThrowStatement(_) => todo!(),
        AnyJsStatement::JsTryFinallyStatement(_) => todo!(),
        AnyJsStatement::JsTryStatement(_) => todo!(),
        AnyJsStatement::JsVariableStatement(_) => todo!(),
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

fn print_expression(
    expression: &AnyJsExpression,
    options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
) -> Result<Doc> {
    if let Some(doc) = cache.get(expression.syntax()) {
        return Ok(doc.clone());
    }
    let doc: Doc = match expression {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            literal.to_owned().trim_trivia().unwrap().text().into()
        }
        AnyJsExpression::JsArrayExpression(_) => todo!(),
        AnyJsExpression::JsArrowFunctionExpression(_) => todo!(),
        AnyJsExpression::JsAssignmentExpression(_) => todo!(),
        AnyJsExpression::JsAwaitExpression(_) => todo!(),
        AnyJsExpression::JsBinaryExpression(_) => todo!(),
        AnyJsExpression::JsBogusExpression(_) => todo!(),
        AnyJsExpression::JsCallExpression(call) => group(
            concat([
                call.callee().unwrap().trim_trivia().unwrap().text().into(),
                "(".into(),
                indent(softline()),
                join(
                    &concat([",".into(), if_break(softline(), Some(" ".into()), None)]),
                    call.arguments()
                        .unwrap()
                        .args()
                        .into_iter()
                        .filter_map(|arg| arg.ok())
                        .filter_map(|arg| match arg {
                            AnyJsCallArgument::AnyJsExpression(expr) => {
                                print_expression(&expr, options, cache).ok()
                            }
                            AnyJsCallArgument::JsSpread(spread) => Some(concat([
                                "...".into(),
                                print_expression(&spread.argument().unwrap(), options, cache)
                                    .ok()?,
                            ])),
                        })
                        .collect::<Vec<_>>(),
                ),
                dedent(softline()),
                ")".into(),
            ]),
            None,
            false,
            None,
        ),
        AnyJsExpression::JsClassExpression(_) => todo!(),
        AnyJsExpression::JsComputedMemberExpression(_) => todo!(),
        AnyJsExpression::JsConditionalExpression(_) => todo!(),
        AnyJsExpression::JsFunctionExpression(_) => todo!(),
        AnyJsExpression::JsIdentifierExpression(ident) => {
            ident.name()?.trim_trivia().unwrap().text().into()
        }
        AnyJsExpression::JsImportCallExpression(_) => todo!(),
        AnyJsExpression::JsImportMetaExpression(_) => todo!(),
        AnyJsExpression::JsInExpression(_) => todo!(),
        AnyJsExpression::JsInstanceofExpression(_) => todo!(),
        AnyJsExpression::JsLogicalExpression(_) => todo!(),
        AnyJsExpression::JsNewExpression(_) => todo!(),
        AnyJsExpression::JsNewTargetExpression(_) => todo!(),
        AnyJsExpression::JsObjectExpression(_) => todo!(),
        AnyJsExpression::JsParenthesizedExpression(_) => todo!(),
        AnyJsExpression::JsPostUpdateExpression(_) => todo!(),
        AnyJsExpression::JsPreUpdateExpression(_) => todo!(),
        AnyJsExpression::JsSequenceExpression(_) => todo!(),
        AnyJsExpression::JsStaticMemberExpression(_) => todo!(),
        AnyJsExpression::JsSuperExpression(_) => todo!(),
        AnyJsExpression::JsTemplateExpression(_) => todo!(),
        AnyJsExpression::JsThisExpression(_) => todo!(),
        AnyJsExpression::JsUnaryExpression(_) => todo!(),
        AnyJsExpression::JsYieldExpression(_) => todo!(),
        AnyJsExpression::JsxTagExpression(_) => unimplemented!("not JS"),
        AnyJsExpression::TsAsExpression(_) => unimplemented!("not JS"),
        AnyJsExpression::TsInstantiationExpression(_) => unimplemented!("not JS"),
        AnyJsExpression::TsNonNullAssertionExpression(_) => unimplemented!("not JS"),
        AnyJsExpression::TsSatisfiesExpression(_) => unimplemented!("not JS"),
        AnyJsExpression::TsTypeAssertionExpression(_) => unimplemented!("not JS"),
    };

    cache.insert(expression.syntax().clone(), doc.clone());

    Ok(doc)
}

pub(crate) fn print_module(js_module: JsModule, options: &PrettyPrinter) -> Result<Doc> {
    fn print_module_recursive(
        item: &AnyJsModuleItem,
        options: &PrettyPrinter,
        cache: &mut HashMap<JsSyntaxNode, Doc>,
    ) -> Result<Doc> {
        // check if we have already printed this node
        if let Some(doc) = cache.get(item.syntax()) {
            return Ok(doc.clone());
        }

        // if not, process it
        let result = process(item, options, cache);
        if let Ok(result) = &result {
            // and cache the result
            cache.insert(item.syntax().clone(), result.clone());
        }
        result
    }
    fn process(
        item: &AnyJsModuleItem,
        options: &PrettyPrinter,
        cache: &mut HashMap<JsSyntaxNode, Doc>,
    ) -> Result<Doc> {
        match item {
            AnyJsModuleItem::JsExport(export) => print_export(export, options, cache),
            AnyJsModuleItem::JsImport(import) => print_import(import, options, cache),
            AnyJsModuleItem::AnyJsStatement(statement) => {
                print_statement(statement, options, cache)
            }
        }
    }

    // Within an abstract syntax tree, the same subtrees can be found multiple times.
    // As an optimization (those subtrees can be huge) and to maintain the
    // reference structure of the tree, the results are cached in
    // a map and reused if they are encountered again.
    let mut cache = HashMap::new();

    let imports = js_module
        .items()
        .iter()
        .filter(|item| matches!(item, AnyJsModuleItem::JsImport(_)))
        .map(|ref item| print_module_recursive(item, options, &mut cache))
        .collect::<Result<Vec<_>>>()
        .and_then(|imports| {
            if !imports.is_empty() {
                Ok(concat([join(&hardline(), imports), hardline(), hardline()]))
            } else {
                Ok("".into())
            }
        })?;

    let statements = js_module
        .items()
        .iter()
        .filter(|item| matches!(item, AnyJsModuleItem::AnyJsStatement(_)))
        .map(|ref item| print_module_recursive(item, options, &mut cache))
        .collect::<Result<Vec<_>>>()
        .and_then(|statements| {
            if !statements.is_empty() {
                Ok(concat([join(&hardline(), statements), hardline()]))
            } else {
                Ok("".into())
            }
        })?;

    let exports = js_module
        .items()
        .iter()
        .filter(|item| matches!(item, AnyJsModuleItem::JsExport(_)))
        .map(|ref item| print_module_recursive(item, options, &mut cache))
        .collect::<Result<Vec<_>>>()
        .and_then(|exports| {
            if !exports.is_empty() {
                Ok(join(&hardline(), exports))
            } else {
                Ok("".into())
            }
        })?;

    Ok(concat([imports, statements, exports]))
}

pub(crate) fn print_script(_js_script: JsScript, _options: &PrettyPrinter) -> Result<Doc> {
    todo!()
}
