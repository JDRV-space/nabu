import { copyFile, mkdir } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(fileURLToPath(new URL("../package.json", import.meta.url)));
const vendorDirectory = join(root, "assets", "document-parsers");

const parserAssets = [
  {
    packagePath: "node_modules/pdfjs-dist/build/pdf.min.mjs",
    vendorName: "pdf.min.mjs"
  },
  {
    packagePath: "node_modules/pdfjs-dist/build/pdf.worker.min.mjs",
    vendorName: "pdf.worker.min.mjs"
  },
  {
    packagePath: "node_modules/jszip/dist/jszip.min.js",
    vendorName: "jszip.min.js"
  }
];

await mkdir(vendorDirectory, { recursive: true });

for (const asset of parserAssets) {
  await copyFile(
    join(root, asset.packagePath),
    join(vendorDirectory, asset.vendorName)
  );
  console.log(`vendored ${asset.vendorName}`);
}
