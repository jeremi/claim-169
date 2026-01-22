import { useState } from "react"
import { Encoder, type Claim169Input, type CwtMetaInput } from "claim169"
import { QRCodeSVG } from "qrcode.react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Select } from "@/components/ui/select"
import { Textarea } from "@/components/ui/textarea"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Play, Loader2, Copy, Check, Download, Sparkles } from "lucide-react"
import { hexToBytes, copyToClipboard } from "@/lib/utils"

// Sample test keys (from test vectors - for testing only!)
const SAMPLE_ED25519_PRIVATE_KEY = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
const SAMPLE_ED25519_PUBLIC_KEY = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"

type SigningMethod = "ed25519" | "ecdsa" | "unsigned"
type EncryptionMethod = "none" | "aes256" | "aes128"

export function EncodePanel() {
  const [claim169, setClaim169] = useState<Claim169Input>({
    id: "",
    fullName: "",
  })
  const [cwtMeta, setCwtMeta] = useState<CwtMetaInput>({
    issuer: "",
  })
  const [signingMethod, setSigningMethod] = useState<SigningMethod>("ed25519")
  const [privateKey, setPrivateKey] = useState("")
  const [encryptionMethod, setEncryptionMethod] = useState<EncryptionMethod>("none")
  const [encryptionKey, setEncryptionKey] = useState("")
  const [isEncoding, setIsEncoding] = useState(false)
  const [qrData, setQrData] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [copied, setCopied] = useState(false)
  const [publicKeyDisplay, setPublicKeyDisplay] = useState("")

  const loadSampleData = () => {
    const now = Math.floor(Date.now() / 1000)
    setClaim169({
      id: "ID-12345-DEMO",
      fullName: "Jane Marie Smith",
      firstName: "Jane",
      lastName: "Smith",
      dateOfBirth: "1990-05-15",
      gender: 2,
      email: "jane.smith@example.com",
      phone: "+1 555 123 4567",
      address: "123 Main Street\nNew York, NY 10001",
      nationality: "US",
      maritalStatus: 2,
    })
    setCwtMeta({
      issuer: "https://identity.example.org",
      subject: "ID-12345-DEMO",
      issuedAt: now,
      expiresAt: now + 365 * 24 * 60 * 60, // 1 year
    })
    setSigningMethod("ed25519")
    setPrivateKey(SAMPLE_ED25519_PRIVATE_KEY)
    setPublicKeyDisplay(SAMPLE_ED25519_PUBLIC_KEY)
    setEncryptionMethod("none")
    setEncryptionKey("")
    setQrData(null)
    setError(null)
  }

  const handleEncode = async () => {
    setIsEncoding(true)
    setError(null)
    setQrData(null)

    try {
      // Build claim169 input, only include non-empty fields
      const claimInput: Claim169Input = {}
      if (claim169.id) claimInput.id = claim169.id
      if (claim169.fullName) claimInput.fullName = claim169.fullName
      if (claim169.firstName) claimInput.firstName = claim169.firstName
      if (claim169.lastName) claimInput.lastName = claim169.lastName
      if (claim169.dateOfBirth) claimInput.dateOfBirth = claim169.dateOfBirth
      if (claim169.gender !== undefined && claim169.gender !== 0) claimInput.gender = claim169.gender
      if (claim169.email) claimInput.email = claim169.email
      if (claim169.phone) claimInput.phone = claim169.phone
      if (claim169.address) claimInput.address = claim169.address
      if (claim169.nationality) claimInput.nationality = claim169.nationality
      if (claim169.maritalStatus !== undefined && claim169.maritalStatus !== 0) claimInput.maritalStatus = claim169.maritalStatus

      // Build CWT metadata
      const metaInput: CwtMetaInput = {}
      if (cwtMeta.issuer) metaInput.issuer = cwtMeta.issuer
      if (cwtMeta.subject) metaInput.subject = cwtMeta.subject
      if (cwtMeta.expiresAt) metaInput.expiresAt = cwtMeta.expiresAt
      if (cwtMeta.issuedAt) metaInput.issuedAt = cwtMeta.issuedAt

      let encoder = new Encoder(claimInput, metaInput)

      // Apply signing
      if (signingMethod === "ed25519") {
        if (!privateKey.trim()) {
          throw new Error("Private key is required for Ed25519 signing")
        }
        const keyBytes = hexToBytes(privateKey.trim())
        if (keyBytes.length !== 32) {
          throw new Error("Ed25519 private key must be 32 bytes (64 hex characters)")
        }
        encoder = encoder.signWithEd25519(keyBytes)
      } else if (signingMethod === "ecdsa") {
        if (!privateKey.trim()) {
          throw new Error("Private key is required for ECDSA signing")
        }
        const keyBytes = hexToBytes(privateKey.trim())
        if (keyBytes.length !== 32) {
          throw new Error("ECDSA P-256 private key must be 32 bytes (64 hex characters)")
        }
        encoder = encoder.signWithEcdsaP256(keyBytes)
      } else {
        encoder = encoder.allowUnsigned()
      }

      // Apply encryption
      if (encryptionMethod !== "none") {
        if (!encryptionKey.trim()) {
          throw new Error("Encryption key is required")
        }
        const keyBytes = hexToBytes(encryptionKey.trim())
        if (encryptionMethod === "aes256") {
          if (keyBytes.length !== 32) {
            throw new Error("AES-256 key must be 32 bytes (64 hex characters)")
          }
          encoder = encoder.encryptWithAes256(keyBytes)
        } else {
          if (keyBytes.length !== 16) {
            throw new Error("AES-128 key must be 16 bytes (32 hex characters)")
          }
          encoder = encoder.encryptWithAes128(keyBytes)
        }
      }

      const encoded = encoder.encode()
      setQrData(encoded)
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err))
    } finally {
      setIsEncoding(false)
    }
  }

  const handleCopy = async () => {
    if (qrData) {
      await copyToClipboard(qrData)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    }
  }

  const handleDownloadQr = () => {
    if (!qrData) return
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

  const setNow = () => {
    setCwtMeta({ ...cwtMeta, issuedAt: Math.floor(Date.now() / 1000) })
  }

  const setExpires = (days: number) => {
    setCwtMeta({
      ...cwtMeta,
      expiresAt: Math.floor(Date.now() / 1000) + days * 24 * 60 * 60,
    })
  }

  return (
    <div className="grid lg:grid-cols-2 gap-6">
      <div className="space-y-4">
        <Card>
          <CardHeader className="pb-3">
            <div className="flex items-center justify-between">
              <CardTitle className="text-lg">Identity Data</CardTitle>
              <Button variant="outline" size="sm" onClick={loadSampleData}>
                <Sparkles className="h-4 w-4 mr-2" />
                Load Sample
              </Button>
            </div>
            {publicKeyDisplay && (
              <CardDescription className="mt-2">
                <span className="text-xs">
                  Public key for verification:{" "}
                  <code className="bg-muted px-1 py-0.5 rounded text-[10px] font-mono break-all">
                    {publicKeyDisplay}
                  </code>
                </span>
              </CardDescription>
            )}
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="id">ID</Label>
                <Input
                  id="id"
                  value={claim169.id || ""}
                  onChange={(e) => setClaim169({ ...claim169, id: e.target.value })}
                  placeholder="USER-12345"
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="fullName">Full Name</Label>
                <Input
                  id="fullName"
                  value={claim169.fullName || ""}
                  onChange={(e) => setClaim169({ ...claim169, fullName: e.target.value })}
                  placeholder="John Doe"
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="firstName">First Name</Label>
                <Input
                  id="firstName"
                  value={claim169.firstName || ""}
                  onChange={(e) => setClaim169({ ...claim169, firstName: e.target.value })}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="lastName">Last Name</Label>
                <Input
                  id="lastName"
                  value={claim169.lastName || ""}
                  onChange={(e) => setClaim169({ ...claim169, lastName: e.target.value })}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="dob">Date of Birth</Label>
                <Input
                  id="dob"
                  type="date"
                  value={claim169.dateOfBirth || ""}
                  onChange={(e) => setClaim169({ ...claim169, dateOfBirth: e.target.value })}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="gender">Gender</Label>
                <Select
                  id="gender"
                  value={String(claim169.gender || 0)}
                  onChange={(e) => setClaim169({ ...claim169, gender: Number(e.target.value) })}
                >
                  <option value="0">Not specified</option>
                  <option value="1">Male</option>
                  <option value="2">Female</option>
                  <option value="3">Other</option>
                </Select>
              </div>
              <div className="space-y-2">
                <Label htmlFor="email">Email</Label>
                <Input
                  id="email"
                  type="email"
                  value={claim169.email || ""}
                  onChange={(e) => setClaim169({ ...claim169, email: e.target.value })}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="phone">Phone</Label>
                <Input
                  id="phone"
                  value={claim169.phone || ""}
                  onChange={(e) => setClaim169({ ...claim169, phone: e.target.value })}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="nationality">Nationality</Label>
                <Input
                  id="nationality"
                  value={claim169.nationality || ""}
                  onChange={(e) => setClaim169({ ...claim169, nationality: e.target.value })}
                  placeholder="US"
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="maritalStatus">Marital Status</Label>
                <Select
                  id="maritalStatus"
                  value={String(claim169.maritalStatus || 0)}
                  onChange={(e) => setClaim169({ ...claim169, maritalStatus: Number(e.target.value) })}
                >
                  <option value="0">Not specified</option>
                  <option value="1">Unmarried</option>
                  <option value="2">Married</option>
                  <option value="3">Divorced</option>
                </Select>
              </div>
            </div>
            <div className="space-y-2">
              <Label htmlFor="address">Address</Label>
              <Textarea
                id="address"
                value={claim169.address || ""}
                onChange={(e) => setClaim169({ ...claim169, address: e.target.value })}
                placeholder="123 Main Street, City, Country"
              />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg">CWT Metadata</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2 col-span-2">
                <Label htmlFor="issuer">Issuer</Label>
                <Input
                  id="issuer"
                  value={cwtMeta.issuer || ""}
                  onChange={(e) => setCwtMeta({ ...cwtMeta, issuer: e.target.value })}
                  placeholder="https://identity.example.com"
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="subject">Subject</Label>
                <Input
                  id="subject"
                  value={cwtMeta.subject || ""}
                  onChange={(e) => setCwtMeta({ ...cwtMeta, subject: e.target.value })}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="issuedAt">Issued At (Unix)</Label>
                <div className="flex gap-2">
                  <Input
                    id="issuedAt"
                    type="number"
                    value={cwtMeta.issuedAt || ""}
                    onChange={(e) => setCwtMeta({ ...cwtMeta, issuedAt: Number(e.target.value) || undefined })}
                  />
                  <Button variant="outline" size="sm" onClick={setNow}>
                    Now
                  </Button>
                </div>
              </div>
              <div className="space-y-2 col-span-2">
                <Label htmlFor="expiresAt">Expires At (Unix)</Label>
                <div className="flex gap-2">
                  <Input
                    id="expiresAt"
                    type="number"
                    value={cwtMeta.expiresAt || ""}
                    onChange={(e) => setCwtMeta({ ...cwtMeta, expiresAt: Number(e.target.value) || undefined })}
                    className="flex-1"
                  />
                  <Button variant="outline" size="sm" onClick={() => setExpires(30)}>
                    +30d
                  </Button>
                  <Button variant="outline" size="sm" onClick={() => setExpires(365)}>
                    +1y
                  </Button>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg">Signing</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex flex-wrap gap-4">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="signing"
                  checked={signingMethod === "ed25519"}
                  onChange={() => setSigningMethod("ed25519")}
                  className="w-4 h-4"
                />
                <span className="text-sm">Ed25519</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="signing"
                  checked={signingMethod === "ecdsa"}
                  onChange={() => setSigningMethod("ecdsa")}
                  className="w-4 h-4"
                />
                <span className="text-sm">ECDSA P-256</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="signing"
                  checked={signingMethod === "unsigned"}
                  onChange={() => setSigningMethod("unsigned")}
                  className="w-4 h-4"
                />
                <span className="text-sm text-muted-foreground">Unsigned (testing only)</span>
              </label>
            </div>

            {signingMethod !== "unsigned" && (
              <div className="space-y-2">
                <Label htmlFor="privateKey">Private Key (hex)</Label>
                <Input
                  id="privateKey"
                  type="password"
                  placeholder="32 bytes (64 hex chars)"
                  value={privateKey}
                  onChange={(e) => setPrivateKey(e.target.value)}
                  className="font-mono text-sm"
                />
              </div>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg">Encryption (Optional)</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex flex-wrap gap-4">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="encryption"
                  checked={encryptionMethod === "none"}
                  onChange={() => setEncryptionMethod("none")}
                  className="w-4 h-4"
                />
                <span className="text-sm">None</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="encryption"
                  checked={encryptionMethod === "aes256"}
                  onChange={() => setEncryptionMethod("aes256")}
                  className="w-4 h-4"
                />
                <span className="text-sm">AES-256-GCM</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="encryption"
                  checked={encryptionMethod === "aes128"}
                  onChange={() => setEncryptionMethod("aes128")}
                  className="w-4 h-4"
                />
                <span className="text-sm">AES-128-GCM</span>
              </label>
            </div>

            {encryptionMethod !== "none" && (
              <div className="space-y-2">
                <Label htmlFor="encryptionKey">Encryption Key (hex)</Label>
                <Input
                  id="encryptionKey"
                  type="password"
                  placeholder={encryptionMethod === "aes256" ? "32 bytes (64 hex chars)" : "16 bytes (32 hex chars)"}
                  value={encryptionKey}
                  onChange={(e) => setEncryptionKey(e.target.value)}
                  className="font-mono text-sm"
                />
              </div>
            )}
          </CardContent>
        </Card>

        <Button onClick={handleEncode} disabled={isEncoding} className="w-full">
          {isEncoding ? (
            <Loader2 className="h-4 w-4 mr-2 animate-spin" />
          ) : (
            <Play className="h-4 w-4 mr-2" />
          )}
          Generate QR Code
        </Button>
      </div>

      <div className="space-y-4">
        {error && (
          <Card className="border-destructive">
            <CardHeader>
              <CardTitle className="text-destructive">Encode Error</CardTitle>
            </CardHeader>
            <CardContent>
              <pre className="text-sm bg-destructive/10 p-4 rounded-md overflow-auto whitespace-pre-wrap">
                {error}
              </pre>
            </CardContent>
          </Card>
        )}

        {qrData && (
          <>
            <Card>
              <CardHeader className="pb-3">
                <CardTitle className="text-lg">Generated QR Code</CardTitle>
              </CardHeader>
              <CardContent className="flex flex-col items-center gap-4">
                <div className="p-4 bg-white rounded-lg">
                  <QRCodeSVG
                    id="qr-code-svg"
                    value={qrData}
                    size={256}
                    level="M"
                  />
                </div>
                <Button variant="outline" onClick={handleDownloadQr}>
                  <Download className="h-4 w-4 mr-2" />
                  Download PNG
                </Button>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="pb-3">
                <div className="flex items-center justify-between">
                  <CardTitle className="text-lg">Base45 Data</CardTitle>
                  <Button variant="ghost" size="sm" onClick={handleCopy}>
                    {copied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
                  </Button>
                </div>
              </CardHeader>
              <CardContent>
                <pre className="text-xs bg-muted p-4 rounded-md overflow-auto max-h-48 break-all whitespace-pre-wrap font-mono">
                  {qrData}
                </pre>
                <p className="text-xs text-muted-foreground mt-2">
                  {qrData.length} characters
                </p>
              </CardContent>
            </Card>
          </>
        )}

        {!qrData && !error && (
          <Card>
            <CardContent className="py-12 text-center text-muted-foreground">
              <p>Fill in the form and click Generate to create a QR code</p>
            </CardContent>
          </Card>
        )}
      </div>
    </div>
  )
}
