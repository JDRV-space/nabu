# Contributing To Nabu

Nabu is a small Rust/Wasm app. Keep changes local, readable, and proportional
to the behavior being changed.

Do not report suspected vulnerabilities in public issues. Follow
[SECURITY.md](SECURITY.md) instead.

## Local Setup

The supported build path requires macOS or Linux on arm64 or x86_64, `rustup`,
Node.js 20 or newer with npm, `curl`, `tar`, and a SHA-256 command. The exact
Rust and Trunk versions are owned by `rust-toolchain.toml` and
`scripts/install-trunk.sh`.

Run the reproducible build once, then start the development server:

```bash
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
./scripts/build.sh
trunk serve
```

The development server runs at `http://127.0.0.1:8080`.

## Validation

CI owns the mandatory automated checks. Run the smallest relevant subset while
developing and the full set before requesting review:

```bash
npm test
npm audit --audit-level=high
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --locked
./scripts/build.sh
```

Use `npm run prepare:assets` after changing parser dependency versions and
commit the resulting vendored assets and parser license inventory together.

For documentation-only changes, run `git diff --check` and verify changed
links. UI and browser-behavior changes also require representative manual
validation with the browser name and version recorded in the pull request.

## Code Ownership

- Keep document parsing in `src/parser/` and `assets/document-parsers/`.
- Keep IndexedDB and encryption behavior in `src/storage/`.
- Keep UI state in `src/state/` and rendering in `src/components/`.
- Do not describe browser-local encryption as device security.
- Do not document planned behavior as implemented behavior.

## Pull Requests

Explain the user-visible behavior, list the checks you ran, and attach visual
evidence only when it helps reviewers evaluate a UI change. Link related issues
without copying changing project status into stable documentation.
