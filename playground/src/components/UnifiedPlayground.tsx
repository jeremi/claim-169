import { useState, useEffect, useCallback, useRef } from "react"
import { Encoder, Decoder, type Claim169Input, type CwtMetaInput, type DecodeResult } from "claim169"
import { IdentityPanel } from "@/components/IdentityPanel"
import { ResultPanel } from "@/components/ResultPanel"
import { QrScanner } from "@/components/QrScanner"
import { hexToBytes } from "@/lib/utils"
import type { PipelineStage } from "@/components/PipelineDetails"
import type { VerificationStatus } from "@/components/VerificationBadge"

// Sample test keys (from test vectors - for testing only!)
const SAMPLE_ED25519_PRIVATE_KEY = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
const SAMPLE_ED25519_PUBLIC_KEY = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"

export type SigningMethod = "ed25519" | "ecdsa" | "unsigned"
export type EncryptionMethod = "none" | "aes256" | "aes128"

// Pre-made examples
const EXAMPLES = {
  "ed25519-signed": {
    name: "Ed25519 Signed",
    qrData: "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4",
    publicKey: "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
  },
  "demographics-full": {
    name: "Full Demographics",
    qrData: "6BFA$BMVPYUOM43QVJH.AB27/RBNIDSMGU*4WKQYPDP-MBVDN66$EQM0JBF6YRC/S9N%8T2M.52+11A9G+PQELUPWJ0$RK2GW0PTZJ$6EC/7Z$Q718XV1WGD$DE0%CBBS0I7YEPUMD1F4OPQ3LD:T50KNJ6DKSVG%63EUHK2LJB4BQH 8UMQI5S7-L24WFDVB2N*OQRRMLF15U7X9L1AGVFU0WCJC0.CCW/DB*C*YKTZM9ULPW5U0ND94X9F/07UK2K*EC%2$I0BH3YUA*4MRQQIHS9WDB813.3B11%RRAFFM23N263SG91CDMIWFEI40$43AAII9UNAN6 P01E$M7A*VSTCUL1LILRV2 4BSL5HWPWGSEQOGHPZBV2-P6902HFVZTBJ37-8YF9Y/8JCA9O3XA0:VES4",
    publicKey: "",
  },
  "encrypted-signed": {
    name: "Encrypted + Signed",
    qrData: "6BFCA0410DFWXQG.FKTK06U0 DKAPKT K33LMEL-PLV9BIQFGWFU2G8 N.BGZF1VSIYEV7Q3FLRS3AJNDF.3Z0N+ZE:MD1D8R22LF3WC9*9O-P9VYJ3NGZ%3L9BTN0/5LOP0X2RL0ERSVBHVN EUZUQ2C$Y05CLKYU+9653893EU%2ICEALJ864MHB:QK 2SSTKGY6EPJFFS:R0OBF7O57NDH$12PBS1GOQGKY2BDW0Z02NPQO0SPGZ1REG0GPGV6OCRN0RI$IN751T%2SO2TFI2DSV431V5TYUW5H9 9QJU.JN7M6$95:1FJKF9P84DRF/P:0",
    publicKey: "994c54604862f73d4bce14120b318f720119c6498b59257fb89cbead939ba0f5",
    decryptionKey: "101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
    encryptionMethod: "aes256" as EncryptionMethod,
  },
}

