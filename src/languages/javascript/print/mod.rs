use std::collections::HashMap;

use biome_js_syntax::JsSyntaxNode;

use crate::document::Doc;

pub(self) mod export;
pub(self) mod expression;
pub(self) mod import;
pub mod module;
pub(self) mod statement;

pub(self) type Cache = HashMap<JsSyntaxNode, Doc>;
