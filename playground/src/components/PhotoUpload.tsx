import { useState, useRef, useEffect, useCallback } from "react"
import { useTranslation } from "react-i18next"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { compressPhoto, createPhotoPreviewUrl } from "@/lib/image"
import { PHOTO_FORMAT_MAP, cn } from "@/lib/utils"
import { ImagePlus, X, ChevronDown, ChevronRight } from "lucide-react"

interface PhotoUploadProps {
  photo: Uint8Array | undefined
  photoFormat: number | undefined
  onPhotoChange: (photo: Uint8Array | undefined, format: number | undefined) => void
  encodedSize?: number
}

export function PhotoUpload({
  photo,
  photoFormat,
  onPhotoChange,
  encodedSize,
}: PhotoUploadProps) {
  const { t } = useTranslation()
  const [resolution, setResolution] = useState(48)
  const [quality, setQuality] = useState(60)
  const [showSettings, setShowSettings] = useState(false)
  const [isDragOver, setIsDragOver] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [previewUrl, setPreviewUrl] = useState<string | null>(null)
  const [showLightbox, setShowLightbox] = useState(false)

  const originalFileRef = useRef<File | null>(null)
  const fileInputRef = useRef<HTMLInputElement>(null)
  const debounceTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null)
  const compressionVersionRef = useRef(0)

  // Manage preview URL lifecycle — create and revoke in the same effect
  useEffect(() => {
    if (!photo || photo.length === 0) {
      setPreviewUrl(null)
      return
    }
    const url = createPhotoPreviewUrl(photo, photoFormat)
    setPreviewUrl(url)
    return () => URL.revokeObjectURL(url)
  }, [photo, photoFormat])

  // Clean up debounce timer on unmount
  useEffect(() => {
    return () => {
      if (debounceTimerRef.current) {
        clearTimeout(debounceTimerRef.current)
      }
    }
  }, [])

  const handleFile = useCallback(async (file: File) => {
    setError(null)
    originalFileRef.current = file
    const version = ++compressionVersionRef.current
    try {
      const result = await compressPhoto(file, {
        maxDimension: resolution,
        quality: quality / 100,
      })
      if (compressionVersionRef.current !== version) return
      onPhotoChange(result.data, result.format)
    } catch (err) {
      if (compressionVersionRef.current !== version) return
      const message = err instanceof Error ? err.message : String(err)
      if (message.includes("too large")) {
        setError(t("photo.tooLarge"))
      } else {
        setError(t("photo.invalidFile"))
      }
    }
  }, [resolution, quality, onPhotoChange, t])

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (file) handleFile(file)
    // Reset input value so re-uploading the same file works
    e.target.value = ""
  }

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragOver(false)
    const file = e.dataTransfer.files?.[0]
    if (file && file.type.startsWith("image/")) {
      handleFile(file)
    } else if (file) {
      setError(t("photo.invalidFile"))
    }
  }

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragOver(true)
  }

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragOver(false)
  }

  const handleRemove = () => {
    originalFileRef.current = null
    compressionVersionRef.current++
    onPhotoChange(undefined, undefined)
    setError(null)
  }

  // Re-compress when sliders change (debounced)
  const recompress = useCallback(async (dim: number, qual: number) => {
    if (!originalFileRef.current) return
    const version = ++compressionVersionRef.current
    try {
      const result = await compressPhoto(originalFileRef.current, {
        maxDimension: dim,
        quality: qual / 100,
      })
      if (compressionVersionRef.current !== version) return
      onPhotoChange(result.data, result.format)
    } catch {
      // Original file ref is stale or invalid — ignore
    }
  }, [onPhotoChange])

  // Both sliders share a single debounce timer — the last slider change within
  // 150ms wins, and the fired callback receives explicit parameter values
  // (not stale closures), so the result is always correct.
  const handleResolutionChange = (value: number) => {
    setResolution(value)
    if (debounceTimerRef.current) clearTimeout(debounceTimerRef.current)
    debounceTimerRef.current = setTimeout(() => recompress(value, quality), 150)
  }

  const handleQualityChange = (value: number) => {
    setQuality(value)
    if (debounceTimerRef.current) clearTimeout(debounceTimerRef.current)
    debounceTimerRef.current = setTimeout(() => recompress(resolution, value), 150)
  }

  const hasOriginalFile = originalFileRef.current !== null
  const formatLabel = photoFormat ? PHOTO_FORMAT_MAP[photoFormat] : undefined
  const sizeBytes = photo?.length ?? 0

  // Capacity color based on total Base45 string length
  const capacityColor = !encodedSize
    ? "text-muted-foreground"
    : encodedSize > 2100
      ? "text-red-600 dark:text-red-400"
      : encodedSize > 1800
        ? "text-amber-600 dark:text-amber-400"
        : "text-green-600 dark:text-green-400"

  return (
    <div className="space-y-2">
      <Label className="text-xs font-medium">{t("photo.title")}</Label>

      {!photo || photo.length === 0 ? (
        // Drop zone / file picker
        <div
          onDrop={handleDrop}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onClick={() => fileInputRef.current?.click()}
          className={cn(
            "flex items-center justify-center gap-2 p-4 rounded-lg border-2 border-dashed cursor-pointer transition-colors",
            isDragOver
              ? "border-green-500 bg-green-50 dark:bg-green-950/20"
              : "border-muted-foreground/30 hover:border-muted-foreground/50",
          )}
        >
          <ImagePlus className="h-5 w-5 text-muted-foreground" />
          <span className="text-sm text-muted-foreground">
            {isDragOver ? t("photo.dropzoneActive") : t("photo.dropzone")}
          </span>
          <input
            ref={fileInputRef}
            type="file"
            accept="image/*"
            onChange={handleInputChange}
            className="hidden"
          />
        </div>
      ) : (
        // Photo loaded — preview + controls
        <div className="flex items-start gap-3 p-3 rounded-lg border border-green-200 bg-green-50/50 dark:border-green-900 dark:bg-green-950/20">
          {/* Thumbnail preview — click to enlarge */}
          {previewUrl && (
            <button
              type="button"
              onClick={() => setShowLightbox(true)}
              className="shrink-0 cursor-zoom-in"
            >
              <img
                src={previewUrl}
                alt={t("photo.compressedPreviewAlt")}
                className="w-12 h-12 rounded border border-green-300 dark:border-green-700"
                style={{ imageRendering: "pixelated" }}
              />
            </button>
          )}

          <div className="flex-1 min-w-0 space-y-1">
            {/* Size + format info */}
            <div className="flex items-center gap-2 text-xs">
              <span className={capacityColor}>
                ~{sizeBytes} {t("photo.bytes")}
              </span>
              {formatLabel && (
                <span className="inline-flex items-center px-1.5 py-0.5 rounded bg-green-100 text-green-800 dark:bg-green-900/50 dark:text-green-300 text-[10px] font-medium">
                  {formatLabel}
                </span>
              )}
            </div>

            {/* Compression settings toggle */}
            <Button
              variant="ghost"
              size="sm"
              className="h-6 px-0 text-xs text-muted-foreground hover:text-foreground"
              onClick={() => setShowSettings(!showSettings)}
            >
              {showSettings ? <ChevronDown className="h-3 w-3 mr-1" /> : <ChevronRight className="h-3 w-3 mr-1" />}
              {t("photo.compressionSettings")}
            </Button>

            {showSettings && (
              <div className="space-y-2 pt-1">
                {hasOriginalFile ? (
                  <>
                    {/* Resolution slider */}
                    <div className="space-y-1">
                      <div className="flex items-center justify-between">
                        <Label className="text-[10px]">{t("photo.resolution")}</Label>
                        <span className="text-[10px] text-muted-foreground">{resolution}px</span>
                      </div>
                      <input
                        type="range"
                        min={16}
                        max={96}
                        step={8}
                        value={resolution}
                        onChange={(e) => handleResolutionChange(Number(e.target.value))}
                        className="w-full h-1 accent-green-600"
                      />
                    </div>

                    {/* Quality slider */}
                    <div className="space-y-1">
                      <div className="flex items-center justify-between">
                        <Label className="text-[10px]">{t("photo.quality")}</Label>
                        <span className="text-[10px] text-muted-foreground">{quality}%</span>
                      </div>
                      <input
                        type="range"
                        min={10}
                        max={100}
                        step={5}
                        value={quality}
                        onChange={(e) => handleQualityChange(Number(e.target.value))}
                        className="w-full h-1 accent-green-600"
                      />
                    </div>
                  </>
                ) : (
                  <p className="text-[10px] text-muted-foreground italic">
                    {t("photo.slidersDisabled")}
                  </p>
                )}
              </div>
            )}
          </div>

          {/* Remove button */}
          <Button
            variant="ghost"
            size="sm"
            className="h-6 w-6 p-0 text-muted-foreground hover:text-red-600"
            onClick={handleRemove}
            title={t("photo.remove")}
          >
            <X className="h-4 w-4" />
          </Button>
        </div>
      )}

      {error && (
        <p className="text-xs text-red-600 dark:text-red-400">{error}</p>
      )}

      {/* Lightbox overlay */}
      {showLightbox && previewUrl && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 cursor-zoom-out"
          onClick={() => setShowLightbox(false)}
        >
          <img
            src={previewUrl}
            alt={t("photo.compressedPreviewAlt")}
            className="max-w-[80vw] max-h-[80vh] rounded-lg border-2 border-white/20"
            style={{ imageRendering: "pixelated" }}
          />
        </div>
      )}
    </div>
  )
}
