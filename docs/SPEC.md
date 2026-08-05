# Nabu Technical Specification

Nabu is a client-side RSVP speed reader built with Rust, Leptos, WebAssembly,
and browser storage APIs. It imports documents in the browser, stores them in
IndexedDB, and presents text in timed word groups.

## Scope

Nabu is a local browser app. It does not provide accounts, server-side document
storage, sync, collaboration, sharing, or backup. The current reader supports
TXT, Markdown, PDF, and DOCX input, configurable speed, configurable word group
size, fullscreen reading, keyboard controls, and a browser-local document
library.

## Runtime Architecture

```text
index.html
  loads same-origin parser assets, CSS, and the Trunk-generated WASM app

src/main.rs
  mounts the Leptos app

src/components/mod.rs
  routes between the library, reader, and stats views

src/components/
  renders upload, library, reader controls, reader view, and stats UI

src/state/
  owns application state shared by Leptos signals

src/parser/
  validates uploaded files and extracts text from TXT, Markdown, PDF, and DOCX

src/storage/
  persists document records in IndexedDB and encrypts document content with AES-GCM
```

PDF and DOCX parsing depends on browser JavaScript assets under
`assets/document-parsers/`. Those assets are copied from pinned npm packages by
`npm run prepare:assets`; they are not loaded from a third-party CDN at runtime.

## Document Flow

1. The user selects or drops a document in the browser.
2. The upload code checks file size and file signature where practical.
3. Parser code extracts text and sanitizes HTML-derived content.
4. The app tokenizes text into reader words.
5. The document record is encrypted and stored in IndexedDB.
6. The reader displays words using the configured WPM and words-per-flash
   settings.

## Storage And Privacy Limits

Document content is encrypted before being written to IndexedDB. The AES-GCM key
is stored in browser `localStorage` as `nabu_key`.

This protects against accidental server-side document retention because Nabu has
no backend document store. It does not protect against a compromised browser,
same-origin script bug, browser extension, user profile, device compromise, or
someone with access to the browser profile. Clearing site data can delete both
the document library and the stored key.

## Security Controls

- Uploaded text that can contain markup is sanitized with `ammonia`.
- PDF.js and JSZip are served as same-origin static assets copied from pinned
  npm packages.
- PDF parsing disables PDF.js evaluation, limits documents to 2,000 pages,
  limits retained extracted text to 10 million characters, and releases page
  and worker resources after extraction.
- DOCX packages are checked before inflation for unsafe or duplicate paths,
  unsupported ZIP features, entry count, declared sizes, and compression ratio.
- DOCX `word/document.xml` is expanded through a pausable bounded stream, then
  checked for a 10 MiB size limit, declarations/entities, element count, and
  nesting depth before browser XML parsing.
- The deployment CSP is expected to allow same-origin scripts, generated WASM,
  same-origin worker assets, Google Fonts styles/fonts, and local image/blob
  data needed by the app.
- There is no remote document upload path in the application code.

Trunk 0.21.14 generates integrity attributes for its generated JavaScript, CSS,
and WebAssembly resources by default. Parser scripts are same-origin assets and
do not currently use integrity attributes. Deployment security claims remain
unverified until the published artifact and response headers have been checked.

## Parser Dependencies

- `pdfjs-dist`: browser PDF parsing assets.
- `jszip`: DOCX package reading in the browser.
- `pulldown-cmark`: Markdown parsing.
- `ammonia`: HTML sanitization.

## Known Limitations

- Browser-local encryption is not device security.
- PDF and DOCX extraction can fail on malformed, scanned, encrypted, or unusual
  files.
- Large files are constrained by browser memory and IndexedDB behavior.
- The reader calculates session progress for display but does not write the
  document's progress or last-read fields back to IndexedDB.
- The stats view uses default in-memory values; reading sessions do not update
  or persist those statistics.
- There is no account recovery or document backup.

## Validation

The canonical full validation path is:

```bash
npm test
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --locked
./scripts/build.sh
```

`scripts/build.sh` installs locked npm dependencies, regenerates browser parser
assets, and performs the pinned Trunk release build. For docs-only changes, run
`git diff --check` and verify changed links. Browser behavior still needs manual
validation with representative TXT, Markdown, PDF, and DOCX files.
