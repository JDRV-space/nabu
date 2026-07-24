# Document Parser Assets

These files are same-origin runtime parser assets copied from npm packages by:

```bash
npm ci
npm run prepare:assets
```

The package versions are pinned in `package-lock.json`.

- `pdf.min.mjs` and `pdf.worker.min.mjs`: `pdfjs-dist`, Apache-2.0.
- `jszip.min.js`: `jszip`, MIT or GPL-3.0-or-later.
- `document-parsers.js`: local browser glue for PDF and DOCX text extraction
