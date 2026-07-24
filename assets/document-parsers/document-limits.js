const END_OF_CENTRAL_DIRECTORY_SIGNATURE = 0x06054b50;
const CENTRAL_DIRECTORY_SIGNATURE = 0x02014b50;
const LOCAL_FILE_HEADER_SIGNATURE = 0x04034b50;
const ZIP64_SENTINEL_16 = 0xffff;
const ZIP64_SENTINEL_32 = 0xffffffff;
const MAX_ZIP_COMMENT_BYTES = 0xffff;
const END_OF_CENTRAL_DIRECTORY_BYTES = 22;
const CENTRAL_DIRECTORY_HEADER_BYTES = 46;
const LOCAL_FILE_HEADER_BYTES = 30;
const UTF8_FLAG = 0x0800;
const ENCRYPTED_FLAG = 0x0001;
const SUPPORTED_COMPRESSION_METHODS = new Set([0, 8]);
const REQUIRED_DOCX_ENTRIES = new Set([
  "[Content_Types].xml",
  "_rels/.rels",
  "word/document.xml"
]);

export const DOCUMENT_LIMITS = Object.freeze({
  maxArchiveBytes: 50 * 1024 * 1024,
  maxPdfPages: 2_000,
  maxPdfTextCharacters: 10_000_000,
  maxEntryCount: 2048,
  maxEntryNameBytes: 1024,
  maxEntryUncompressedBytes: 50 * 1024 * 1024,
  maxTotalUncompressedBytes: 100 * 1024 * 1024,
  maxCompressionRatio: 100,
  maxDocumentXmlBytes: 10 * 1024 * 1024,
  maxXmlElements: 100_000,
  maxXmlDepth: 256
});

export class DocumentLimitError extends Error {
  constructor(message) {
    super(message);
    this.name = "DocumentLimitError";
  }
}

export function assertPdfPageCount(pageCount, limits = DOCUMENT_LIMITS) {
  if (
    !Number.isSafeInteger(pageCount) ||
    pageCount < 1 ||
    pageCount > limits.maxPdfPages
  ) {
    throw new DocumentLimitError(
      `PDF exceeds the ${limits.maxPdfPages}-page limit`
    );
  }
}

export function addPdfTextCharacters(
  currentCharacters,
  additionalCharacters,
  limits = DOCUMENT_LIMITS
) {
  const totalCharacters = currentCharacters + additionalCharacters;
  if (
    !Number.isSafeInteger(currentCharacters) ||
    !Number.isSafeInteger(additionalCharacters) ||
    currentCharacters < 0 ||
    additionalCharacters < 0 ||
    totalCharacters > limits.maxPdfTextCharacters
  ) {
    throw new DocumentLimitError(
      "PDF exceeds the extracted text limit"
    );
  }
  return totalCharacters;
}

function asBytes(data) {
  if (data instanceof Uint8Array) {
    return data;
  }
  if (data instanceof ArrayBuffer) {
    return new Uint8Array(data);
  }
  if (ArrayBuffer.isView(data)) {
    return new Uint8Array(data.buffer, data.byteOffset, data.byteLength);
  }
  throw new DocumentLimitError("DOCX archive must be binary data");
}

function findEndOfCentralDirectory(bytes, view) {
  const minimumOffset = Math.max(
    0,
    bytes.byteLength -
      END_OF_CENTRAL_DIRECTORY_BYTES -
      MAX_ZIP_COMMENT_BYTES
  );

  for (
    let offset = bytes.byteLength - END_OF_CENTRAL_DIRECTORY_BYTES;
    offset >= minimumOffset;
    offset -= 1
  ) {
    if (
      view.getUint32(offset, true) === END_OF_CENTRAL_DIRECTORY_SIGNATURE
    ) {
      const commentLength = view.getUint16(offset + 20, true);
      if (
        offset + END_OF_CENTRAL_DIRECTORY_BYTES + commentLength ===
        bytes.byteLength
      ) {
        return offset;
      }
    }
  }

  throw new DocumentLimitError("DOCX archive has no valid central directory");
}

function decodeEntryName(bytes, flags) {
  const decoder = new TextDecoder("utf-8", {
    fatal: (flags & UTF8_FLAG) !== 0
  });

  try {
    return decoder.decode(bytes);
  } catch {
    throw new DocumentLimitError("DOCX archive has an invalid entry name");
  }
}

