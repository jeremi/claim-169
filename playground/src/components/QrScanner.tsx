import { useEffect, useRef, useState } from "react"
import { useTranslation } from "react-i18next"
import { Html5Qrcode } from "html5-qrcode"
import { Button } from "@/components/ui/button"
import { buttonVariants } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import { Camera, X, Upload } from "lucide-react"

interface QrScannerProps {
  onScan: (data: string) => void
  onClose: () => void
}

export function QrScanner({ onScan, onClose }: QrScannerProps) {
  const { t } = useTranslation()
  const [error, setError] = useState<string | null>(null)
  const scannerRef = useRef<Html5Qrcode | null>(null)
  const containerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    let cancelled = false

    const startScanner = async () => {
      if (!containerRef.current) return

      try {
        const scanner = new Html5Qrcode("qr-reader")
        scannerRef.current = scanner

        const cameras = await Html5Qrcode.getCameras()
        if (cancelled) {
          scanner.clear()
          return
        }

        if (cameras.length === 0) {
          setError(t("scanner.noCameras"))
          return
        }

        await scanner.start(
          { facingMode: "environment" },
          {
            fps: 10,
            qrbox: { width: 250, height: 250 },
          },
          (decodedText) => {
            onScan(decodedText)
            stopScanner()
          },
          () => {
            // Ignore scan failures
          }
        )

        // If cleanup ran while start() was in progress, stop immediately
        if (cancelled) {
          try {
            await scanner.stop()
            scanner.clear()
          } catch {
            // ignore
          }
        }
      } catch (err) {
        if (!cancelled) {
          setError(`${t("scanner.cameraFailed")}: ${err instanceof Error ? err.message : String(err)}`)
        }
      }
    }

    startScanner()

    return () => {
      cancelled = true
      stopScanner()
    }
  }, [])

  const stopScanner = async () => {
    if (scannerRef.current) {
      try {
        await scannerRef.current.stop()
        scannerRef.current.clear()
      } catch {
        // May already be stopped or not yet started
      }
      scannerRef.current = null
    }

    // Fallback: manually stop any video tracks the library left behind
    const allVideos = document.querySelectorAll("video")
    allVideos.forEach((v) => {
      if (v.srcObject instanceof MediaStream) {
        v.srcObject.getTracks().forEach(track => track.stop())
        v.srcObject = null
      }
    })
  }

  const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (!file) return

    try {
      await stopScanner()
      const scanner = new Html5Qrcode("qr-reader")
      const result = await scanner.scanFile(file, true)
      scanner.clear()
      onScan(result)
    } catch (err) {
      setError(`Failed to scan image: ${err instanceof Error ? err.message : String(err)}`)
    }
  }

  return (
    <div className="fixed inset-0 z-50 bg-black/80 flex items-center justify-center p-4">
      <div className="bg-background rounded-lg max-w-md w-full p-4">
        <div className="flex items-center justify-between mb-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Camera className="h-5 w-5" />
            {t("scanner.title")}
          </h3>
          <Button variant="ghost" size="icon" onClick={onClose}>
            <X className="h-4 w-4" />
          </Button>
        </div>

        {error && (
          <div className="bg-destructive/10 text-destructive text-sm p-3 rounded-md mb-4">
            {error}
          </div>
        )}

        <div
          ref={containerRef}
          id="qr-reader"
          className="w-full rounded-lg overflow-hidden bg-muted mb-4"
          style={{ minHeight: 300 }}
        />

        <div className="flex flex-col gap-2">
          <label className={cn(buttonVariants({ variant: "outline" }), "w-full cursor-pointer")}>
            <input
              type="file"
              accept="image/*"
              onChange={handleFileUpload}
              className="hidden"
            />
            <Upload className="h-4 w-4 mr-2" />
            {t("scanner.uploadImage")}
          </label>
          <Button variant="secondary" onClick={onClose}>
            {t("scanner.cancel")}
          </Button>
        </div>
      </div>
    </div>
  )
}
