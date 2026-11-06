use std::path::PathBuf;

/// Configuration for one external language server process.
#[derive(Debug, Clone)]
pub struct LanguageServerConfig {
    pub id: String,
    pub executable: PathBuf,
    pub args: Vec<String>,
    pub file_extensions: Vec<String>,
}

/// Owns language server processes for a workspace.
///
/// TODO: implement process lifecycle, initialize/shutdown, JSON-RPC request IDs,
/// document versioning, cancellation, restart policy, and capability negotiation.
#[derive(Debug, Default)]
pub struct LanguageServerManager {
    configured: Vec<LanguageServerConfig>,
}

impl LanguageServerManager {
    /// Register a language server configuration.
    pub fn register(&mut self, config: LanguageServerConfig) {
        self.configured.push(config);
    }

    /// Return configured language servers.
    pub fn configured(&self) -> &[LanguageServerConfig] {
        &self.configured
    }
}
