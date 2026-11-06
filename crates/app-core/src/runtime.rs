use std::path::PathBuf;

use rust_ide_shared::{BackendCommand, BackendEvent};
use thiserror::Error;
use tokio::sync::{broadcast, mpsc};

/// Backend runtime/channel errors.
#[derive(Debug, Error)]
pub enum BackendError {
    #[error("backend command queue is closed")]
    CommandQueueClosed,
}

/// Handle used by the UI/application layer to submit commands and subscribe to events.
#[derive(Clone)]
pub struct BackendRuntime {
    commands: mpsc::Sender<BackendCommand>,
    events: broadcast::Sender<BackendEvent>,
}

impl BackendRuntime {
    /// Spawn the backend command loop. Call from an active Tokio runtime.
    pub fn spawn() -> Self {
        let (commands, mut receiver) = mpsc::channel::<BackendCommand>(128);
        let (events, _) = broadcast::channel::<BackendEvent>(512);
        let event_sender = events.clone();

        tokio::spawn(async move {
            while let Some(command) = receiver.recv().await {
                let event = match command {
                    BackendCommand::OpenFile { request_id, document_id, path } => {
                        match tokio::fs::read_to_string(&path).await {
                            Ok(contents) => BackendEvent::FileOpened {
                                request_id, document_id, path, contents,
                            },
                            Err(error) => BackendEvent::OperationFailed {
                                request_id,
                                operation: "open_file",
                                message: format!("{}: {error}", path.display()),
                            },
                        }
                    }
                    BackendCommand::SaveFile { request_id, document_id, path, contents } => {
                        match tokio::fs::write(&path, contents).await {
                            Ok(()) => BackendEvent::FileSaved { request_id, document_id, path },
                            Err(error) => BackendEvent::OperationFailed {
                                request_id,
                                operation: "save_file",
                                message: format!("{}: {error}", path.display()),
                            },
                        }
                    }
                };
                // Do not panic if the UI unsubscribed during shutdown. Subscribe before dispatching work.
                let _ = event_sender.send(event);
            }
        });

        Self { commands, events }
    }

    /// Submit a backend command without blocking the UI thread.
    pub async fn dispatch(&self, command: BackendCommand) -> Result<(), BackendError> {
        self.commands.send(command).await.map_err(|_| BackendError::CommandQueueClosed)
    }

    /// Subscribe to backend events. Use a dedicated receiver per UI subscriber.
    pub fn subscribe(&self) -> broadcast::Receiver<BackendEvent> {
        self.events.subscribe()
    }

    /// Convenience helper for opening a file.
    pub async fn open_file(
        &self,
        request_id: rust_ide_shared::RequestId,
        document_id: rust_ide_shared::DocumentId,
        path: PathBuf,
    ) -> Result<(), BackendError> {
        self.dispatch(BackendCommand::OpenFile { request_id, document_id, path }).await
    }
}
