import assert from "node:assert/strict";
import test from "node:test";
import JSZip from "jszip";

import {
  addPdfTextCharacters,
  assertPdfPageCount,
  DOCUMENT_LIMITS,
  inspectDocxArchive,
  readZipEntryBounded,
  validateDocumentXml
} from "../assets/document-parsers/document-limits.js";

const CONTENT_TYPES_XML =
  '<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>';
const RELATIONSHIPS_XML =
  '<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"/>';
const DOCUMENT_XML =
  '<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>Safe text</w:t></w:r></w:p></w:body></w:document>';

async function makeDocx(documentXml = DOCUMENT_XML, additionalEntries = {}) {
  const zip = new JSZip();
  zip.file("[Content_Types].xml", CONTENT_TYPES_XML);
  zip.file("_rels/.rels", RELATIONSHIPS_XML);
  zip.file("word/document.xml", documentXml);
  for (const [name, content] of Object.entries(additionalEntries)) {
    zip.file(name, content);
  }
  return zip.generateAsync({
    type: "uint8array",
    compression: "DEFLATE",
    compressionOptions: { level: 9 }
  });
}

function findCentralDirectoryEntry(bytes, expectedName) {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  for (let offset = 0; offset <= bytes.byteLength - 46; offset += 1) {
    if (view.getUint32(offset, true) !== 0x02014b50) {
      continue;
    }
    const nameLength = view.getUint16(offset + 28, true);
    const name = new TextDecoder().decode(
      bytes.subarray(offset + 46, offset + 46 + nameLength)
    );
    if (name === expectedName) {
      return offset;
    }
  }
  throw new Error(`Missing central directory entry ${expectedName}`);
}

test("accepts a bounded DOCX and streams its document XML", async () => {
  const archive = await makeDocx();
  const inspection = inspectDocxArchive(archive);
  const zip = await JSZip.loadAsync(archive);
  const documentBytes = await readZipEntryBounded(
    zip.file("word/document.xml"),
    DOCUMENT_LIMITS.maxDocumentXmlBytes,
    inspection.documentXml.uncompressedSize
  );

  assert.equal(inspection.entryCount, 5);
  assert.equal(new TextDecoder().decode(documentBytes), DOCUMENT_XML);
  assert.doesNotThrow(() => validateDocumentXml(DOCUMENT_XML));
});

test("rejects excessive PDF pages and extracted text", () => {
  assert.doesNotThrow(() => assertPdfPageCount(DOCUMENT_LIMITS.maxPdfPages));
  assert.throws(
    () => assertPdfPageCount(DOCUMENT_LIMITS.maxPdfPages + 1),
    /page limit/
  );
  assert.equal(addPdfTextCharacters(10, 20), 30);
  assert.throws(
    () =>
      addPdfTextCharacters(
        DOCUMENT_LIMITS.maxPdfTextCharacters,
        1
      ),
    /extracted text limit/
  );
});

test("rejects archives with unreasonable entry counts", async () => {
  const archive = await makeDocx(DOCUMENT_XML, { "word/extra.xml": "<extra/>" });

  assert.throws(
    () =>
      inspectDocxArchive(archive, {
        ...DOCUMENT_LIMITS,
        maxEntryCount: 3
      }),
    /3-entry limit/
  );
});

test("rejects highly compressed document parts before inflation", async () => {
  const archive = await makeDocx(
    DOCUMENT_XML.replace("Safe text", "A".repeat(20_000))
  );

  assert.throws(
    () =>
      inspectDocxArchive(archive, {
        ...DOCUMENT_LIMITS,
        maxCompressionRatio: 10
      }),
    /compression ratio limit/
  );
});

test("rejects oversized document XML before inflation", async () => {
  const archive = await makeDocx(
    DOCUMENT_XML.replace("Safe text", "A".repeat(512))
  );

  assert.throws(
    () =>
      inspectDocxArchive(archive, {
        ...DOCUMENT_LIMITS,
        maxCompressionRatio: 10_000,
        maxDocumentXmlBytes: 256
      }),
    /document XML exceeds/
  );
});

test("stops streamed inflation when a forged size header understates output", async () => {
  const archive = await makeDocx(
    DOCUMENT_XML.replace("Safe text", "A".repeat(20_000))
  );
  const forgedArchive = archive.slice();
  const forgedView = new DataView(
    forgedArchive.buffer,
    forgedArchive.byteOffset,
    forgedArchive.byteLength
  );
  const centralOffset = findCentralDirectoryEntry(
    forgedArchive,
    "word/document.xml"
  );
  forgedView.setUint32(centralOffset + 24, 32, true);

  const zip = await JSZip.loadAsync(forgedArchive);
  await assert.rejects(
    readZipEntryBounded(zip.file("word/document.xml"), 1024, 32),
    /runtime inflation limit/
  );
});

test("rejects declarations, excessive elements, and excessive XML depth", () => {
  assert.throws(
    () =>
      validateDocumentXml(
        '<!DOCTYPE w:document [<!ENTITY bomb "boom">]><w:document/>'
      ),
    /declarations and entities/
  );
  assert.throws(
    () =>
      validateDocumentXml("<root><a/><b/></root>", {
        ...DOCUMENT_LIMITS,
        maxXmlElements: 2
      }),
    /element limit/
  );
  assert.throws(
    () =>
      validateDocumentXml("<root><a><b><c/></b></a></root>", {
        ...DOCUMENT_LIMITS,
        maxXmlDepth: 2
      }),
    /nesting depth limit/
  );
});
