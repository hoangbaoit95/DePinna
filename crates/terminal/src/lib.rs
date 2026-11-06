//! PTY/session boundary. Rendering and VT parsing are intentionally separate from PTY I/O.

use std::path::PathBuf;

/// Requested terminal process configuration.
#[derive(Debug, Clone)]
pub struct TerminalConfig {
    pub working_directory: PathBuf,
    pub command: PathBuf,
    pub args: Vec<String>,
    pub columns: u16,
    pub rows: u16,
}

/// Opaque identifier for a running terminal session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalSessionId(pub u64);

/// TODO: implement a session around `portable_pty::PtyPair`.
/// Keep PTY reader/writer tasks independent from Iced rendering. Add a bounded output channel
/// and a scrollback/VT parser before presenting terminal cells to the UI.
pub struct TerminalSession;
