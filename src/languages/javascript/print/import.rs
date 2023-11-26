use anyhow::Result;
use biome_js_syntax::{AnyJsImportClause, AnyJsNamedImport, AnyJsNamedImportSpecifier, JsImport};
use biome_rowan::AstNode as _;

use crate::{
    document::{builders::*, Doc},
    PrettyPrinter,
};

use super::Cache;

pub fn print_import(import: &JsImport, _options: &PrettyPrinter, cache: &mut Cache) -> Result<Doc> {
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
