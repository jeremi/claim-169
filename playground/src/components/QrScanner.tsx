import { useEffect, useRef, useState } from "react"
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
  const [error, setError] = useState<string | null>(null)
  const [isScanning, setIsScanning] = useState(false)
  const scannerRef = useRef<Html5Qrcode | null>(null)
  const containerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const startScanner = async () => {
      if (!containerRef.current) return

      try {
        const scanner = new Html5Qrcode("qr-reader")
        scannerRef.current = scanner

        const cameras = await Html5Qrcode.getCameras()
        if (cameras.length === 0) {
          setError("No cameras found. Try uploading an image instead.")
          return
        }

        setIsScanning(true)
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
      } catch (err) {
        setError(`Camera access failed: ${err instanceof Error ? err.message : String(err)}`)
        setIsScanning(false)
      }
    }

    startScanner()

    return () => {
      stopScanner()
    }
  }, [])

  const stopScanner = async () => {
    if (scannerRef.current && isScanning) {
      try {
        await scannerRef.current.stop()
        scannerRef.current.clear()
      } catch {
        // Ignore stop errors
      }
    }
    setIsScanning(false)
  }

  const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (!file) return

    try {
      await stopScanner()
      const scanner = new Html5Qrcode("qr-reader")
      const result = await scanner.scanFile(file, true)
      onScan(result)
      scanner.clear()
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
            Scan QR Code
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
            Upload Image
          </label>
          <Button variant="secondary" onClick={onClose}>
            Cancel
          </Button>
        </div>
      </div>
    </div>
  )
}
