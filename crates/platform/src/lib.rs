//! Operating-system integration boundary.

use std::path::PathBuf;

/// Shell profile selected for a new integrated terminal.
#[derive(Debug, Clone)]
pub struct ShellProfile {
    pub executable: PathBuf,
    pub args: Vec<String>,
}

/// Return the conventional interactive shell for the current platform.
pub fn default_shell() -> ShellProfile {
    #[cfg(target_os = "windows")]
    {
        let system_root = std::env::var_os("SystemRoot")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(r"C:\Windows"));
        ShellProfile {
            executable: system_root.join(r"System32\WindowsPowerShell\v1.0\powershell.exe"),
            args: vec!["-NoLogo".into()],
        }
    }

    #[cfg(target_os = "macos")]
    {
        ShellProfile { executable: PathBuf::from("/bin/zsh"), args: vec!["-l".into()] }
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let shell = std::env::var_os("SHELL").map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/bin/sh"));
        ShellProfile { executable: shell, args: Vec::new() }
    }

    #[cfg(not(any(unix, target_os = "windows")))]
    {
        ShellProfile { executable: PathBuf::from("sh"), args: Vec::new() }
    }
}
