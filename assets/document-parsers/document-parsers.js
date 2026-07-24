import * as pdfjsLib from "./pdf.min.mjs";
import {
  addPdfTextCharacters,
  assertPdfPageCount,
  DOCUMENT_LIMITS,
  inspectDocxArchive,
  readZipEntryBounded,
  validateDocumentXml
} from "./document-limits.js";

const jsZip = globalThis.JSZip;

pdfjsLib.GlobalWorkerOptions.workerSrc = new URL(
  "./pdf.worker.min.mjs",
  import.meta.url
).toString();

globalThis.extractPdfText = async function extractPdfText(arrayBuffer) {
  const loadingTask = pdfjsLib.getDocument({
    data: arrayBuffer,
    disableFontFace: true,
    enableXfa: false,
    isEvalSupported: false,
    stopAtErrors: true
  });
  let pdf;

  try {
    pdf = await loadingTask.promise;
    assertPdfPageCount(pdf.numPages);
    const pages = [];
    let extractedCharacters = 0;

    for (let pageNumber = 1; pageNumber <= pdf.numPages; pageNumber += 1) {
      const page = await pdf.getPage(pageNumber);
      try {
        const textContent = await page.getTextContent();
        const pageParts = [];
        for (const item of textContent.items) {
          if (typeof item.str !== "string") {
            continue;
          }
          const separatorCharacters = pageParts.length === 0 ? 0 : 1;
          extractedCharacters = addPdfTextCharacters(
            extractedCharacters,
            item.str.length + separatorCharacters
          );
          pageParts.push(item.str);
        }

        if (pages.length > 0) {
          extractedCharacters = addPdfTextCharacters(extractedCharacters, 1);
        }
        pages.push(pageParts.join(" "));
      } finally {
        page.cleanup();
      }
    }

    return pages.join("\n").trim();
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(`PDF parsing failed: ${message}`);
  } finally {
    if (pdf) {
      await pdf.destroy();
    } else {
      await loadingTask.destroy();
    }
  }
};

globalThis.extractDocxText = async function extractDocxText(arrayBuffer) {
  if (!jsZip) {
    throw new Error("DOCX parsing failed: JSZip is not loaded");
  }

  try {
    const inspection = inspectDocxArchive(arrayBuffer);
    const zip = await jsZip.loadAsync(arrayBuffer, { createFolders: false });
    const documentEntry = zip.file("word/document.xml");
    if (!documentEntry || documentEntry.dir) {
      throw new Error("DOCX archive has no readable word/document.xml entry");
    }

    const documentBytes = await readZipEntryBounded(
      documentEntry,
      DOCUMENT_LIMITS.maxDocumentXmlBytes,
      inspection.documentXml.uncompressedSize
    );
    const documentXml = new TextDecoder("utf-8", { fatal: true }).decode(
      documentBytes
    );
    validateDocumentXml(documentXml);
    const parsedDocument = new DOMParser().parseFromString(
      documentXml,
      "application/xml"
    );
    if (parsedDocument.querySelector("parsererror")) {
      throw new Error("DOCX document XML is malformed");
    }

    const paragraphs = Array.from(parsedDocument.getElementsByTagName("w:p"));
    const paragraphText = paragraphs
      .map((paragraph) =>
        Array.from(paragraph.getElementsByTagName("w:t"))
          .map((node) => node.textContent ?? "")
          .join("")
          .trim()
      )
      .filter(Boolean)
      .join("\n");

    if (paragraphText) {
      return paragraphText;
    }

    return Array.from(parsedDocument.getElementsByTagName("w:t"))
      .map((node) => node.textContent ?? "")
      .join("")
      .trim();
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(`DOCX parsing failed: ${message}`);
  }
};
