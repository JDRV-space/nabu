# Development Notes

This is a small Rust/WASM app. Keep changes boring, local, and easy to verify.

## Local Setup

Prerequisites:

- Rust stable
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`

Run the app:

```bash
trunk serve
```

The development server runs at `http://127.0.0.1:8080`.

## Checks

Use the smallest check that covers the change:

```bash
cargo fmt
cargo build --target wasm32-unknown-unknown
trunk serve
```

For docs-only changes, `git diff --check` is usually enough.

## Code Notes

- Keep document parsing in `src/parser/`.
- Keep IndexedDB and encryption behavior in `src/storage/`.
- Keep UI state in `src/state/` and rendering in `src/components/`.
- Do not describe browser-local encryption as device security.