function validateEntryName(name) {
  const segments = name.split("/");
  if (
    !name ||
    name.startsWith("/") ||
    name.startsWith("\\") ||
    name.includes("\\") ||
    name.includes("\0") ||
    segments.some((segment) => segment === "." || segment === "..")
  ) {
    throw new DocumentLimitError(
      `DOCX archive has an unsafe entry name: ${name || "<empty>"}`
    );
  }
}

function assertRange(start, length, upperBound, message) {
  if (
    !Number.isSafeInteger(start) ||
    !Number.isSafeInteger(length) ||
    start < 0 ||
    length < 0 ||
    start + length > upperBound
  ) {
    throw new DocumentLimitError(message);
  }
}

function inspectLocalEntry(bytes, view, centralOffset, entry) {
  assertRange(
    entry.localHeaderOffset,
    LOCAL_FILE_HEADER_BYTES,
    centralOffset,
    `DOCX entry ${entry.name} has an invalid local header`
  );

  if (
    view.getUint32(entry.localHeaderOffset, true) !==
    LOCAL_FILE_HEADER_SIGNATURE
  ) {
    throw new DocumentLimitError(
      `DOCX entry ${entry.name} has an invalid local header`
    );
  }

  const localFlags = view.getUint16(entry.localHeaderOffset + 6, true);
  const localCompressionMethod = view.getUint16(
    entry.localHeaderOffset + 8,
    true
  );
  const localNameLength = view.getUint16(entry.localHeaderOffset + 26, true);
  const localExtraLength = view.getUint16(entry.localHeaderOffset + 28, true);
  const localNameOffset = entry.localHeaderOffset + LOCAL_FILE_HEADER_BYTES;
  const dataOffset = localNameOffset + localNameLength + localExtraLength;

  assertRange(
    localNameOffset,
    localNameLength + localExtraLength + entry.compressedSize,
    centralOffset,
    `DOCX entry ${entry.name} extends outside the archive payload`
  );

  const localName = decodeEntryName(
    bytes.subarray(localNameOffset, localNameOffset + localNameLength),
    localFlags
  );
  if (
    localName !== entry.name ||
    localFlags !== entry.flags ||
    localCompressionMethod !== entry.compressionMethod
  ) {
    throw new DocumentLimitError(
      `DOCX entry ${entry.name} has inconsistent headers`
    );
  }

  return {
    start: entry.localHeaderOffset,
    end: dataOffset + entry.compressedSize,
    name: entry.name
  };
}

