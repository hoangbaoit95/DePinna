//! Project/workspace metadata and file watching boundary.

use std::path::{Path, PathBuf};

use rust_ide_shared::WorkspaceId;
use thiserror::Error;

/// An opened project root.
#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub root: PathBuf,
    pub display_name: String,
}

impl Workspace {
    /// Create workspace metadata from a directory path.
    pub fn new(id: WorkspaceId, root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        let display_name = root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Workspace")
            .to_owned();
        Self { id, root, display_name }
    }

    /// Check whether a path is located under this workspace root.
    pub fn contains(&self, path: &Path) -> bool {
        path.starts_with(&self.root)
    }
}

/// Workspace-level errors.
#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("workspace root does not exist: {0}")]
    MissingRoot(PathBuf),
}
