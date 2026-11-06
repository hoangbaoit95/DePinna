//! Version-control domain types and provider boundary.

use std::path::PathBuf;

/// Working-tree state for one path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileChangeKind {
    Untracked,
    Modified,
    Added,
    Deleted,
    Renamed,
    Conflicted,
}

/// One changed path in a repository.
#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: PathBuf,
    pub kind: FileChangeKind,
    pub staged: bool,
}

/// Snapshot of repository state for the source-control panel.
#[derive(Debug, Clone, Default)]
pub struct GitStatusSnapshot {
    pub branch: Option<String>,
    pub changes: Vec<FileChange>,
}

/// Adapter contract. Implement with `gix` (preferred Rust-native path) or `git2` as needed.
pub trait GitProvider: Send + Sync {
    fn status(&self, repository_root: &std::path::Path) -> anyhow::Result<GitStatusSnapshot>;
}
