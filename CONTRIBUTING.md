# Contributing to Nabu

Thank you for your interest in contributing to Nabu. This guide will help you get started.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/): `cargo install trunk`

### Local Development

```bash
git clone https://github.com/JDRV-space/nabu.git
cd nabu
trunk serve
```

The development server runs at `http://127.0.0.1:8080` with hot reload.

## How to Contribute

### Reporting Bugs

Use the [Bug Report](https://github.com/JDRV-space/nabu/issues/new?template=bug_report.md) template to file an issue.

### Suggesting Features

Use the [Feature Request](https://github.com/JDRV-space/nabu/issues/new?template=feature_request.md) template to propose a feature.

### Submitting Code

1. **Fork** the repository
2. **Create** a feature branch from `main`:
   ```bash
   git checkout -b feature/your-feature
   ```
3. **Make** your changes
4. **Verify** the build compiles:
   ```bash
   cargo build --target wasm32-unknown-unknown
   ```
5. **Test** locally with `trunk serve`
6. **Commit** with a clear message:
   ```bash
   git commit -m "Add your feature description"
   ```
7. **Push** to your fork:
   ```bash
   git push origin feature/your-feature
   ```
8. **Open** a Pull Request against `main`

## Code Guidelines

- Follow standard Rust conventions and formatting (`cargo fmt`)
- Run `cargo clippy` before submitting
- Keep commits focused and atomic
- Write clear commit messages describing *what* and *why*

## Project Structure

| Directory | Purpose |
|-----------|---------|
| `src/components/` | Leptos UI components |
| `src/state/` | Application state management |
| `src/storage/` | IndexedDB persistence layer |
| `src/parser/` | Document format parsing |
| `src/utils/` | Shared utility functions |
| `docs/` | Project specification and notes |
| `scripts/` | Build and deployment scripts |

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
