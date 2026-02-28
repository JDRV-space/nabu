<div align="center">

# Nabu

Speed reading app built with Rust and WebAssembly. Named after the Mesopotamian god of writing and wisdom.

[![Live Demo](https://img.shields.io/badge/Live-nabu--reader.vercel.app-FF4136?style=for-the-badge&logo=vercel&logoColor=white)](https://nabu-reader.vercel.app)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Leptos](https://img.shields.io/badge/Leptos_0.8-EF3939?style=for-the-badge&logo=rust&logoColor=white)](https://leptos.dev/)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/JDRV-space/nabu/pulls)

</div>

Uses RSVP (Rapid Serial Visual Presentation) to flash words at configurable speeds. Everything runs client-side, your documents never leave your browser.

## Features

- RSVP engine with adjustable WPM (100-1000)
- Word grouping: display 1, 3, 5, 10, or 20 words per flash
- ORP (Optimal Recognition Point) highlighting in amber for faster recognition
- Document library with IndexedDB storage and AES-GCM encryption
- Supports PDF, TXT, DOCX, and Markdown
- Fullscreen mode for distraction-free reading
- Full keyboard shortcut support
- Dark theme with void black background and amber accents

## Tech Stack

- **Language:** Rust
- **Framework:** Leptos 0.8 (CSR)
- **Compilation:** WebAssembly via wasm-bindgen
- **Build:** Trunk
- **Storage:** IndexedDB with AES-GCM encryption
- **PDF parsing:** pdf.js 4.0
- **DOCX parsing:** JSZip 3.10
- **Sanitization:** ammonia (XSS prevention)
- **Deployment:** Vercel with auto-deploy
- **Typography:** Space Grotesk

## Quick Start

Prerequisites:
- Rust (stable)
- wasm32-unknown-unknown target
- Trunk

```bash
git clone https://github.com/JDRV-space/nabu.git
cd nabu

rustup target add wasm32-unknown-unknown
cargo install trunk

trunk serve
```

App runs at http://127.0.0.1:8080

For production: `./scripts/build.sh` (output in `dist/`)

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Space | Play/Pause |
| Left/Right | Adjust WPM by 50 |
| Up/Down | Adjust WPM by 10 |
| R | Restart |
| F | Fullscreen |
| ESC | Exit reader |

## Project Structure

```
nabu/
├── src/
│   ├── main.rs              # Entry point
│   ├── components/          # reader, library, settings, controls, upload
│   ├── state/               # Application state and signals
│   ├── storage/             # IndexedDB + encryption
│   └── parser/              # PDF, DOCX, TXT, MD parsing
├── docs/SPEC.md             # Full specification
├── scripts/                 # Build scripts
├── index.html
├── style.css
├── Cargo.toml
├── Trunk.toml
├── vercel.json
└── LICENSE
```

## Security

- XSS prevention via ammonia
- AES-GCM encryption for stored documents
- Magic bytes validation and size limits on uploads
- Strict CSP headers via Vercel
- No backend = no server-side attack surface

## Contributing

PRs welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

1. Fork the repo
2. Create feature branch
3. Commit changes
4. Push and open PR

## Documentation

- [Project Specification](docs/SPEC.md): design system, features, UI mockups

## License

MIT. See [LICENSE](LICENSE).

## Author

Juan Diego Rodriguez [@JDRV-space](https://github.com/JDRV-space)
