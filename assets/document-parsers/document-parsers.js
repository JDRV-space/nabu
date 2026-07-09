import * as pdfjsLib from "./pdf.min.mjs";

const jsZip = globalThis.JSZip;

pdfjsLib.GlobalWorkerOptions.workerSrc = new URL(
  "./pdf.worker.min.mjs",
  import.meta.url
).toString();

globalThis.extractPdfText = async function extractPdfText(arrayBuffer) {
  try {
    const pdf = await pdfjsLib.getDocument({ data: arrayBuffer }).promise;
    const pages = [];

    for (let pageNumber = 1; pageNumber <= pdf.numPages; pageNumber += 1) {
      const page = await pdf.getPage(pageNumber);
      const textContent = await page.getTextContent();
      pages.push(textContent.items.map((item) => item.str).join(" "));
    }

    return pages.join("\n").trim();
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(`PDF parsing failed: ${message}`);
  }
};

globalThis.extractDocxText = async function extractDocxText(arrayBuffer) {
  if (!jsZip) {
    throw new Error("DOCX parsing failed: JSZip is not loaded");
  }

  try {
    const zip = await jsZip.loadAsync(arrayBuffer);
    const documentXml = await zip.file("word/document.xml").async("string");
    const parsedDocument = new DOMParser().parseFromString(
      documentXml,
      "application/xml"
    );

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
