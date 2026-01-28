<div align="center">

# Nabu

### Speed Reading, Reinvented

*Named after the Mesopotamian god of writing, scribes, literacy, and wisdom*

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Leptos](https://img.shields.io/badge/Leptos_0.8-EF3939?style=for-the-badge&logo=rust&logoColor=white)](https://leptos.dev/)
[![Vercel](https://img.shields.io/badge/Vercel-000000?style=for-the-badge&logo=vercel&logoColor=white)](https://nabu-reader.vercel.app)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

**[Live Demo](https://nabu-reader.vercel.app)** · **[Report Bug](https://github.com/JDRV-space/nabu/issues/new?template=bug_report.md)** · **[Request Feature](https://github.com/JDRV-space/nabu/issues/new?template=feature_request.md)**

---

</div>

## Overview

**Nabu** is a high-performance speed reading web application built entirely in **Rust** and compiled to **WebAssembly**. It uses **RSVP (Rapid Serial Visual Presentation)** to display words at configurable speeds, helping users read faster and retain more.

All processing happens client-side. Your documents never leave your browser.

---

## Features

| Feature | Description |
|---------|-------------|
| **RSVP Engine** | Rapid Serial Visual Presentation with adjustable WPM (100 - 1000) |
| **Word Grouping** | Display 1, 3, 5, 10, or 20 words per flash with proportional speed scaling |
| **ORP Highlighting** | Optimal Recognition Point highlighted in amber for faster word recognition |
| **Document Library** | Persistent storage with IndexedDB and AES-GCM encryption (client-side, key stored locally) |
| **Multi-Format** | Supports PDF, TXT, DOCX, and Markdown files |
| **Fullscreen Mode** | Immersive distraction-free reading experience |
| **Keyboard Driven** | Full keyboard shortcut support for power users |
| **Dark Theme** | Minimal dark UI with void black background and amber accents |
| **Zero Backend** | 100% client-side -- your documents never leave your browser |

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| **Language** | [Rust](https://www.rust-lang.org/) |
| **Framework** | [Leptos 0.8](https://leptos.dev/) (CSR) |
| **Compilation** | [WebAssembly](https://webassembly.org/) via [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/) |
| **Build Tool** | [Trunk](https://trunkrs.dev/) |
| **Storage** | IndexedDB with [AES-GCM](https://docs.rs/aes-gcm) encryption |
| **PDF Parsing** | [pdf.js](https://mozilla.github.io/pdf.js/) 4.0 |
| **DOCX Parsing** | [JSZip](https://stuk.github.io/jszip/) 3.10 |
| **Sanitization** | [ammonia](https://docs.rs/ammonia) (XSS prevention) |
| **Deployment** | [Vercel](https://vercel.com/) with auto-deploy on push |
| **Typography** | [Space Grotesk](https://fonts.google.com/specimen/Space+Grotesk) |

---

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [`wasm32-unknown-unknown`](https://rustwasm.github.io/docs/book/) target
- [Trunk](https://trunkrs.dev/)

### Installation

```bash
# Clone the repository
git clone https://github.com/JDRV-space/nabu.git
cd nabu

# Add the WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk

# Start the development server
trunk serve
```

The app will be available at `http://127.0.0.1:8080`.

### Production Build

```bash
./scripts/build.sh
```

Output will be in the `dist/` directory, ready for deployment.

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Space` | Play / Pause |
| `Arrow Left` / `Right` | Adjust WPM by 50 |
| `Arrow Up` / `Down` | Adjust WPM by 10 |
| `R` | Restart from beginning |
| `F` | Toggle fullscreen |
| `ESC` | Exit reader |

---

## Project Structure

```
nabu/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── components/
│   │   ├── mod.rs              # Component exports
│   │   ├── reader.rs           # RSVP reader engine
│   │   ├── library.rs          # Document library UI
│   │   ├── settings.rs         # Settings panel
│   │   ├── stats.rs            # Reading statistics
│   │   ├── controls.rs         # Playback controls
│   │   └── upload.rs           # File upload handler
│   ├── state/
│   │   └── mod.rs              # Application state & signals
│   ├── storage/
│   │   └── mod.rs              # IndexedDB + AES-GCM encryption
│   └── parser/
│       └── mod.rs              # Document parsing (PDF, DOCX, TXT, MD)
├── docs/
│   └── SPEC.md                 # Full project specification
├── scripts/
│   ├── build.sh                # Local build script
│   └── build-vercel.sh         # Vercel deployment build
├── index.html                  # Entry HTML (pdf.js / JSZip setup)
├── style.css                   # Stylesheet (dark theme, amber accents)
├── Cargo.toml                  # Rust dependencies
├── Trunk.toml                  # Trunk build configuration
├── vercel.json                 # Vercel deployment config
└── LICENSE                     # MIT License
```

---

## Security

- **XSS Prevention** -- All user content sanitized via the [ammonia](https://docs.rs/ammonia) crate
- **Encrypted Storage** -- Documents encrypted with AES-GCM before IndexedDB persistence
- **File Validation** -- Magic bytes verification and size limits on uploads
- **CSP Headers** -- Strict Content Security Policy enforced via Vercel config
- **No Server** -- Zero backend means zero attack surface for data exfiltration

---

## Contributing

Contributions are welcome. Please read the [Contributing Guide](CONTRIBUTING.md) before submitting a PR.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -m 'Add your feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a Pull Request

---

## Documentation

- **[Project Specification](docs/SPEC.md)** -- Design system, features, and UI mockups

---

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## Author

**Juan Diego Rodriguez** -- [@JDRV-space](https://github.com/JDRV-space)

---

<div align="center">

*Nabu -- read faster, retain more.*

</div>
