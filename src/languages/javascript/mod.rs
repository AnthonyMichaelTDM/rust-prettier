mod print;

use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{
    AnyJsRoot, JsFileSource, JsSyntaxNode, LanguageVariant, LanguageVersion, ModuleKind,
};
use biome_parser::diagnostic::ParseDiagnostic;
use thiserror::Error;

use crate::{document::Doc, PrettyPrinter, PrintToDoc};

use super::{HandleComments, ParseToAST};

pub struct Javascript {}

impl Javascript {
    #[allow(unused)]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Error, Debug)]
pub enum JsError {
    #[error("Parse Error: {0:?}")]
    ParseError(#[from] ParseError),
    #[error("Print Error: {0:?}")]
    PrintError(#[from] anyhow::Error),
}
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{0:?}")]
    ParseErrorDiagnostics(Vec<ParseDiagnostic>),
}

impl ParseToAST<AnyJsRoot, JsSyntaxNode, JsError, PrettyPrinter> for Javascript {
    fn parse(
        &self,
        text: std::borrow::Cow<str>,
        _options: &PrettyPrinter,
    ) -> Result<AnyJsRoot, JsError> {
        // parse source text with granular control
        let module = JsFileSource::default()
            .with_version(LanguageVersion::ES2022)
            .with_module_kind(ModuleKind::Module)
            .with_variant(LanguageVariant::Standard);
        let parsed = parse(&text, module, JsParserOptions::default());

        if parsed.has_errors() {
            return Err(JsError::ParseError(ParseError::ParseErrorDiagnostics(
                parsed
                    .diagnostics()
                    .iter()
                    .filter(|d| d.is_error())
                    .cloned()
                    .collect::<Vec<_>>(),
            )));
        }

        // Ok(parsed.syntax())

        Ok(parsed.tree())
    }

    fn loc_start(&self, node: JsSyntaxNode) -> usize {
        node.text_range().start().into()
    }

    fn loc_end(&self, node: JsSyntaxNode) -> usize {
        node.text_range().end().into()
    }
}

impl PrintToDoc<AnyJsRoot, JsSyntaxNode, AnyJsRoot, JsError, PrettyPrinter> for Javascript {
    fn print(&self, path: AnyJsRoot, options: &PrettyPrinter) -> Result<Doc, JsError> {
        match path {
            AnyJsRoot::JsExpressionSnipped(_) => todo!(),
            AnyJsRoot::JsModule(js_module) => Ok(print::module::print_module(js_module, options)?),
            AnyJsRoot::JsScript(_) => todo!(),
        }
    }

    fn insert_pragma(&self, _text: &str) -> String {
        todo!()
    }
}

impl HandleComments<AnyJsRoot, JsSyntaxNode, AnyJsRoot, JsError, PrettyPrinter> for Javascript {}
