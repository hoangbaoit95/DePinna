# Architecture notes

## Runtime model

Start with one native desktop process. The following may be child processes:

- Language servers (rust-analyzer, typescript-language-server, pyright, etc.)
- Debug adapters (CodeLLDB, debugpy, js-debug, Delve)
- Shell processes attached to PTYs

Do not split internal crates into independent OS processes prematurely. Keep crate APIs process-agnostic so selected services can be isolated later.

## UI/backend contract

The intended data flow is:

```text
Iced Message
   -> application command
      -> backend service
         -> domain event / request response
            -> Iced Message
               -> state update + render
```

`de-pinna-shared` owns cross-layer IDs and command/event DTOs. `de-pinna-app-core` owns the dispatcher/runtime. UI code should not spawn shell processes, parse LSP packets, or access Git repositories directly.

## Threading policy

- UI update/view functions must remain short and non-blocking.
- Use Tokio tasks for filesystem, LSP, process, and indexing operations.
- Give each open document a clear owner for its text buffer and monotonically increasing revision.
- Give each LSP server/session its own process and request map.
- Use bounded channels for terminal output and high-volume events.
- Never block the UI waiting on a language server, disk, or Git operation.

## Text coordinate policy

Use explicit newtypes when implementing the full editor:

- `CharOffset`: Ropey Unicode scalar-value offset.
- `ByteOffset`: UTF-8 byte offset for Tree-sitter.
- `LspPosition`: line + character using the position encoding negotiated with the server.
- `PixelPoint`: screen coordinate for hit testing/rendering.

Do not pass raw `usize` across these boundaries without a conversion function.
