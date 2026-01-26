import { useState } from "react"
import { useTranslation } from "react-i18next"
import { QRCodeSVG } from "qrcode.react"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Label } from "@/components/ui/label"
import { VerificationBadge, type VerificationStatus } from "@/components/VerificationBadge"
import { PipelineDetails, type PipelineStage } from "@/components/PipelineDetails"
import { ErrorDisplay } from "@/components/ErrorDisplay"
import { Camera, Copy, Check, Download, ChevronDown, ChevronRight } from "lucide-react"
import { copyToClipboard, cn } from "@/lib/utils"
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

    const svgData = new XMLSerializer().serializeToString(svg)
    const canvas = document.createElement("canvas")
    const ctx = canvas.getContext("2d")
    const img = new Image()

    img.onload = () => {
      canvas.width = img.width
      canvas.height = img.height
      ctx?.drawImage(img, 0, 0)
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

      {/* Base45 Data */}
      {base45Data && (
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <Label htmlFor="base45">{t("encoded.base45Label")}</Label>
            <div className="flex items-center gap-2">
              <span className="text-xs text-muted-foreground">
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
              {JSON.stringify({ claim169, cwtMeta }, null, 2)}
            </pre>
          )}
        </>
      )}
    </div>
  )
}