export function inspectDocxArchive(data, limits = DOCUMENT_LIMITS) {
  const bytes = asBytes(data);
  if (bytes.byteLength > limits.maxArchiveBytes) {
    throw new DocumentLimitError("DOCX archive exceeds the compressed size limit");
  }
  if (bytes.byteLength < END_OF_CENTRAL_DIRECTORY_BYTES) {
    throw new DocumentLimitError("DOCX archive is truncated");
  }

  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const endOffset = findEndOfCentralDirectory(bytes, view);
  const diskNumber = view.getUint16(endOffset + 4, true);
  const centralDirectoryDisk = view.getUint16(endOffset + 6, true);
  const entriesOnDisk = view.getUint16(endOffset + 8, true);
  const entryCount = view.getUint16(endOffset + 10, true);
  const centralDirectorySize = view.getUint32(endOffset + 12, true);
  const centralDirectoryOffset = view.getUint32(endOffset + 16, true);

  if (
    diskNumber !== 0 ||
    centralDirectoryDisk !== 0 ||
    entriesOnDisk !== entryCount
  ) {
    throw new DocumentLimitError("Multi-volume DOCX archives are not supported");
  }
  if (
    entryCount === ZIP64_SENTINEL_16 ||
    centralDirectorySize === ZIP64_SENTINEL_32 ||
    centralDirectoryOffset === ZIP64_SENTINEL_32
  ) {
    throw new DocumentLimitError("ZIP64 DOCX archives are not supported");
  }
  if (entryCount === 0 || entryCount > limits.maxEntryCount) {
    throw new DocumentLimitError(
      `DOCX archive exceeds the ${limits.maxEntryCount}-entry limit`
    );
  }
  if (centralDirectoryOffset + centralDirectorySize !== endOffset) {
    throw new DocumentLimitError(
      "DOCX archive has an invalid central directory range"
    );
  }

  const entries = new Map();
  let totalUncompressedBytes = 0;
  let offset = centralDirectoryOffset;

  for (let index = 0; index < entryCount; index += 1) {
    assertRange(
      offset,
      CENTRAL_DIRECTORY_HEADER_BYTES,
      endOffset,
      "DOCX central directory is truncated"
    );
    if (view.getUint32(offset, true) !== CENTRAL_DIRECTORY_SIGNATURE) {
      throw new DocumentLimitError("DOCX central directory is malformed");
    }

    const flags = view.getUint16(offset + 8, true);
    const compressionMethod = view.getUint16(offset + 10, true);
    const compressedSize = view.getUint32(offset + 20, true);
    const uncompressedSize = view.getUint32(offset + 24, true);
    const nameLength = view.getUint16(offset + 28, true);
    const extraLength = view.getUint16(offset + 30, true);
    const commentLength = view.getUint16(offset + 32, true);
    const diskStart = view.getUint16(offset + 34, true);
    const localHeaderOffset = view.getUint32(offset + 42, true);
    const entryLength =
      CENTRAL_DIRECTORY_HEADER_BYTES +
      nameLength +
      extraLength +
      commentLength;

    assertRange(
      offset,
      entryLength,
      endOffset,
      "DOCX central directory entry is truncated"
    );
    if (
      compressedSize === ZIP64_SENTINEL_32 ||
      uncompressedSize === ZIP64_SENTINEL_32 ||
      localHeaderOffset === ZIP64_SENTINEL_32 ||
      diskStart === ZIP64_SENTINEL_16
    ) {
      throw new DocumentLimitError("ZIP64 DOCX entries are not supported");
    }
    if (diskStart !== 0) {
      throw new DocumentLimitError("Multi-volume DOCX entries are not supported");
    }
    if ((flags & ENCRYPTED_FLAG) !== 0) {
      throw new DocumentLimitError("Encrypted DOCX entries are not supported");
    }
    if (!SUPPORTED_COMPRESSION_METHODS.has(compressionMethod)) {
      throw new DocumentLimitError(
        `DOCX entry uses unsupported compression method ${compressionMethod}`
      );
    }
    if (nameLength === 0 || nameLength > limits.maxEntryNameBytes) {
      throw new DocumentLimitError("DOCX entry name exceeds the allowed size");
    }

    const nameOffset = offset + CENTRAL_DIRECTORY_HEADER_BYTES;
    const name = decodeEntryName(
      bytes.subarray(nameOffset, nameOffset + nameLength),
      flags
    );
    validateEntryName(name);
    if (entries.has(name)) {
      throw new DocumentLimitError(`DOCX archive has duplicate entry ${name}`);
    }
    if (uncompressedSize > limits.maxEntryUncompressedBytes) {
      throw new DocumentLimitError(
        `DOCX entry ${name} exceeds the uncompressed size limit`
      );
    }
    if (
      uncompressedSize > 0 &&
      (compressedSize === 0 ||
        uncompressedSize / compressedSize > limits.maxCompressionRatio)
    ) {
      throw new DocumentLimitError(
        `DOCX entry ${name} exceeds the compression ratio limit`
      );
    }

    totalUncompressedBytes += uncompressedSize;
    if (totalUncompressedBytes > limits.maxTotalUncompressedBytes) {
      throw new DocumentLimitError(
        "DOCX archive exceeds the total uncompressed size limit"
      );
    }

    entries.set(name, {
      name,
      flags,
      compressionMethod,
      compressedSize,
      uncompressedSize,
      localHeaderOffset
    });
    offset += entryLength;
  }

  if (offset !== endOffset) {
    throw new DocumentLimitError("DOCX central directory size is inconsistent");
  }
  for (const requiredEntry of REQUIRED_DOCX_ENTRIES) {
    if (!entries.has(requiredEntry)) {
      throw new DocumentLimitError(
        `DOCX archive is missing required entry ${requiredEntry}`
      );
    }
  }

  const documentXml = entries.get("word/document.xml");
  if (documentXml.uncompressedSize > limits.maxDocumentXmlBytes) {
    throw new DocumentLimitError(
      "DOCX document XML exceeds the uncompressed size limit"
    );
  }

  const payloadRanges = [...entries.values()]
    .map((entry) =>
      inspectLocalEntry(bytes, view, centralDirectoryOffset, entry)
    )
    .sort((left, right) => left.start - right.start);
  for (let index = 1; index < payloadRanges.length; index += 1) {
    if (payloadRanges[index].start < payloadRanges[index - 1].end) {
      throw new DocumentLimitError(
        `DOCX entries ${payloadRanges[index - 1].name} and ${payloadRanges[index].name} overlap`
      );
    }
  }

  return {
    entryCount,
    totalUncompressedBytes,
    documentXml
  };
}

