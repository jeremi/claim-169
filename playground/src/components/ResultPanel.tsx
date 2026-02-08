import { useState, useEffect } from "react"
import { useTranslation } from "react-i18next"
import { QRCodeSVG } from "qrcode.react"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Label } from "@/components/ui/label"
import { VerificationBadge, type VerificationStatus } from "@/components/VerificationBadge"
import { PipelineDetails, type PipelineStage } from "@/components/PipelineDetails"
import { ErrorDisplay } from "@/components/ErrorDisplay"
import { Camera, Copy, Check, Download, ChevronDown, ChevronRight, AlertTriangle } from "lucide-react"
import { copyToClipboard, cn, PHOTO_FORMAT_MAP } from "@/lib/utils"
import { createPhotoPreviewUrl } from "@/lib/image"
import type { ParsedError } from "@/lib/errors"

interface ResultPanelProps {
  base45Data: string
  onBase45Change: (value: string) => void
  verificationStatus: VerificationStatus
  algorithm?: string
  pipelineStages: PipelineStage[]
  onScanClick: () => void
  isProcessing: boolean
  error: string | null
  parsedError?: ParsedError | null
  claim169: Record<string, unknown>
  cwtMeta: Record<string, unknown>
}

export function ResultPanel({
  base45Data,
  onBase45Change,
  verificationStatus,
  algorithm,
  pipelineStages,
  onScanClick,
  isProcessing,
  error,
  parsedError,
  claim169,
  cwtMeta,
}: ResultPanelProps) {
  const { t } = useTranslation()
  const [copied, setCopied] = useState(false)
  const [showRawJson, setShowRawJson] = useState(false)
  const [decodedPhotoUrl, setDecodedPhotoUrl] = useState<string | null>(null)
  const [showPhotoLightbox, setShowPhotoLightbox] = useState(false)

  // Build photo preview URL from decoded data
  const decodedPhoto = claim169.photo as Uint8Array | undefined
  const decodedPhotoFormat = claim169.photoFormat as number | undefined

  useEffect(() => {
    if (!decodedPhoto || !(decodedPhoto instanceof Uint8Array) || decodedPhoto.length === 0) {
      setDecodedPhotoUrl(null)
      return
    }
    const url = createPhotoPreviewUrl(decodedPhoto, decodedPhotoFormat)
    setDecodedPhotoUrl(url)
    return () => URL.revokeObjectURL(url)
  }, [decodedPhoto, decodedPhotoFormat])

  // JSON replacer to show byte arrays compactly
  const jsonReplacer = (_key: string, value: unknown) => {
    if (value instanceof Uint8Array) {
      return `<${value.length} bytes>`
    }
    return value
  }

  const handleCopy = async () => {
    if (base45Data) {
      await copyToClipboard(base45Data)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    }
  }

  const handleDownloadQr = () => {
    if (!base45Data) return
    const svg = document.getElementById("qr-code-svg")
    if (!svg) return

    // Export at high resolution with a quiet zone so QR scanners can read the code.
    // The quiet zone (white border) is required by the QR spec — without it, many
    // scanners fail to detect the finder patterns.
    const exportSize = 1024
    const margin = 48
    const qrSize = exportSize - margin * 2

    const svgClone = svg.cloneNode(true) as SVGElement
    svgClone.setAttribute("width", String(qrSize))
    svgClone.setAttribute("height", String(qrSize))
    const svgData = new XMLSerializer().serializeToString(svgClone)

    const canvas = document.createElement("canvas")
    canvas.width = exportSize
    canvas.height = exportSize
    const ctx = canvas.getContext("2d")
    if (!ctx) return

    // White background provides the quiet zone
    ctx.fillStyle = "#ffffff"
    ctx.fillRect(0, 0, exportSize, exportSize)

    // Disable smoothing so QR modules stay crisp (no anti-aliasing blur)
    ctx.imageSmoothingEnabled = false

    const img = new Image()
    img.onload = () => {
      ctx.drawImage(img, margin, margin, qrSize, qrSize)
      const pngUrl = canvas.toDataURL("image/png")
      const link = document.createElement("a")
      link.download = "claim169-qr.png"
      link.href = pngUrl
      link.click()
    }

    img.src = "data:image/svg+xml;base64," + btoa(svgData)
  }

  return (
    <div className="space-y-4">
      {/* Header */}
      <h2 className="text-lg font-semibold">{t("result.title")}</h2>

      {/* QR Code - Large and prominent */}
      <div className={cn(
        "flex flex-col items-center gap-4 p-6 rounded-lg border-2 transition-colors",
        base45Data
          ? "bg-white dark:bg-gray-950 border-gray-200 dark:border-gray-800"
          : "bg-muted/30 border-dashed border-muted-foreground/30"
      )}>
        {base45Data ? (
          <>
            <QRCodeSVG
              id="qr-code-svg"
              value={base45Data}
              size={220}
              level="M"
              includeMargin
              className="rounded"
            />
            <div className="flex gap-2">
              <Button variant="outline" size="sm" onClick={handleDownloadQr}>
                <Download className="h-4 w-4 mr-2" />
                {t("encoded.downloadPng")}
              </Button>
              <Button variant="outline" size="sm" onClick={onScanClick}>
                <Camera className="h-4 w-4 mr-2" />
                {t("encoded.scan")}
              </Button>
            </div>
          </>
        ) : (
          <div className="text-center py-8">
            <div className="w-32 h-32 mx-auto mb-4 rounded-lg border-2 border-dashed border-muted-foreground/30 flex items-center justify-center">
              <Camera className="h-12 w-12 text-muted-foreground/50" />
            </div>
            <p className="text-sm text-muted-foreground mb-4">{t("result.noQrYet")}</p>
            <Button variant="outline" onClick={onScanClick}>
              <Camera className="h-4 w-4 mr-2" />
              {t("encoded.scanQr")}
            </Button>
          </div>
        )}
      </div>

      {/* Verification Badge - Prominent */}
      {base45Data && (
        <VerificationBadge
          status={verificationStatus}
          algorithm={algorithm}
          error={error || undefined}
        />
      )}

      {/* Decoded Photo Preview */}
      {base45Data && decodedPhotoUrl && decodedPhoto && (
        <div className="flex items-center gap-3 p-3 rounded-lg border bg-muted/30">
          <button
            type="button"
            onClick={() => setShowPhotoLightbox(true)}
            className="shrink-0 cursor-zoom-in"
          >
            <img
              src={decodedPhotoUrl}
              alt={t("photo.decodedPhotoAlt")}
              className="w-12 h-12 rounded border"
              style={{ imageRendering: "pixelated" }}
            />
          </button>
          <div className="text-xs text-muted-foreground">
            <span>{decodedPhoto.length} {t("photo.bytes")}</span>
            {decodedPhotoFormat && PHOTO_FORMAT_MAP[decodedPhotoFormat] && (
              <span className="ml-2 inline-flex items-center px-1.5 py-0.5 rounded bg-muted text-[10px] font-medium">
                {PHOTO_FORMAT_MAP[decodedPhotoFormat]}
              </span>
            )}
          </div>
        </div>
      )}

      {/* Capacity Warnings */}
      {base45Data && base45Data.length > 2100 && (
        <div className="flex items-start gap-2 p-2 rounded bg-red-100 dark:bg-red-900/30 text-red-800 dark:text-red-200 text-xs">
          <AlertTriangle className="h-4 w-4 shrink-0 mt-0.5" />
          <span>{t("result.qrTooLarge")}</span>
        </div>
      )}
      {base45Data && base45Data.length > 1800 && base45Data.length <= 2100 && (
        <div className="flex items-start gap-2 p-2 rounded bg-amber-100 dark:bg-amber-900/30 text-amber-800 dark:text-amber-200 text-xs">
          <AlertTriangle className="h-4 w-4 shrink-0 mt-0.5" />
          <span>{t("result.qrNearLimit")}</span>
        </div>
      )}

      {/* Base45 Data */}
      {base45Data && (
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <Label htmlFor="base45">{t("encoded.base45Label")}</Label>
            <div className="flex items-center gap-2">
              <span className={cn(
                "text-xs",
                base45Data.length > 2100
                  ? "text-red-600 dark:text-red-400 font-medium"
                  : base45Data.length > 1800
                    ? "text-amber-600 dark:text-amber-400"
                    : "text-muted-foreground"
              )}>
                {base45Data.length} {t("encoded.chars")}
              </span>
              <Button variant="ghost" size="sm" onClick={handleCopy} className="h-6 px-2">
                {copied ? <Check className="h-3 w-3" /> : <Copy className="h-3 w-3" />}
              </Button>
            </div>
          </div>
          <Textarea
            id="base45"
            placeholder={t("encoded.base45Placeholder")}
            value={base45Data}
            onChange={(e) => onBase45Change(e.target.value)}
            className="font-mono text-xs min-h-[80px] resize-none"
          />
        </div>
      )}

      {/* Error Display */}
      {parsedError ? (
        <ErrorDisplay error={parsedError} />
      ) : error && (
        <div className="p-3 rounded-lg bg-red-50 dark:bg-red-950/30 border border-red-200 dark:border-red-900">
          <p className="text-sm text-red-600 dark:text-red-400">{error}</p>
        </div>
      )}

      {/* Processing Indicator */}
      {isProcessing && (
        <div className="text-center text-sm text-muted-foreground">
          {t("common.processing")}
        </div>
      )}

      {/* Pipeline Details */}
      {pipelineStages.length > 0 && (
        <PipelineDetails stages={pipelineStages} />
      )}

      {/* Raw JSON Toggle */}
      {base45Data && (
        <>
          <Button
            variant="ghost"
            className="w-full justify-between"
            onClick={() => setShowRawJson(!showRawJson)}
          >
            <span>{t("decoded.rawJson")}</span>
            {showRawJson ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
          </Button>

          {showRawJson && (
            <pre className="p-4 bg-muted rounded-lg text-xs font-mono overflow-auto max-h-64">
              {JSON.stringify({ claim169, cwtMeta }, jsonReplacer, 2)}
            </pre>
          )}
        </>
      )}

      {/* Photo lightbox overlay */}
      {showPhotoLightbox && decodedPhotoUrl && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 cursor-zoom-out"
          onClick={() => setShowPhotoLightbox(false)}
        >
          <img
            src={decodedPhotoUrl}
            alt={t("photo.decodedPhotoAlt")}
            className="max-w-[80vw] max-h-[80vh] rounded-lg border-2 border-white/20"
            style={{ imageRendering: "pixelated" }}
          />
        </div>
      )}
    </div>
  )
}
