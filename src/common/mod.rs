pub mod end_of_line;

use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Symbol {
    String(&'static str),
    Number(usize),
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
        }
    }
}

impl From<&'static str> for Symbol {
    fn from(s: &'static str) -> Self {
        Self::String(s)
    }
}

impl From<usize> for Symbol {
    fn from(n: usize) -> Self {
        Self::Number(n)
    }
}