export function readZipEntryBounded(
  entry,
  maxBytes,
  expectedBytes = undefined
) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    let totalBytes = 0;
    let settled = false;
    const stream = entry.internalStream("uint8array");

    const fail = (error) => {
      if (settled) {
        return;
      }
      settled = true;
      stream.pause();
      reject(error);
    };

    stream
      .on("data", (chunk) => {
        if (settled) {
          return;
        }
        totalBytes += chunk.byteLength;
        if (totalBytes > maxBytes) {
          fail(
            new DocumentLimitError(
              "DOCX document XML exceeded its runtime inflation limit"
            )
          );
          return;
        }
        chunks.push(chunk);
      })
      .on("error", (error) => {
        fail(error);
      })
      .on("end", () => {
        if (settled) {
          return;
        }
        if (expectedBytes !== undefined && totalBytes !== expectedBytes) {
          fail(
            new DocumentLimitError(
              "DOCX document XML size does not match its archive header"
            )
          );
          return;
        }

        const result = new Uint8Array(totalBytes);
        let resultOffset = 0;
        for (const chunk of chunks) {
          result.set(chunk, resultOffset);
          resultOffset += chunk.byteLength;
        }
        settled = true;
        resolve(result);
      });

    stream.resume();
  });
}

function findMarkupEnd(xml, startOffset) {
  let quote = null;
  for (let offset = startOffset; offset < xml.length; offset += 1) {
    const character = xml[offset];
    if (quote !== null) {
      if (character === quote) {
        quote = null;
      }
    } else if (character === '"' || character === "'") {
      quote = character;
    } else if (character === ">") {
      return offset;
    }
  }
  return -1;
}

function findRequiredTerminator(xml, startOffset, terminator) {
  const endOffset = xml.indexOf(terminator, startOffset);
  if (endOffset === -1) {
    throw new DocumentLimitError("DOCX document XML is malformed");
  }
  return endOffset + terminator.length;
}

export function validateDocumentXml(xml, limits = DOCUMENT_LIMITS) {
  let elementCount = 0;
  let depth = 0;
  let offset = 0;

  while ((offset = xml.indexOf("<", offset)) !== -1) {
    if (xml.startsWith("<!--", offset)) {
      offset = findRequiredTerminator(xml, offset + 4, "-->");
      continue;
    }
    if (xml.startsWith("<![CDATA[", offset)) {
      offset = findRequiredTerminator(xml, offset + 9, "]]>");
      continue;
    }
    if (xml.startsWith("<?", offset)) {
      offset = findRequiredTerminator(xml, offset + 2, "?>");
      continue;
    }
    if (xml.startsWith("<!", offset)) {
      throw new DocumentLimitError(
        "DOCX document XML declarations and entities are not allowed"
      );
    }

    const endOffset = findMarkupEnd(xml, offset + 1);
    if (endOffset === -1) {
      throw new DocumentLimitError("DOCX document XML is malformed");
    }

    const markup = xml.slice(offset + 1, endOffset).trim();
    if (markup.startsWith("/")) {
      depth -= 1;
      if (depth < 0) {
        throw new DocumentLimitError("DOCX document XML is malformed");
      }
    } else {
      const name = markup.match(/^([A-Za-z_][\w:.-]*)/u);
      if (!name) {
        throw new DocumentLimitError("DOCX document XML is malformed");
      }
      elementCount += 1;
      if (elementCount > limits.maxXmlElements) {
        throw new DocumentLimitError(
          "DOCX document XML exceeds the element limit"
        );
      }
      if (!markup.endsWith("/")) {
        depth += 1;
        if (depth > limits.maxXmlDepth) {
          throw new DocumentLimitError(
            "DOCX document XML exceeds the nesting depth limit"
          );
        }
      }
    }
    offset = endOffset + 1;
  }

  if (depth !== 0) {
    throw new DocumentLimitError("DOCX document XML is malformed");
  }
}
