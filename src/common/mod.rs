pub mod ast_path;
pub mod end_of_line;

use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Symbol {
    String(String),
    Number(usize),
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::String(s) => write!(f, "{}", s),
            Symbol::Number(n) => write!(f, "{}", n),
        }
    }
}
