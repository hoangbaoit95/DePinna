//! UI-independent text buffer and editing primitives.

mod buffer;
mod position;

pub use buffer::{BufferError, TextBuffer};
pub use position::{TextPosition, TextRange};
