# Photo Compression Guide for Claim 169 QR Codes

This guide covers best practices for embedding identity photos in Claim 169 QR codes. QR codes have limited capacity (~2,331 alphanumeric characters at Version 40, Level M), so photos must be aggressively compressed to fit alongside demographic data and cryptographic signatures.

## Byte Budget

A typical Claim 169 QR code at Version 20 (~1,156 Base45 characters) has roughly this internal budget:

| Component | Bytes |
|-----------|-------|
| COSE/CWT overhead | 50-80 |
| Ed25519 signature | 64 |
| Demographic text fields | 100-300 |
| **Photo** | **400-700** |
| **Total** | **~1,005** |

The MOSIP specification targets 1,005 bytes for the entire payload. Ed25519 signatures (64 bytes) leave significantly more room for photo data than RSA-2048 (256 bytes), which is one reason the spec favors EdDSA.

## Format Selection

At 32-64px resolution, **container overhead dominates** total file size. This makes format choice critical:

| Format | Container Overhead | Claim 169 Code | Recommendation |
|--------|-------------------|-----------------|----------------|
| WebP | ~30 bytes | 4 | **Best choice** |
| JPEG | ~155 bytes | 1 | Acceptable fallback |
| JPEG2000 | ~200+ bytes | 2 | Avoid for tiny images |
| AVIF | ~303 bytes | 3 | Avoid for tiny images |

**WebP is the optimal format** for this use case. Its RIFF container adds only ~20-30 bytes of overhead, and its VP8 codec uses 4x4 sub-block prediction that adapts well to fine facial detail. AVIF and JPEG2000 have superior compression algorithms, but their container overhead (303 and 200+ bytes respectively) consumes 37-75% of the photo byte budget before encoding a single pixel.

JPEG is an acceptable fallback for browsers without WebP encoding support (Safari < 16). Its 8x8 DCT blocks produce more visible artifacts at small sizes than WebP's 4x4 sub-blocks, but the file sizes are still workable.

### Browser WebP Detection

`canvas.toBlob("image/webp")` silently produces a PNG blob on browsers that lack WebP encoding support. Always check `blob.type` after encoding:

```typescript
let blob = await canvasToBlob(canvas, "image/webp", quality)
if (blob.type !== "image/webp") {
  // Browser doesn't support WebP encoding — fall back to JPEG
  blob = await canvasToBlob(canvas, "image/jpeg", quality)
}
```

Set the `photoFormat` field based on the **actual** output MIME type, not the requested format.

## Resolution

### Standards Context

Formal identity document standards set high resolution requirements that are not applicable to QR-embedded photos:

| Standard | Minimum | Purpose |
|----------|---------|---------|
| ISO/IEC 19794-5 | 90px inter-eye distance | Automated face recognition |
| ICAO Doc 9303 | 300 DPI, 35x45mm | Passport printing |
| ISO 18013-5 (mDL) | 192x240px | Digital driver's license |

These requirements target automated biometric matching. QR-embedded photos serve a different purpose: **visual verification by a human** (comparing the photo to the person present).

### Recommended Resolutions

| Resolution | Pixel Count | Estimated Size (WebP q=60) | Use Case |
|-----------|-------------|---------------------------|----------|
| 32x32 | 1,024 | 200-350 bytes | Minimal, tight byte budget |
| **48x48** | **2,304** | **400-700 bytes** | **Default — best tradeoff** |
| 56x56 | 3,136 | 500-900 bytes | Better detail if budget allows |
| 64x64 | 4,096 | 700-1,200 bytes | High quality, large QR only |

**48x48 is the recommended default.** Research shows humans can recognize faces at as low as 16x16 pixels (Harmon and Julesz), so 48x48 provides comfortable headroom for visual verification while staying within the byte budget.

If the QR code has room (few demographic fields, using Version 40), 56x56 provides noticeably better detail for only ~100-200 additional bytes.

## Quality Setting

For WebP at 48x48 resolution:

| Quality | File Size | Visual Quality |
|---------|-----------|---------------|
| 40-50% | Smallest | Noticeable degradation |
| **55-65%** | **Good tradeoff** | **Adequate for identification** |
| 65-80% | 20-40% larger | Marginal visible improvement |
| 80-100% | Diminishing returns | Not justified at this resolution |

