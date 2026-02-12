export interface CompressedPhoto {
  data: Uint8Array
  format: number
  width: number
  height: number
  originalSize: number
}

export interface CompressOptions {
  maxDimension?: number
  quality?: number
  grayscale?: boolean
}

const MAX_FILE_SIZE = 10 * 1024 * 1024 // 10 MB

const FORMAT_WEBP = 4
const FORMAT_JPEG = 1

const PORTRAIT_ASPECT = 3 / 4 // width / height

/**
 * Compress an image file to a small portrait photo suitable for QR embedding.
 *
 * Uses a 3:4 portrait crop (matching passport photo proportions) with
 * upper-center bias. Fills the canvas white to eliminate the alpha channel,
 * and strips the ICC profile from WebP output to minimize byte overhead.
 * Tries WebP first, falls back to JPEG if the browser doesn't support WebP encoding.
 */
export async function compressPhoto(
  file: File,
  options: CompressOptions = {},
): Promise<CompressedPhoto> {
  const { maxDimension = 48, quality = 0.6, grayscale = false } = options

  if (file.size > MAX_FILE_SIZE) {
    throw new Error("File too large (max 10MB)")
  }

  const bitmap = await loadImage(file)

  const width = maxDimension
  const height = Math.round(maxDimension * 4 / 3)

  const canvas = document.createElement("canvas")
  canvas.width = width
  canvas.height = height
  const ctx = canvas.getContext("2d")
  if (!ctx) {
    throw new Error("Failed to create canvas 2D context")
  }

  // 3:4 portrait crop — constrain to source dimensions
  let cropW: number, cropH: number
  if (bitmap.width / bitmap.height > PORTRAIT_ASPECT) {
    // Source is wider than 3:4 — constrain by height
    cropH = bitmap.height
    cropW = Math.round(cropH * PORTRAIT_ASPECT)
  } else {
    // Source is taller than 3:4 — constrain by width
    cropW = bitmap.width
    cropH = Math.round(cropW / PORTRAIT_ASPECT)
  }

  // Center horizontally, bias toward top vertically (faces in upper portion)
  const sx = (bitmap.width - cropW) / 2
  const sy = Math.max(0, (bitmap.height - cropH) * 0.2)

  console.log(`[photo] Crop: sx=${sx.toFixed(0)} sy=${sy.toFixed(0)} cropW=${cropW} cropH=${cropH}`)
  console.log(`[photo] Source: ${bitmap.width}×${bitmap.height} → canvas: ${width}×${height}`)

  // Fill white to ensure opaque pixels (no alpha channel in output)
  ctx.fillStyle = "#ffffff"
  ctx.fillRect(0, 0, width, height)

  if (grayscale) {
    ctx.filter = "grayscale(1)"
  }
  ctx.drawImage(bitmap, sx, sy, cropW, cropH, 0, 0, width, height)
  ctx.filter = "none"

  // Try WebP first
  let blob = await canvasToBlob(canvas, "image/webp", quality)

  let format: number
  if (blob.type === "image/webp") {
    format = FORMAT_WEBP
  } else {
    // Browser produced PNG instead of WebP — fall back to JPEG
    blob = await canvasToBlob(canvas, "image/jpeg", quality)
    format = FORMAT_JPEG
  }

  const arrayBuffer = await blob.arrayBuffer()
  let photoData = new Uint8Array(arrayBuffer)
  if (format === FORMAT_WEBP) {
    photoData = new Uint8Array(stripWebpIccProfile(photoData))
  }
  return {
    data: photoData,
    format,
    width,
    height,
    originalSize: file.size,
  }
}

export interface FitResult extends CompressedPhoto {
  qualityUsed: number // 0.0-1.0
  reachedTarget: boolean // false if min quality still exceeds maxBytes
}

/**
 * Binary-search over quality to compress a photo within a byte budget.
 *
 * Runs up to 8 iterations of binary search (covers 10%-100% at <1% precision).
 * Returns the highest quality that fits within `maxBytes`, or the minimum-quality
 * result with `reachedTarget: false` if the target is unreachable.
 */
export async function compressPhotoToFit(
  file: File,
  options: CompressOptions & { maxBytes: number },
): Promise<FitResult> {
  const { maxBytes, maxDimension, grayscale } = options

  let low = 0.1
  let high = 1.0
  let bestResult: CompressedPhoto | null = null
  let bestQuality = low

  for (let i = 0; i < 8; i++) {
    const mid = (low + high) / 2
    const result = await compressPhoto(file, {
      maxDimension,
      quality: mid,
      grayscale,
    })

    if (result.data.length <= maxBytes) {
      bestResult = result
      bestQuality = mid
      low = mid
    } else {
      high = mid
    }
  }

  // Final compression at the best quality found
  if (bestResult) {
    return { ...bestResult, qualityUsed: bestQuality, reachedTarget: true }
  }

  // Even minimum quality exceeds target — return best-effort result
  const fallback = await compressPhoto(file, {
    maxDimension,
    quality: 0.1,
    grayscale,
  })
  return { ...fallback, qualityUsed: 0.1, reachedTarget: false }
}

