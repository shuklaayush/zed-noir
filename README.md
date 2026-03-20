# zed-noir

Noir support for Zed.

This extension now has a working local development loop with:

- tree-sitter-based syntax highlighting
- outline support for core Noir declarations
- `nargo lsp` startup through the extension runtime

## Workspace Layout

- `languages/noir/config.toml`
- `languages/noir/highlights.scm`
- `languages/noir/outline.scm`
- `extension.toml`
- `src/lib.rs`

The extension currently points at the local `tree-sitter-noir` repository during
development.

## Local Development Loop

Extension validation:

```bash
cd zed-noir
cargo check
```

Grammar validation:

```bash
cd ../tree-sitter-noir
tree-sitter generate
tree-sitter test
tree-sitter parse examples/simple.nr
tree-sitter parse examples/edge_cases.nr
```

Runtime validation:

```bash
zeditor --foreground /path/to/file.nr
```

When iterating on the dev extension in Zed, use the Noir extension's
`Rebuild` action after changes. In this workspace, that has been the most
reliable way to force Zed to rebuild and reload both the Rust extension runtime
and the grammar integration.

## Rebuild Workflow

There are two separate WASM artifacts:

1. The extension runtime:

```bash
cd zed-noir
cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/zed_noir.wasm extension.wasm
```

2. The tree-sitter grammar:

```bash
cd ../tree-sitter-noir
tree-sitter build --wasm
cp tree-sitter-noir.wasm ../zed-noir/grammars/noir.wasm
```

Important:

- `extension.wasm` must come from `wasm32-wasip2`
- `wasm32-wasip1` produces the wrong format for the current Zed extension runtime
- rebuilding only one of the two WASM files can leave the extension in a
  confusing half-working state
- after local changes, the Zed dev extension `Rebuild` action is usually still
  required before editor behavior updates in a running session
- if the grammar source URL changes during development, a stale generated
  checkout in `zed-noir/grammars/noir` can block rebuilds until it is removed
  and recreated

## Troubleshooting

### No syntax highlighting

Likely area:

- grammar build or grammar reload

Check:

- confirm `grammars/noir.wasm` exists
- use the Noir dev extension `Rebuild` action

If Zed reports that `grammars/noir` is not a git clone of the configured
repository:

- remove the generated `zed-noir/grammars/noir` checkout
- rebuild the dev extension so Zed recreates it from the current grammar source

### Syntax highlighting works, but hover and diagnostics do not

Likely area:

- extension runtime or LSP startup

Check:

- confirm `extension.wasm` was built from `wasm32-wasip2`
- run `zeditor --foreground path/to/file.nr`
- look for a foreground log line like:

```text
[lsp] starting language server process ... args: ["lsp"]
```

If that line never appears, the Noir buffer probably did not attach a running
language server.

### Highlighting or hover behavior looks stale after local changes

Likely area:

- dev extension reload state

Check:

- use the Noir dev extension `Rebuild` action, even if you already restarted
  Zed
- then re-open a Noir file and re-check behavior

In this workspace, `Rebuild` has been more reliable than restart alone.

### You are not sure whether the problem is grammar, runtime, or LSP

Use this split:

- no syntax highlighting: suspect grammar build/reload
- syntax highlighting but no hover/diagnostics: suspect extension runtime or
  `nargo lsp`
- plain-text hover with minimal formatting: LSP is running, but hover payload
  quality is limited by `nargo lsp`

## Current Limitations

- Hover quality is currently limited by the `nargo lsp` payload. The extension
  can start the language server, but hover content is still mostly plain text.
- Foreground logs are more reliable than `~/.local/share/zed/logs/Zed.log` in
  this workspace. Disk-backed log updates have been inconsistent.
- Highlighting is coherent for common Noir code, but not yet polished to the
  level of flagship language extensions.
- Reload behavior is still somewhat manual: a plain editor restart is not always
  enough, and using the dev extension `Rebuild` action remains part of the
  normal workflow.

## Real Project Fixture

The primary local Noir project used for validation is:

- `../firma/src/main.nr`

It is useful for checking:

- trailing-comma parameter parsing
- imported function calls
- `assert` highlighting
- LSP attachment inside a real Noir project