**60% quality is recommended.** Going lower saves only 30-50 bytes with visible quality loss. Going higher costs 50-100+ bytes with minimal visible improvement.

## Preprocessing

### Crop Strategy

Use **top-center crop to square**:

- Faces tend to appear in the upper portion of photos
- At 48x48, even one pixel of vertical offset matters
- Square crop is preferred over 3:4 portrait because faces need horizontal detail (ears, jaw line) as much as vertical

### Gaussian Blur

Apply a **light Gaussian blur (0.3px radius)** before encoding:

- Reduces high-frequency sensor noise that compressors struggle with
- Saves 5-15% file size with no perceptible quality loss at 48x48
- At this resolution, a 0.3px blur removes noise without softening facial features

```typescript
ctx.filter = "blur(0.3px)"
ctx.drawImage(source, sx, sy, size, size, 0, 0, dimension, dimension)
ctx.filter = "none"
```

### Color vs Grayscale

**Keep color photos.** Research shows that at low resolutions, color actively helps human visual recognition because shape cues are degraded and color provides additional discriminating features. The byte savings from grayscale (~60-120 bytes at 48x48) are not worth the tradeoff. If desperately tight on bytes, grayscale is a viable last resort.

### Avoid Histogram Equalization

CLAHE or histogram equalization can increase dynamic range, which increases entropy and file size. At 48x48, the visual improvement is marginal and can make the photo look unnatural.

## Pipeline Interaction with zlib

The Claim 169 encoding pipeline applies zlib compression after photo encoding:

```
Photo (WebP/JPEG) -> CBOR -> CWT -> COSE_Sign1 -> zlib -> Base45 -> QR
```

Compressed image formats (WebP, JPEG) produce high-entropy output that zlib cannot further compress. This means:

- **Do not use raw/uncompressed pixel data** hoping zlib will handle it. A raw 48x48 RGB image is 6,912 bytes; even with zlib compression it would be ~2,300-3,500 bytes. WebP at q=60 produces 400-700 bytes.
- The zlib stage primarily benefits the text/structural parts of the CBOR payload (demographic fields, COSE headers).
- Photo bytes are effectively passed through zlib unchanged.

## What Similar Systems Do

### India's Aadhaar QR Code
- Uses JPEG photos, ~500-900 bytes
- RSA-2048 signature (256 bytes) leaves less room for photo than Ed25519
- UIDAI acknowledges the photo is "low resolution" and "may not be sufficient to recognize the person"

### EU Digital COVID Certificate (DCC)
- Uses the same pipeline (CBOR -> CWT -> COSE -> zlib -> Base45 -> QR)
- **Does not include a photo** — the architects chose to avoid it due to QR size constraints
- Identity verification relies on a separate ID document

### Mobile Driver's License (ISO 18013-5)
- **Does not put the photo in the QR code**
- QR code is used only for device engagement (bootstrapping BLE/NFC/WiFi)
- Photo is transferred over the higher-bandwidth channel
- Minimum photo resolution: 192x240px (impossible to fit in a QR code)

### MOSIP Claim 169
- Targets 1,005 bytes total payload
- Uses Ed25519 (64-byte signature vs Aadhaar's 256-byte RSA)
- Photo format is implementer's choice; WebP recommended for size efficiency

## Quick Reference

For implementers who want the recommended defaults:

```
Format:     WebP (code 4), JPEG fallback (code 1)
Resolution: 48x48 pixels
Quality:    60%
Crop:       Top-center, square
Blur:       0.3px Gaussian before encoding
Color:      Keep color (do not convert to grayscale)
Target:     400-700 bytes
```

## Capacity Monitoring

Monitor the total Base45 string length after encoding. Thresholds for QR scannability:

| Base45 Length | Status | Action |
|--------------|--------|--------|
| < 1,800 chars | Safe | No action needed |
| 1,800-2,100 chars | Warning | Consider reducing photo size or removing optional fields |
| > 2,100 chars | Critical | QR code may not scan reliably at Level M |

These thresholds assume QR Version 40, Error Correction Level M. Lower QR versions have proportionally lower limits.
