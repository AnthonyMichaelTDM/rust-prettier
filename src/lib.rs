//! The high level idea of "pretty printing" is to parse an input string into an Abstract Syntax Tree (AST),
//! and then walk the tree to print it out in a different format.
//!
//! we can use tree-sitter to create the AST, but we need to implement the pretty printing ourselves.
//!
//! to facilitate this, we expose a `PrettyPrinterBuilder` struct to the user, which they can use to configure a `PrettyPrinter` to their liking.
//!
use anyhow::Result;
use derive_builder::Builder;

pub mod document;

pub trait Parser {
    fn parse(&self, input: impl AsRef<str>) -> String;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Builder)]
pub struct PrettyPrinter {
    #[builder(default = "vec![\"graphql\"]")]
    parsers: Vec<&'static str>, // TODO make this an enum
    #[builder(default = "80")]
    print_width: usize,
    #[builder(default = "4")]
    tab_width: usize,
}

impl PrettyPrinter {
    pub fn format(&self, _input: impl AsRef<str>) -> Result<String> {
        todo!()
    }
}
