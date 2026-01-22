import { useState } from "react"
import { Decoder, type DecodeResult } from "claim169"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Select } from "@/components/ui/select"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { QrScanner } from "@/components/QrScanner"
import { ResultViewer } from "@/components/ResultViewer"
import { Camera, Play, Loader2, FileText } from "lucide-react"
import { hexToBytes } from "@/lib/utils"

type VerificationMethod = "ed25519" | "ecdsa" | "skip"
type DecryptionMethod = "none" | "aes256" | "aes128"

// Pre-made examples from test vectors
const EXAMPLES = {
  "": {
    name: "Select an example...",
    qrData: "",
    verification: "skip" as VerificationMethod,
    publicKey: "",
    decryption: "none" as DecryptionMethod,
    decryptionKey: "",
  },
  "ed25519-signed": {
    name: "Ed25519 Signed (simple)",
    qrData: "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4",
    verification: "ed25519" as VerificationMethod,
    publicKey: "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    decryption: "none" as DecryptionMethod,
    decryptionKey: "",
  },
  "demographics-full": {
    name: "Full Demographics (all fields)",
    qrData: "6BFA$BMVPYUOM43QVJH.AB27/RBNIDSMGU*4WKQYPDP-MBVDN66$EQM0JBF6YRC/S9N%8T2M.52+11A9G+PQELUPWJ0$RK2GW0PTZJ$6EC/7Z$Q718XV1WGD$DE0%CBBS0I7YEPUMD1F4OPQ3LD:T50KNJ6DKSVG%63EUHK2LJB4BQH 8UMQI5S7-L24WFDVB2N*OQRRMLF15U7X9L1AGVFU0WCJC0.CCW/DB*C*YKTZM9ULPW5U0ND94X9F/07UK2K*EC%2$I0BH3YUA*4MRQQIHS9WDB813.3B11%RRAFFM23N263SG91CDMIWFEI40$43AAII9UNAN6 P01E$M7A*VSTCUL1LILRV2 4BSL5HWPWGSEQOGHPZBV2-P6902HFVZTBJ37-8YF9Y/8JCA9O3XA0:VES4",
    verification: "skip" as VerificationMethod,
    publicKey: "",
    decryption: "none" as DecryptionMethod,
    decryptionKey: "",
  },
  "encrypted-signed": {
    name: "Encrypted + Signed",
    qrData: "6BFCA0410DFWXQG.FKTK06U0 DKAPKT K33LMEL-PLV9BIQFGWFU2G8 N.BGZF1VSIYEV7Q3FLRS3AJNDF.3Z0N+ZE:MD1D8R22LF3WC9*9O-P9VYJ3NGZ%3L9BTN0/5LOP0X2RL0ERSVBHVN EUZUQ2C$Y05CLKYU+9653893EU%2ICEALJ864MHB:QK 2SSTKGY6EPJFFS:R0OBF7O57NDH$12PBS1GOQGKY2BDW0Z02NPQO0SPGZ1REG0GPGV6OCRN0RI$IN751T%2SO2TFI2DSV431V5TYUW5H9 9QJU.JN7M6$95:1FJKF9P84DRF/P:0",
    verification: "ed25519" as VerificationMethod,
    publicKey: "994c54604862f73d4bce14120b318f720119c6498b59257fb89cbead939ba0f5",
    decryption: "aes256" as DecryptionMethod,
    decryptionKey: "101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
  },
}

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
  const [selectedExample, setSelectedExample] = useState("")

  const loadExample = (exampleKey: string) => {
    setSelectedExample(exampleKey)
    if (exampleKey && EXAMPLES[exampleKey as keyof typeof EXAMPLES]) {
      const example = EXAMPLES[exampleKey as keyof typeof EXAMPLES]
      setQrData(example.qrData)
      setVerificationMethod(example.verification)
      setPublicKey(example.publicKey)
      setDecryptionMethod(example.decryption)
      setDecryptionKey(example.decryptionKey)
      setResult(null)
      setError(null)
    }
  }

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
            <div className="flex items-center justify-between">
              <CardTitle className="text-lg">QR Data Input</CardTitle>
              <div className="flex items-center gap-2">
                <FileText className="h-4 w-4 text-muted-foreground" />
                <Select
                  value={selectedExample}
                  onChange={(e) => loadExample(e.target.value)}
                  className="w-48 text-sm"
                >
                  {Object.entries(EXAMPLES).map(([key, example]) => (
                    <option key={key} value={key}>
                      {example.name}
                    </option>
                  ))}
                </Select>
              </div>
            </div>
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
