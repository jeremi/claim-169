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
}

const MAX_FILE_SIZE = 10 * 1024 * 1024 // 10 MB

const FORMAT_WEBP = 4
const FORMAT_JPEG = 1

/**
 * Compress an image file to a small photo suitable for QR embedding.
 *
 * Uses top-center crop (faces tend to be in upper portion) and resizes to
 * a square of `maxDimension` pixels. Tries WebP first, falls back to JPEG
 * if the browser doesn't support WebP encoding.
 */
export async function compressPhoto(
  file: File,
  options: CompressOptions = {},
): Promise<CompressedPhoto> {
  const { maxDimension = 48, quality = 0.6 } = options

  if (file.size > MAX_FILE_SIZE) {
    throw new Error("File too large (max 10MB)")
  }

  const bitmap = await loadImage(file)

  const canvas = document.createElement("canvas")
  canvas.width = maxDimension
  canvas.height = maxDimension
  const ctx = canvas.getContext("2d")
  if (!ctx) {
    throw new Error("Failed to create canvas 2D context")
  }

  // Top-center crop to square
  const size = Math.min(bitmap.width, bitmap.height)
  const sx = (bitmap.width - size) / 2
  const sy = 0 // top-aligned

  // Light Gaussian blur reduces high-frequency noise before compression,
  // saving 5-15% file size with no visible quality loss at small dimensions.
  ctx.filter = "blur(0.3px)"
  ctx.drawImage(bitmap, sx, sy, size, size, 0, 0, maxDimension, maxDimension)
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
  return {
    data: new Uint8Array(arrayBuffer),
    format,
    width: maxDimension,
    height: maxDimension,
    originalSize: file.size,
  }
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
