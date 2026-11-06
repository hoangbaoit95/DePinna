# Rust IDE Template (Iced + Rust)

A Windows-first, cross-platform Cargo workspace scaffold for a native Rust code editor inspired by the interaction model of Zed.

> **Status:** starter template, not a finished IDE. The desktop shell runs as an Iced application. The editor core, Tree-sitter boundary, LSP framing, backend command/event bridge, terminal boundary, Git provider contract, and DAP message models are scaffolds for continued implementation.

## Architecture

```text
apps/desktop
    └── crates/app-ui              Iced presentation layer
            └── future custom editor widget

crates/app-core                    Application orchestration + Tokio channels
crates/shared                      IDs and backend command/event contracts
crates/editor-core                 Ropey-backed text buffer, UI independent
crates/syntax                      Tree-sitter parser boundary
crates/language-service            LSP client framing + server manager scaffold
crates/workspace                   Workspace/project metadata
crates/terminal                    PTY session boundary (portable-pty)
crates/git                         Git provider interface
crates/debugger                    DAP request/response models
crates/platform                    OS-specific defaults and integration boundary
```

### Dependency direction

- `app-ui` may depend on application/domain interfaces, but domain crates must never depend on Iced.
- `editor-core` must remain independent of rendering, LSP, and the OS.
- Each language server and debug adapter should be a separate child process.
- Keep terminal PTY I/O and terminal rendering separate.
- Do not share one global `Mutex<AppState>` across all subsystems. Use ownership, bounded channels, request IDs, document versions, and cancellation.

## Prerequisites (Windows)

1. Install the latest stable Rust toolchain with `rustup`.
2. Install Visual Studio 2022 Build Tools with **Desktop development with C++** (MSVC linker and Windows SDK).
3. Make sure the `x86_64-pc-windows-msvc` target is installed:

```powershell
rustup target add x86_64-pc-windows-msvc
```

The included `rust-toolchain.toml` selects stable and installs `rustfmt` and `clippy` components.

## Run

From the repository root:

```powershell
cargo run -p de-pinna
```

The initial window is an Iced shell with a project sidebar, an editable Iced `TextEditor` prototype, and a toggleable terminal placeholder. The built-in widget is only a bootstrap; it is not the intended large-file/IDE editor engine.

## Validate

```powershell
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Recommended implementation sequence

### 1. Editor foundation

- Replace the preview area with a dedicated `editor-view` crate and a custom Iced widget.
- Keep `TextBuffer` in `editor-core`; add transactions, multi-cursor, selections, undo/redo, and document versioning.
- Define offset conversions explicitly: Ropey character offsets, Tree-sitter byte offsets, LSP negotiated position encoding, and pixel coordinates must never be mixed.
- Test Vietnamese Telex/VNI input, IME composition, combining marks, emoji, RTL, and CJK editing early.

### 2. Syntax

- Add grammar crates such as `tree-sitter-rust` and `tree-sitter-json`.
- Keep one parser/tree per document or parser worker.
- Apply incremental edits to the old tree; do not reparse all open documents on every keystroke.

### 3. LSP

- `ls-types` is a newer LSP 3.17 type crate; it is still marked unstable by its maintainers. Pin it and isolate it behind adapters. `lsp-types` 0.97.0 is an older, more established alternative (LSP 3.16).
- Implement process lifecycle, Content-Length JSON-RPC, request IDs, cancellation, document sync, server capabilities, restart/backoff, and workspace scoping.
- Do not use `tower-lsp` as the IDE client; it is primarily an LSP server framework.

### 4. Terminal

- Build the PTY process adapter around `portable-pty`.
- Add a VT parser/terminal model (for example, evaluate `alacritty_terminal`) before rendering terminal cells.
- Use bounded channels and coalesce terminal output before sending it to the UI. Test Windows ConPTY, PowerShell, resize, Unicode, and ANSI behavior.

### 5. Git and debugger

- Implement `GitProvider` using `gix` (Rust-native) or `git2`; keep a CLI fallback for workflows not yet supported by the chosen library.
- Implement DAP Content-Length framing, adapter lifecycle, request correlation, events, breakpoints, stack frames, scopes, and variable paging.

### 6. Performance and reliability

- Virtualize editor rows; render only visible lines plus overscan.
- Cache line layout, glyph runs, syntax spans, and measurements by document revision.
- Debounce expensive search/index tasks and cancel stale completion/hover requests.
- Use bounded queues, backpressure, and request/document version checks.
- Profile release builds on large files and large workspaces before optimizing blindly.

## Current dependency baseline

The template uses stable Iced 0.14, Ropey 1.6, Tree-sitter 0.27, `notify` 8.2, and `portable-pty` 0.9. Dependency versions are centralized in the root `Cargo.toml`; commit `Cargo.lock` after the first successful build on your development machine.

## License

MIT OR Apache-2.0. Check each third-party dependency's license and any optional integration before redistribution.
