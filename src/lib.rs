//! The high level idea of "pretty printing" is to parse an input string into an Abstract Syntax Tree (AST),
//! and then walk the tree to print it out in a different format.
//!
//! we can use tree-sitter to create the AST, but we need to implement the pretty printing ourselves.
//!
//! to facilitate this, we expose a `PrettyPrinterBuilder` struct to the user, which they can use to configure a `PrettyPrinter` to their liking.
pub mod common;
mod config;
pub mod document;
mod languages;
pub mod utils;
use std::{path::PathBuf, str::FromStr};

pub use config::{PrettyPrinter, PrettyPrinterBuilder};
pub use languages::{Language, ParseToAST, PrintToDoc};

use anyhow::{anyhow, Result};

use crate::{config::Parser, document::Doc};

impl PrettyPrinter {
    /// # Errors
    /// if there is an error parsing the text to an AST, printing AST to Doc, or formating Doc to string
    pub fn format(&self, _input: impl AsRef<str>) -> Result<String> {
        // determine parser to use
        let parser = if let Some(parser) = self.parser {
            parser
        } else {
            // determine it from file extension
            Parser::infer_from_file_name(
                self.file_path
                    .clone()
                    .ok_or(anyhow!("no file path provided, and no parser specified"))
                    .and_then(|path| {
                        PathBuf::from_str(&path)
                            .map_err(|_| anyhow!("{path} is not a valid file path"))
                    })?
                    .as_ref(),
            )
            .ok_or(anyhow!("could not infer parser from file extension"))?
        };

        // match "parser"'s to "language"'s,
        // for each, we need to:
        // - parse input text into an AST
        // - walk the AST to print it out as a Doc
        // - format the resulting Doc into a string
        // the last step is the same for all languages, so we can do that outside the match
        let doc: Doc = match parser {
            Parser::Javascript => {
                let parser = languages::javascript::Javascript::new();
                let ast = parser.parse(_input.as_ref().into(), self)?;
                parser.print(ast, self)?
            }
        };

        // format the Doc into a string
        return Ok(doc.format(self)?);
    }
}
