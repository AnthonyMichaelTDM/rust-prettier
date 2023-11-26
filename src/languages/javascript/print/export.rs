use anyhow::Result;
use biome_js_syntax::{AnyJsExportClause, JsExport};
use biome_rowan::AstNode as _;

use crate::{
    document::{builders::*, Doc},
    PrettyPrinter,
};

use super::{expression::print_expression, Cache};

pub fn print_export(export: &JsExport, _options: &PrettyPrinter, cache: &mut Cache) -> Result<Doc> {
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