export function UnifiedPlayground() {
  // Core state
  const [base45Data, setBase45Data] = useState("")
  const [claim169, setClaim169] = useState<Claim169Input>({})
  const [cwtMeta, setCwtMeta] = useState<CwtMetaInput>({})

  // Crypto settings
  const [signingMethod, setSigningMethod] = useState<SigningMethod>("ed25519")
  const [privateKey, setPrivateKey] = useState("")
  const [publicKey, setPublicKey] = useState("")
  const [encryptionMethod, setEncryptionMethod] = useState<EncryptionMethod>("none")
  const [encryptionKey, setEncryptionKey] = useState("")

  // Status
  const [verificationStatus, setVerificationStatus] = useState<VerificationStatus>("none")
  const [algorithm, setAlgorithm] = useState<string | undefined>()
  const [error, setError] = useState<string | null>(null)
  const [pipelineStages, setPipelineStages] = useState<PipelineStage[]>([])

  // UI state
  const [showScanner, setShowScanner] = useState(false)
  const [isProcessing, setIsProcessing] = useState(false)

  // Track which side initiated the last change to prevent loops
  const lastChangeSource = useRef<"encode" | "decode" | null>(null)
  const isInitialMount = useRef(true)

  // Derive public key from private key
  useEffect(() => {
    if (!privateKey.trim() || signingMethod === "unsigned") {
      setPublicKey("")
      return
    }

    try {
      const keyBytes = hexToBytes(privateKey.trim())
      if (keyBytes.length !== 32) {
        setPublicKey("")
        return
      }

      // Derive public key based on signing method
      if (signingMethod === "ed25519") {
        // For Ed25519, we use the sample public key if it matches the sample private key
        // In a real implementation, we'd call a WASM function to derive it
        if (privateKey.trim() === SAMPLE_ED25519_PRIVATE_KEY) {
          setPublicKey(SAMPLE_ED25519_PUBLIC_KEY)
        } else {
          // For other keys, we'd need WASM support to derive
          // For now, clear and let the user provide/copy from generated output
          setPublicKey("")
        }
      } else {
        setPublicKey("")
      }
    } catch {
      setPublicKey("")
    }
  }, [privateKey, signingMethod])

  // Encode: When decoded fields change, regenerate Base45
  const regenerateEncoded = useCallback(() => {
    if (lastChangeSource.current === "decode") {
      lastChangeSource.current = null
      return
    }

    // Don't encode if no identity data
    const hasData = claim169.id || claim169.fullName || claim169.firstName || claim169.lastName
    if (!hasData) {
      setBase45Data("")
      setPipelineStages([])
      return
    }

    setIsProcessing(true)
    setError(null)

    try {
      // Build claim169 input, only include non-empty fields
      const claimInput: Claim169Input = {}
      if (claim169.id) claimInput.id = claim169.id
      if (claim169.fullName) claimInput.fullName = claim169.fullName
      if (claim169.firstName) claimInput.firstName = claim169.firstName
      if (claim169.lastName) claimInput.lastName = claim169.lastName
      if (claim169.language) claimInput.language = claim169.language
      if (claim169.secondaryFullName) claimInput.secondaryFullName = claim169.secondaryFullName
      if (claim169.secondaryLanguage) claimInput.secondaryLanguage = claim169.secondaryLanguage
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

      // Track encoding stages
      const stages: PipelineStage[] = []

      // Apply signing
      let algName = "Unsigned"
      if (signingMethod === "ed25519") {
        if (!privateKey.trim()) {
          throw new Error("Private key is required for Ed25519 signing")
        }
        const keyBytes = hexToBytes(privateKey.trim())
        if (keyBytes.length !== 32) {
          throw new Error("Ed25519 private key must be 32 bytes (64 hex characters)")
        }
        encoder = encoder.signWithEd25519(keyBytes)
        algName = "EdDSA (Ed25519)"
      } else if (signingMethod === "ecdsa") {
        if (!privateKey.trim()) {
          throw new Error("Private key is required for ECDSA signing")
        }
        const keyBytes = hexToBytes(privateKey.trim())
        if (keyBytes.length !== 32) {
          throw new Error("ECDSA P-256 private key must be 32 bytes (64 hex characters)")
        }
        encoder = encoder.signWithEcdsaP256(keyBytes)
        algName = "ES256 (P-256)"
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

      // Build pipeline stages (approximate sizes)
      const jsonSize = JSON.stringify(claimInput).length + JSON.stringify(metaInput).length
      stages.push({
        name: "Claim 169",
        inputSize: jsonSize,
        outputSize: jsonSize,
        status: "success",
        details: {
          fields: Object.keys(claimInput).length,
        },
      })

      // Estimate CBOR size (roughly 80% of JSON)
      const cborSize = Math.round(jsonSize * 0.8)
      stages.push({
        name: "CWT/CBOR",
        inputSize: jsonSize,
        outputSize: cborSize,
        status: "success",
        details: {
          hasClaims: Boolean(cwtMeta.issuer || cwtMeta.subject),
        },
      })

      // COSE adds signature
      const signatureSize = signingMethod !== "unsigned" ? 64 : 0
      const coseSize = cborSize + signatureSize + 20 // header overhead
      stages.push({
        name: "COSE_Sign1",
        inputSize: cborSize,
        outputSize: coseSize,
        status: "success",
        details: {
          algorithm: algName,
          signatureBytes: signatureSize,
        },
      })

      // Compression (typically 60-70% ratio)
      const compressedSize = Math.round(coseSize * 0.65)
      stages.push({
        name: "zlib",
        inputSize: coseSize,
        outputSize: compressedSize,
        status: "success",
        details: {
          ratio: `${Math.round((1 - compressedSize / coseSize) * 100)}%`,
        },
      })

      // Base45 expands by ~35%
      const base45Size = encoded.length
      stages.push({
        name: "Base45",
        inputSize: compressedSize,
        outputSize: base45Size,
        status: "success",
        details: {
          characters: base45Size,
        },
      })

      lastChangeSource.current = "encode"
      setBase45Data(encoded)
      setPipelineStages(stages)
      setAlgorithm(algName)
      setVerificationStatus(signingMethod !== "unsigned" ? "verified" : "none")
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err))
      setVerificationStatus("invalid")
    } finally {
      setIsProcessing(false)
    }
  }, [claim169, cwtMeta, signingMethod, privateKey, encryptionMethod, encryptionKey])

  // Decode: When Base45 input changes, decode and populate fields
  const decodeAndPopulate = useCallback(() => {
    if (lastChangeSource.current === "encode") {
      lastChangeSource.current = null
      return
    }

    if (!base45Data.trim()) {
      // Clear everything when empty
      setClaim169({})
      setCwtMeta({})
      setPipelineStages([])
      setVerificationStatus("none")
      setAlgorithm(undefined)
      setError(null)
      return
    }

    setIsProcessing(true)
    setError(null)

    try {
      let decoder = new Decoder(base45Data.trim())

      // Apply decryption if configured
      if (encryptionMethod !== "none" && encryptionKey.trim()) {
        const keyBytes = hexToBytes(encryptionKey.trim())
        if (encryptionMethod === "aes256") {
          decoder = decoder.decryptWithAes256(keyBytes)
        } else {
          decoder = decoder.decryptWithAes128(keyBytes)
        }
      }

      // Apply verification if configured
      if (publicKey.trim() && signingMethod !== "unsigned") {
        const keyBytes = hexToBytes(publicKey.trim())
        if (signingMethod === "ed25519") {
          decoder = decoder.verifyWithEd25519(keyBytes)
        } else if (signingMethod === "ecdsa") {
          decoder = decoder.verifyWithEcdsaP256(keyBytes)
        }
      } else {
        decoder = decoder.allowUnverified()
      }

      const result: DecodeResult = decoder.decode()

      // Build pipeline stages for decode
      const stages: PipelineStage[] = []

      stages.push({
        name: "Base45",
        inputSize: base45Data.length,
        outputSize: Math.round(base45Data.length * 0.74),
        status: "success",
        details: { characters: base45Data.length },
      })

      const decompressedSize = Math.round(base45Data.length * 0.74 / 0.65)
      stages.push({
        name: "zlib",
        inputSize: Math.round(base45Data.length * 0.74),
        outputSize: decompressedSize,
        status: "success",
      })

      stages.push({
        name: "COSE_Sign1",
        inputSize: decompressedSize,
        outputSize: decompressedSize - 84,
        status: "success",
        details: {
          verified: result.verificationStatus === "verified",
        },
      })

      stages.push({
        name: "CWT/CBOR",
        inputSize: decompressedSize - 84,
        outputSize: decompressedSize - 100,
        status: "success",
      })

      const fieldCount = Object.keys(result.claim169).filter(
        (k) => result.claim169[k as keyof typeof result.claim169] !== undefined
      ).length
      stages.push({
        name: "Claim 169",
        inputSize: decompressedSize - 100,
        outputSize: fieldCount,
        status: "success",
        details: { fields: fieldCount },
      })

      lastChangeSource.current = "decode"
      setClaim169(result.claim169)
      setCwtMeta(result.cwtMeta)
      setPipelineStages(stages)

      // Set verification status
      if (result.verificationStatus === "verified") {
        setVerificationStatus("verified")
        setAlgorithm("EdDSA")
      } else if (result.verificationStatus === "skipped") {
        setVerificationStatus("unverified")
        setAlgorithm(undefined)
      } else {
        setVerificationStatus("invalid")
        setAlgorithm(undefined)
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err))
      setVerificationStatus("invalid")
      setPipelineStages([])
    } finally {
      setIsProcessing(false)
    }
  }, [base45Data, publicKey, signingMethod, encryptionMethod, encryptionKey])

  // Debounced encode effect
  useEffect(() => {
    if (isInitialMount.current) {
      isInitialMount.current = false
      return
    }

    const timer = setTimeout(regenerateEncoded, 300)
    return () => clearTimeout(timer)
  }, [regenerateEncoded])

  // Debounced decode effect (triggered by base45Data changes)
  useEffect(() => {
    const timer = setTimeout(decodeAndPopulate, 300)
    return () => clearTimeout(timer)
  }, [decodeAndPopulate])

  // Load sample data
  const loadSampleData = () => {
    const now = Math.floor(Date.now() / 1000)
    setClaim169({
      id: "ID-12345-DEMO",
      fullName: "Siriwan Chaiyaporn",
      firstName: "Siriwan",
      lastName: "Chaiyaporn",
      language: "eng",
      secondaryFullName: "ศิริวรรณ ไชยพร",
      secondaryLanguage: "tha",
      dateOfBirth: "1990-05-15",
      gender: 2,
      email: "siriwan.chaiyaporn@example.com",
      phone: "+66 81 234 5678",
      address: "123 Moo 5, Ban Nong Khai\nNong Khai District, Nong Khai 43000",
      nationality: "TH",
      maritalStatus: 2,
    })
    setCwtMeta({
      issuer: "https://identity.example.org",
      subject: "ID-12345-DEMO",
      issuedAt: now,
      expiresAt: now + 365 * 24 * 60 * 60,
    })
    setSigningMethod("ed25519")
    setPrivateKey(SAMPLE_ED25519_PRIVATE_KEY)
    setPublicKey(SAMPLE_ED25519_PUBLIC_KEY)
    setEncryptionMethod("none")
    setEncryptionKey("")
    setError(null)
  }

  // Load example
  const loadExample = (key: string) => {
    const example = EXAMPLES[key as keyof typeof EXAMPLES]
    if (!example) return

    lastChangeSource.current = null
    setBase45Data(example.qrData)
    setPublicKey(example.publicKey)
    if ("decryptionKey" in example && example.decryptionKey) {
      setEncryptionKey(example.decryptionKey)
      setEncryptionMethod(example.encryptionMethod || "none")
    } else {
      setEncryptionKey("")
      setEncryptionMethod("none")
    }
    setSigningMethod(example.publicKey ? "ed25519" : "unsigned")
    setPrivateKey("")
    setError(null)
  }

  // Handle QR scan
  const handleScan = (data: string) => {
    lastChangeSource.current = null
    setBase45Data(data)
    setShowScanner(false)
  }

  return (
    <div className="grid lg:grid-cols-2 gap-6 h-full">
      {/* Left Panel: Identity + CWT + Crypto */}
      <IdentityPanel
        claim169={claim169}
        onClaim169Change={(value) => {
          lastChangeSource.current = null
          setClaim169(value)
        }}
        cwtMeta={cwtMeta}
        onCwtMetaChange={(value) => {
          lastChangeSource.current = null
          setCwtMeta(value)
        }}
        signingMethod={signingMethod}
        onSigningMethodChange={setSigningMethod}
        privateKey={privateKey}
        onPrivateKeyChange={setPrivateKey}
        publicKey={publicKey}
        encryptionMethod={encryptionMethod}
        onEncryptionMethodChange={setEncryptionMethod}
        encryptionKey={encryptionKey}
        onEncryptionKeyChange={setEncryptionKey}
        onLoadSample={loadSampleData}
        onLoadExample={loadExample}
        examples={EXAMPLES}
      />

      {/* Right Panel: QR Code + Verification + Base45 */}
      <ResultPanel
        base45Data={base45Data}
        onBase45Change={(value) => {
          lastChangeSource.current = null
          setBase45Data(value)
        }}
        verificationStatus={verificationStatus}
        algorithm={algorithm}
        pipelineStages={pipelineStages}
        onScanClick={() => setShowScanner(true)}
        isProcessing={isProcessing}
        error={error}
        claim169={claim169 as Record<string, unknown>}
        cwtMeta={cwtMeta as Record<string, unknown>}
      />

      {showScanner && (
        <QrScanner
          onScan={handleScan}
          onClose={() => setShowScanner(false)}
        />
      )}
    </div>
  )
}
