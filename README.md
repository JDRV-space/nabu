# Nabu

Browser-based RSVP speed reader. It flashes words at a chosen speed so you can
read a document without moving your eyes across a page. Document parsing and
persistence happen in the browser.

## What It Does

- Imports TXT, Markdown, PDF, and DOCX files up to 50 MiB.
- Reads with RSVP at 100-1000 WPM.
- Displays 1, 3, 5, 10, or 20 words per flash.
- Highlights the Optimal Recognition Point in amber.
- Stores encrypted document records in browser IndexedDB.
- Supports fullscreen reading and keyboard controls.

## Run Locally

Prerequisites:

- macOS or Linux on arm64 or x86_64;
- `rustup`;
- Node.js 20 or newer with npm;
- `curl`, `tar`, and either `sha256sum` or `shasum`.

The pinned Trunk installer does not currently support native Windows. Other
platforms require a manual Trunk installation and are not part of the tested
build path.

```bash
git clone https://github.com/JDRV-space/nabu.git
cd nabu

export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
./scripts/build.sh
trunk serve
```

The app runs at `http://127.0.0.1:8080`; release output goes to `dist/`.
`scripts/build.sh` installs the Rust toolchain from `rust-toolchain.toml`,
vendors the locked parser assets, installs the pinned Trunk release, and runs a
locked release build.

Automated browser E2E coverage is not yet available. CI validates compilation,
tests, dependency audits, parser limits, generated assets, and the release
build, but it does not establish a browser support matrix.

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Space | Play/Pause |
| Left/Right | Adjust WPM by 50 |
| Up/Down | Adjust WPM by 10 |
| R | Restart |
| F | Fullscreen |
| ESC | Exit reader |

## Privacy And Limitations

- Documents are encrypted with AES-GCM before storage in browser IndexedDB.
- The encryption key is stored in browser `localStorage` as `nabu_key`.
- This prevents server-side document retention. It does not protect against a
  compromised browser, same-origin script bug, extension, device, or profile.
- Clearing site data can delete the library and its encryption key.
- PDF and DOCX parsing happens in the browser and can fail on malformed,
  scanned, encrypted, or unusual files.
- Google Fonts receives ordinary page-load request metadata; document content
  is not sent to Google Fonts.
- There is no account system, sync, sharing, or backup.

Exact parser limits and security controls are maintained in the
[technical specification](docs/SPEC.md).

## Documentation Ownership

This README owns project orientation and support status; `docs/SPEC.md` owns
stable technical contracts. Code, tests, configuration, and verified
deployments own current behavior. See [AGENTS.md](AGENTS.md) for maintenance
rules.

- [Contribution guide](CONTRIBUTING.md)
- [Security policy](SECURITY.md)
- [Document parser assets](assets/document-parsers/README.md)

## License

Nabu is licensed under Apache License 2.0. See [LICENSE](LICENSE) and
[NOTICE](NOTICE). Parser dependency license texts are distributed with the
vendored assets.
