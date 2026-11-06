//! Incremental parsing boundary around Tree-sitter.

use thiserror::Error;
use tree_sitter::{Language, Parser, Tree};

/// Tree-sitter integration errors.
#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("failed to configure Tree-sitter language: {0}")]
    Language(#[from] tree_sitter::LanguageError),
}

/// Parse source text with a caller-provided Tree-sitter grammar.
///
/// Grammar crates (for example `tree-sitter-rust`) should be added by the application as needed.
pub fn parse(source: &str, language: &Language) -> Result<Option<Tree>, SyntaxError> {
    let mut parser = Parser::new();
    parser.set_language(language)?;
    Ok(parser.parse(source, None))
}
