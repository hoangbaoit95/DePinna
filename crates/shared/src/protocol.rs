use std::path::PathBuf;

use crate::{DocumentId, RequestId};

/// Commands sent from the UI/application layer to backend services.
#[derive(Debug, Clone)]
pub enum BackendCommand {
    /// Read a file from disk.
    OpenFile {
        request_id: RequestId,
        document_id: DocumentId,
        path: PathBuf,
    },
    /// Persist the current document contents to disk.
    SaveFile {
        request_id: RequestId,
        document_id: DocumentId,
        path: PathBuf,
        contents: String,
    },
}

/// Events emitted by backend services and consumed by the UI/application layer.
#[derive(Debug, Clone)]
pub enum BackendEvent {
    /// A file was successfully loaded.
    FileOpened {
        request_id: RequestId,
        document_id: DocumentId,
        path: PathBuf,
        contents: String,
    },
    /// A file was successfully saved.
    FileSaved {
        request_id: RequestId,
        document_id: DocumentId,
        path: PathBuf,
    },
    /// A backend operation failed.
    OperationFailed {
        request_id: RequestId,
        operation: &'static str,
        message: String,
    },
}
