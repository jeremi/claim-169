import { useState } from "react"
import { Decoder, type DecodeResult } from "claim169"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { QrScanner } from "@/components/QrScanner"
import { ResultViewer } from "@/components/ResultViewer"
import { Camera, Play, Loader2 } from "lucide-react"
import { hexToBytes } from "@/lib/utils"

type VerificationMethod = "ed25519" | "ecdsa" | "skip"
type DecryptionMethod = "none" | "aes256" | "aes128"

export function DecodePanel() {
  const [qrData, setQrData] = useState("")
  const [verificationMethod, setVerificationMethod] = useState<VerificationMethod>("skip")
  const [publicKey, setPublicKey] = useState("")
  const [decryptionMethod, setDecryptionMethod] = useState<DecryptionMethod>("none")
  const [decryptionKey, setDecryptionKey] = useState("")
  const [showScanner, setShowScanner] = useState(false)
  const [isDecoding, setIsDecoding] = useState(false)
  const [result, setResult] = useState<DecodeResult | null>(null)
  const [error, setError] = useState<string | null>(null)

  const handleDecode = async () => {
    if (!qrData.trim()) {
      setError("Please enter QR data")
      return
    }

    setIsDecoding(true)
    setError(null)
    setResult(null)

    try {
      let decoder = new Decoder(qrData.trim())

      // Apply decryption if selected
      if (decryptionMethod !== "none") {
        if (!decryptionKey.trim()) {
          throw new Error("Decryption key is required")
        }
        const keyBytes = hexToBytes(decryptionKey.trim())
        if (decryptionMethod === "aes256") {
          if (keyBytes.length !== 32) {
            throw new Error("AES-256 key must be 32 bytes (64 hex characters)")
          }
          decoder = decoder.decryptWithAes256(keyBytes)
        } else {
          if (keyBytes.length !== 16) {
            throw new Error("AES-128 key must be 16 bytes (32 hex characters)")
          }
          decoder = decoder.decryptWithAes128(keyBytes)
        }
      }

      // Apply verification
      if (verificationMethod === "ed25519") {
        if (!publicKey.trim()) {
          throw new Error("Public key is required for Ed25519 verification")
        }
        const keyBytes = hexToBytes(publicKey.trim())
        if (keyBytes.length !== 32) {
          throw new Error("Ed25519 public key must be 32 bytes (64 hex characters)")
        }
        decoder = decoder.verifyWithEd25519(keyBytes)
      } else if (verificationMethod === "ecdsa") {
        if (!publicKey.trim()) {
          throw new Error("Public key is required for ECDSA verification")
        }
        const keyBytes = hexToBytes(publicKey.trim())
        if (keyBytes.length !== 33 && keyBytes.length !== 65) {
          throw new Error("ECDSA P-256 public key must be 33 bytes (compressed) or 65 bytes (uncompressed)")
        }
        decoder = decoder.verifyWithEcdsaP256(keyBytes)
      } else {
        decoder = decoder.allowUnverified()
      }

      const decoded = decoder.decode()
      setResult(decoded)
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err))
    } finally {
      setIsDecoding(false)
    }
  }

  const handleScan = (data: string) => {
    setQrData(data)
    setShowScanner(false)
  }

  return (
    <div className="grid lg:grid-cols-2 gap-6">
      <div className="space-y-4">
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg">QR Data Input</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Textarea
                placeholder="Paste Base45-encoded QR data here (e.g., 6BF...)"
                value={qrData}
                onChange={(e) => setQrData(e.target.value)}
                className="font-mono text-sm min-h-[120px]"
              />
              <Button
                variant="outline"
                onClick={() => setShowScanner(true)}
                className="w-full"
              >
                <Camera className="h-4 w-4 mr-2" />
                Scan QR Code
              </Button>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg">Verification</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex flex-wrap gap-4">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="verification"
                  checked={verificationMethod === "ed25519"}
                  onChange={() => setVerificationMethod("ed25519")}
                  className="w-4 h-4"
                />
                <span className="text-sm">Ed25519</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="verification"
                  checked={verificationMethod === "ecdsa"}
                  onChange={() => setVerificationMethod("ecdsa")}
                  className="w-4 h-4"
                />
                <span className="text-sm">ECDSA P-256</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="verification"
                  checked={verificationMethod === "skip"}
                  onChange={() => setVerificationMethod("skip")}
                  className="w-4 h-4"
                />
                <span className="text-sm text-muted-foreground">Skip (testing only)</span>
              </label>
            </div>

            {verificationMethod !== "skip" && (
              <div className="space-y-2">
                <Label htmlFor="publicKey">Public Key (hex)</Label>
                <Input
                  id="publicKey"
                  placeholder={verificationMethod === "ed25519" ? "32 bytes (64 hex chars)" : "33 or 65 bytes"}
                  value={publicKey}
                  onChange={(e) => setPublicKey(e.target.value)}
                  className="font-mono text-sm"
                />
              </div>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg">Decryption (Optional)</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex flex-wrap gap-4">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="decryption"
                  checked={decryptionMethod === "none"}
                  onChange={() => setDecryptionMethod("none")}
                  className="w-4 h-4"
                />
                <span className="text-sm">None</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="decryption"
                  checked={decryptionMethod === "aes256"}
                  onChange={() => setDecryptionMethod("aes256")}
                  className="w-4 h-4"
                />
                <span className="text-sm">AES-256-GCM</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="decryption"
                  checked={decryptionMethod === "aes128"}
                  onChange={() => setDecryptionMethod("aes128")}
                  className="w-4 h-4"
                />
                <span className="text-sm">AES-128-GCM</span>
              </label>
            </div>

            {decryptionMethod !== "none" && (
              <div className="space-y-2">
                <Label htmlFor="decryptionKey">Decryption Key (hex)</Label>
                <Input
                  id="decryptionKey"
                  placeholder={decryptionMethod === "aes256" ? "32 bytes (64 hex chars)" : "16 bytes (32 hex chars)"}
                  value={decryptionKey}
                  onChange={(e) => setDecryptionKey(e.target.value)}
                  className="font-mono text-sm"
                />
              </div>
            )}
          </CardContent>
        </Card>

        <Button onClick={handleDecode} disabled={isDecoding} className="w-full">
          {isDecoding ? (
            <Loader2 className="h-4 w-4 mr-2 animate-spin" />
          ) : (
            <Play className="h-4 w-4 mr-2" />
          )}
          Decode
        </Button>
      </div>

      <div>
        <ResultViewer result={result} error={error} />
      </div>

      {showScanner && (
        <QrScanner onScan={handleScan} onClose={() => setShowScanner(false)} />
      )}
    </div>
  )
}
