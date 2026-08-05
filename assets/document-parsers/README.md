# Document Parser Assets

This directory contains two different ownership classes.

First-party parser policy and browser integration:

- `document-limits.js`
- `document-parsers.js`

Generated third-party distribution files:

- `pdf.min.mjs` and `pdf.worker.min.mjs` from `pdfjs-dist`
- `jszip.min.js` from `jszip`
- `THIRD_PARTY_LICENSES.txt` for every installed npm package used to assemble
  those browser assets

Regenerate the third-party files with:

```bash
npm ci
npm run prepare:assets
```

`package-lock.json` owns exact package versions. Do not hand-edit generated
files; change the npm dependency, run the commands above, and commit all changed
assets and license texts together. Nabu uses JSZip under its MIT license option.
