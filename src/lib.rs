//! The high level idea of "pretty printing" is to parse an input string into an Abstract Syntax Tree (AST),
//! and then walk the tree to print it out in a different format.
//!
//! we can use tree-sitter to create the AST, but we need to implement the pretty printing ourselves.
//!
//! to facilitate this, we expose a `PrettyPrinterBuilder` struct to the user, which they can use to configure a `PrettyPrinter` to their liking.
//!
use anyhow::Result;
use derive_builder::Builder;

pub mod common;
mod config;
pub mod document;
pub mod utils;
pub use config::*;

use common::end_of_line::EndLine;

// TODO AstPath trait or something

pub trait Parser {
    type AST;
    type Error;

    fn parse(&self, input: String, options: &PrettyPrinter) -> Result<Self::AST, Self::Error> {
        todo!()
    }

    fn print(&self, ast: Self::AST, options: &PrettyPrinter) -> Result<document::Doc, Self::Error> {
        todo!()
    }

    // optional, run on the AST before printing
    fn preprocess(
        &self,
        ast: Self::AST,
        options: &PrettyPrinter,
    ) -> Option<Result<Self::AST, Self::Error>> {
        None
    }

    // optional
    fn insert_pragma(&self, text: String) -> Option<String> {
        None
    }

    // optional
    fn embed(
        &self,
        ast: Self::AST,
        options: &PrettyPrinter,
    ) -> Option<Result<document::Doc, Self::Error>> {
        None
    }
}

pub struct JavascriptParser; // TODO: move to its own file, and implement Parser for it

#[derive(Debug, Clone, Copy)]
pub enum Parsers {
    Javascript,
}

impl<T: AsRef<str>> From<T> for Parsers {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "javascript" | "js" => Parsers::Javascript,
            _ => unimplemented!("no parser for {} is implemented yet", s.as_ref()),
        }
    }
}

impl Parsers {
    pub fn get_javascript_parser() -> JavascriptParser {
        todo!()
    }
}

impl Default for Parsers {
    fn default() -> Self {
        Parsers::Javascript
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Builder)]
pub struct PrettyPrinter {
    #[builder(default, setter(into))]
    parser: Parsers,
    #[builder(default = "80")]
    print_width: usize,
    #[builder(default = "false")]
    use_tabs: bool,
    #[builder(default = "4")]
    tab_width: usize,
    #[builder(default = "EndLine::Lf", setter(into))]
    end_of_line: EndLine,
    // these are options that some of the parsers use, but not all
    // the name these are organized into is just the first one I saw it in, not exhaustive at all
    // angular
    #[builder(default, setter(strip_option))]
    bracket_same_line: Option<bool>,
    #[builder(default, setter(strip_option))]
    html_whitespace_sensitivity: Option<&'static str>, // TODO: enum
    #[builder(default, setter(strip_option))]
    trailing_comma: Option<&'static str>, // TODO: enum
    #[builder(default, setter(strip_option))]
    bracket_spacing: Option<bool>,
    // css
    #[builder(default, setter(strip_option))]
    cursor_offset: Option<usize>,
    #[builder(default, setter(strip_option))]
    range_start: Option<usize>,
    #[builder(default, setter(strip_option))]
    range_end: Option<usize>,
    // handlebars
    #[builder(default, setter(strip_option))]
    single_quote: Option<bool>,
    // flow
    #[builder(default, setter(strip_option))]
    arrow_parens: Option<&'static str>, // TODO: enum
    #[builder(default, setter(strip_option))]
    semi: Option<bool>,
    #[builder(default, setter(strip_option))]
    quote_props: Option<&'static str>, // TODO: enum
    //html
    #[builder(default, setter(strip_option))]
    insert_pragma: Option<bool>,
    #[builder(default, setter(strip_option))]
    require_pragma: Option<bool>,
    #[builder(default, setter(strip_option))]
    single_attribute_per_line: Option<bool>,
    // javascript
    #[builder(default, setter(strip_option))]
    experimental_ternaries: Option<bool>,
    #[builder(default, setter(strip_option))]
    prose_wrap: Option<&'static str>, // TODO: enum
    // jsx
    #[builder(default, setter(strip_option))]
    jsx_bracket_same_line: Option<bool>,
    // mdx
    #[builder(default, setter(strip_option))]
    embedded_language_formatting: Option<&'static str>, // TODO: enum
    // vue
    #[builder(default, setter(strip_option))]
    vue_indent_script_and_style: Option<bool>,
}

impl PrettyPrinter {
    pub fn format(&self, _input: impl AsRef<str>) -> Result<String> {
        todo!()
    }
}