/**
 * Create an object URL for previewing a photo from its raw bytes and format code.
 */
export function createPhotoPreviewUrl(
  data: Uint8Array,
  photoFormat: number | undefined,
): string {
  const mime = photoFormatToMime(photoFormat)
  // Copy to a standalone ArrayBuffer to avoid SharedArrayBuffer compatibility issues
  const copy = new Uint8Array(data).buffer as ArrayBuffer
  const blob = new Blob([copy], { type: mime })
  return URL.createObjectURL(blob)
}

/**
 * Map Claim 169 photo format code to MIME type.
 */
export function photoFormatToMime(format: number | undefined): string {
  switch (format) {
    case 1:
      return "image/jpeg"
    case 2:
      return "image/jp2"
    case 3:
      return "image/avif"
    case 4:
      return "image/webp"
    default:
      return "application/octet-stream"
  }
}

function loadImage(file: File): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const url = URL.createObjectURL(file)
    const img = new Image()
    img.onload = () => {
      URL.revokeObjectURL(url)
      resolve(img)
    }
    img.onerror = () => {
      URL.revokeObjectURL(url)
      reject(new Error("Not a valid image file"))
    }
    img.src = url
  })
}

function canvasToBlob(
  canvas: HTMLCanvasElement,
  mimeType: string,
  quality: number,
): Promise<Blob> {
  return new Promise((resolve, reject) => {
    canvas.toBlob(
      (blob) => {
        if (!blob) {
          reject(new Error("Canvas toBlob returned null"))
          return
        }
        resolve(blob)
      },
      mimeType,
      quality,
    )
  })
}

/**
 * Strip the ICC color profile (ICCP chunk) from a WebP file.
 *
 * Chrome's canvas.toBlob embeds an sRGB ICC profile (~456 bytes) in every
 * WebP via the VP8X extended format. At small dimensions (48×64) this
 * metadata can exceed the actual image data. This function extracts the
 * VP8/VP8L bitstream and rewraps it in a minimal RIFF container.
 */
function stripWebpIccProfile(data: Uint8Array): Uint8Array {
  // Minimum valid WebP: 12-byte RIFF header + at least one chunk header
  if (data.length < 20) return data

  const view = new DataView(data.buffer, data.byteOffset, data.byteLength)

  // Verify RIFF + WEBP signature
  const riff =
    data[0] === 0x52 &&
    data[1] === 0x49 &&
    data[2] === 0x46 &&
    data[3] === 0x46
  const webp =
    data[8] === 0x57 &&
    data[9] === 0x45 &&
    data[10] === 0x42 &&
    data[11] === 0x50
  if (!riff || !webp) return data

  // Check first chunk FourCC at offset 12
  const fourCC = String.fromCharCode(data[12], data[13], data[14], data[15])
  if (fourCC !== "VP8X") {
    // Already simple format (VP8 or VP8L) — no stripping needed
    return data
  }

  // Walk chunks after VP8X to find the VP8/VP8L image data chunk
  // VP8X chunk: 4 (fourCC) + 4 (size) + 10 (payload) = 18 bytes
  let offset = 12 + 4 + 4 + 10 // skip RIFF header (12) + VP8X chunk (18)

  while (offset + 8 <= data.length) {
    const chunkFourCC = String.fromCharCode(
      data[offset],
      data[offset + 1],
      data[offset + 2],
      data[offset + 3],
    )
    const chunkSize = view.getUint32(offset + 4, true)
    // RIFF chunks are padded to even size
    const paddedSize = chunkSize + (chunkSize % 2)

    if (chunkFourCC === "VP8 " || chunkFourCC === "VP8L") {
      // Found the image chunk — rewrap in a minimal RIFF container
      const chunkTotalSize = 8 + paddedSize // fourCC + size + data
      const riffSize = 4 + chunkTotalSize // "WEBP" + chunk
      const result = new Uint8Array(12 + chunkTotalSize)
      const resultView = new DataView(result.buffer)

      // RIFF header
      result.set([0x52, 0x49, 0x46, 0x46]) // "RIFF"
      resultView.setUint32(4, riffSize, true)
      result.set([0x57, 0x45, 0x42, 0x50], 8) // "WEBP"

      // Copy VP8/VP8L chunk as-is
      result.set(data.subarray(offset, offset + chunkTotalSize), 12)

      console.log(
        `[photo] Stripped ICC profile: ${data.length} → ${result.length} bytes`,
      )
      return result
    }

    offset += 8 + paddedSize
  }

  // No VP8/VP8L chunk found — return original
  return data
}
