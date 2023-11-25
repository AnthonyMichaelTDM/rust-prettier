use std::collections::HashMap;

use anyhow::Result;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExportClause, AnyJsExpression, AnyJsImportClause, AnyJsModuleItem,
    AnyJsNamedImport, AnyJsStatement, JsExport, JsImport, JsModule, JsScript, JsSyntaxNode,
};
use biome_rowan::{AstNode, AstNodeList};

use crate::{
    document::{
        builders::{self, concat, dedent, group, hardline, if_break, indent, join, softline},
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
            let imports = match  named_import.named_import().unwrap() {
                    AnyJsNamedImport::JsNamedImportSpecifiers(imports) => imports
                        .specifiers()
                        .into_iter()
                        .filter_map(|specifier| specifier.ok())
                        .map(|specifier| match specifier {
                            biome_js_syntax::AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => todo!(),
                            biome_js_syntax::AnyJsNamedImportSpecifier::JsNamedImportSpecifier(_) => todo!(),
                            biome_js_syntax::AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(name) => name.local_name().unwrap().to_string().into(),
                        }).collect::<Vec<_>>(),
                    AnyJsNamedImport::JsNamespaceImportSpecifier(_) => todo!(),
                };

            let source: Doc = named_import.source().unwrap().to_string().into();

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
        biome_js_syntax::AnyJsStatement::JsBlockStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsBogusStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsBreakStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsClassDeclaration(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsContinueStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsDebuggerStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsDoWhileStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsEmptyStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsExpressionStatement(expr) => concat([
            print_expression(&expr.expression().unwrap(), _options, cache)?,
            ";".into(),
        ]),
        biome_js_syntax::AnyJsStatement::JsForInStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsForOfStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsForStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsFunctionDeclaration(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsIfStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsLabeledStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsReturnStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsSwitchStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsThrowStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsTryFinallyStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsTryStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsVariableStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsWhileStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::JsWithStatement(_) => todo!(),
        biome_js_syntax::AnyJsStatement::TsDeclareFunctionDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        biome_js_syntax::AnyJsStatement::TsDeclareStatement(_) => {
            unimplemented!("TS, not JS")
        }
        biome_js_syntax::AnyJsStatement::TsEnumDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        biome_js_syntax::AnyJsStatement::TsExternalModuleDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        biome_js_syntax::AnyJsStatement::TsGlobalDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        biome_js_syntax::AnyJsStatement::TsImportEqualsDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        biome_js_syntax::AnyJsStatement::TsInterfaceDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        biome_js_syntax::AnyJsStatement::TsModuleDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        biome_js_syntax::AnyJsStatement::TsTypeAliasDeclaration(_) => {
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
        AnyJsExpression::AnyJsLiteralExpression(literal) => literal.to_string().into(),
        AnyJsExpression::JsArrayExpression(_) => todo!(),
        AnyJsExpression::JsArrowFunctionExpression(_) => todo!(),
        AnyJsExpression::JsAssignmentExpression(_) => todo!(),
        AnyJsExpression::JsAwaitExpression(_) => todo!(),
        AnyJsExpression::JsBinaryExpression(_) => todo!(),
        AnyJsExpression::JsBogusExpression(_) => todo!(),
        AnyJsExpression::JsCallExpression(call) => group(
            concat([
                call.callee().unwrap().to_string().into(),
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
            ]),
            None,
            false,
            None,
        ),
        AnyJsExpression::JsClassExpression(_) => todo!(),
        AnyJsExpression::JsComputedMemberExpression(_) => todo!(),
        AnyJsExpression::JsConditionalExpression(_) => todo!(),
        AnyJsExpression::JsFunctionExpression(_) => todo!(),
        AnyJsExpression::JsIdentifierExpression(ident) => ident.name().unwrap().text().into(),
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

    let imports = join(
        &hardline(),
        js_module
            .items()
            .iter()
            .filter(|item| matches!(item, AnyJsModuleItem::JsImport(_)))
            .map(|ref item| print_module_recursive(item, options, &mut cache))
            .collect::<Result<Vec<_>>>()?,
    );

    let statements = join(
        &hardline(),
        js_module
            .items()
            .iter()
            .filter(|item| matches!(item, AnyJsModuleItem::AnyJsStatement(_)))
            .map(|ref item| print_module_recursive(item, options, &mut cache))
            .collect::<Result<Vec<_>>>()?,
    );

    let exports = join(
        &hardline(),
        js_module
            .items()
            .iter()
            .filter(|item| matches!(item, AnyJsModuleItem::JsExport(_)))
            .map(|ref item| print_module_recursive(item, options, &mut cache))
            .collect::<Result<Vec<_>>>()?,
    );

    Ok(builders::concat([
        imports,
        hardline(),
        hardline(),
        statements,
        hardline(),
        hardline(),
        exports,
    ]))
}

pub(crate) fn print_script(js_script: JsScript, options: &PrettyPrinter) -> Result<Doc> {
    todo!()
}
