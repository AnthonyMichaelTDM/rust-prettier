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
pub use config::{PrettyPrinter, PrettyPrinterBuilder};
