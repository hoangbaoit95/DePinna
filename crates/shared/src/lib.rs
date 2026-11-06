//! Shared identifiers and small data types used across IDE crates.

pub mod ids;
pub mod protocol;

pub use ids::{DocumentId, RequestId, WorkspaceId};
pub use protocol::{BackendCommand, BackendEvent};
