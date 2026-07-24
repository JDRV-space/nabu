# Nabu

Browser-based RSVP speed reader. It flashes words at a chosen speed so you can read a document without moving your eyes across a page.

Live app: https://nabu-reader.vercel.app

## What It Does

- Imports TXT, Markdown, PDF, and DOCX files.
- Reads with RSVP at 100-1000 WPM.
- Displays 1, 3, 5, 10, or 20 words per flash.
- Highlights the Optimal Recognition Point in amber.
- Stores documents locally in browser IndexedDB.
- Supports fullscreen reading and keyboard controls.

## Run Locally

Prerequisites:

- rustup
- Node.js and npm

```bash
git clone https://github.com/JDRV-space/nabu.git
cd nabu

export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
./scripts/build.sh
trunk serve
```

The app runs at `http://127.0.0.1:8080`.

Build output goes to `dist/`.

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Space | Play/Pause |
| Left/Right | Adjust WPM by 50 |
| Up/Down | Adjust WPM by 10 |
| R | Restart |
| F | Fullscreen |
| ESC | Exit reader |

## Limitations

- Documents are stored in browser IndexedDB and encrypted with AES-GCM.
- The encryption key is stored in browser `localStorage` as `nabu_key`.
- This is privacy against server upload. It is not protection against a compromised browser, same-origin script bug, browser extension, device, or user profile.
- PDF.js and JSZip are copied from pinned npm packages and served from the same origin. They are not loaded from cdnjs at runtime.
- Clearing site data can delete the library and the stored key.
- PDF and DOCX parsing happens in the browser and can fail on malformed or unusual files.
- PDF imports disable PDF.js evaluation and reject more than 2,000 pages or
  more than 10 million extracted text characters.
- DOCX imports reject ZIP64, encrypted or multi-volume archives, unsafe or
  duplicate paths, more than 2,048 entries, more than 100 MiB total expanded
  content, entries over 100:1 compression, and `word/document.xml` over 10 MiB.
- There is no account system, sync, sharing, or backup.

## Project Layout

```text
nabu/
├── assets/                 # static assets, CSS, and vendored parser files
├── src/
│   ├── main.rs             # entry point
│   ├── components/         # reader, library, settings, controls, upload
│   ├── state/              # application state and signals
│   ├── storage/            # IndexedDB and document encryption
│   └── parser/             # PDF, DOCX, TXT, MD parsing
├── docs/SPEC.md            # technical specification and known limits
├── scripts/                # build scripts
├── Cargo.toml
├── .npmrc
├── package.json
├── package-lock.json
├── Trunk.toml
├── vercel.json
└── LICENSE
```

## Documentation

- [Project specification](docs/SPEC.md)

## License

Apache License 2.0. See [LICENSE](LICENSE).

Attribution notices are listed in [NOTICE](NOTICE).
