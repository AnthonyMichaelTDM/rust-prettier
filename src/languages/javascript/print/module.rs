use std::collections::HashMap;

use anyhow::Result;
use biome_js_syntax::{AnyJsModuleItem, JsModule};
use biome_rowan::{AstNode as _, AstNodeList as _};

use crate::{
    document::{builders::*, Doc},
    languages::javascript::print::{
        export::print_export, import::print_import, statement::print_statement, Cache,
    },
    PrettyPrinter,
};

pub fn print_module(js_module: JsModule, options: &PrettyPrinter) -> Result<Doc> {
    fn print_module_recursive(
        item: &AnyJsModuleItem,
        options: &PrettyPrinter,
        cache: &mut Cache,
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
    fn process(item: &AnyJsModuleItem, options: &PrettyPrinter, cache: &mut Cache) -> Result<Doc> {
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
                Ok(join(&hardline(), statements))
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

    let eof = js_module
        .eof_token()
        .map_or(String::new(), |token| token.text().into())
        .into();

    Ok(concat([imports, statements, exports, eof]))
}
