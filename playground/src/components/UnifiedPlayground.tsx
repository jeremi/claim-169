import { useState, useEffect, useCallback, useRef } from "react"
import { Encoder, Decoder, type Claim169Input, type CwtMetaInput, type DecodeResult } from "claim169"
import { IdentityPanel } from "@/components/IdentityPanel"
import { ResultPanel } from "@/components/ResultPanel"
import { QrScanner } from "@/components/QrScanner"
import {
  hexToBytes,
  generateEd25519KeyPair,
  generateEcdsaP256KeyPair,
  generateAesKey,
  detectPublicKeyFormat,
  parseEncryptionKey,
} from "@/lib/utils"
import { parseDecodeError, buildPartialPipeline, type ParsedError } from "@/lib/errors"
import type { PipelineStage } from "@/components/PipelineDetails"
import type { VerificationStatus } from "@/components/VerificationBadge"

export type SigningMethod = "ed25519" | "ecdsa" | "unsigned"
export type EncryptionMethod = "none" | "aes256" | "aes128"
export type CompressionMode = "zlib" | "none" | "adaptive-brotli:9" | "brotli:9"


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
  const [compressionMode, setCompressionMode] = useState<CompressionMode>("zlib")

  // Status
  const [verificationStatus, setVerificationStatus] = useState<VerificationStatus>("none")
  const [algorithm, setAlgorithm] = useState<string | undefined>()
  const [error, setError] = useState<string | null>(null)
  const [parsedError, setParsedError] = useState<ParsedError | null>(null)
  const [pipelineStages, setPipelineStages] = useState<PipelineStage[]>([])

  // UI state
  const [showScanner, setShowScanner] = useState(false)
  const [isProcessing, setIsProcessing] = useState(false)
  const [samplePhotoUrl, setSamplePhotoUrl] = useState<string | null>(null)

  // Track which side initiated the last change to prevent loops
  const lastChangeSource = useRef<"encode" | "decode" | null>(null)
  const isInitialMount = useRef(true)
  // Guards against onClaim169Change resetting lastChangeSource during
  // programmatic example loads (where async photo fetch can race with
  // the debounced encode/decode effects).
  const programmaticLoadRef = useRef(false)

  // Generate fresh keys when signing method changes (if no key is set)
  const generateFreshKeys = useCallback(async (method: SigningMethod) => {
    if (method === "unsigned") {
      setPrivateKey("")
      setPublicKey("")
      return
    }

    if (method === "ed25519") {
      const keyPair = await generateEd25519KeyPair()
      setPrivateKey(keyPair.privateKey)
      setPublicKey(keyPair.publicKey)
    } else if (method === "ecdsa") {
      const keyPair = await generateEcdsaP256KeyPair()
      setPrivateKey(keyPair.privateKey)
      setPublicKey(keyPair.publicKey)
    }
  }, [])

  // Handle signing method change - generate new keys
  const handleSigningMethodChange = useCallback((method: SigningMethod) => {
    setSigningMethod(method)
    // Generate fresh keys when switching methods (for security)
    generateFreshKeys(method)
  }, [generateFreshKeys])

  // Handle encryption method change - generate new key
  const handleEncryptionMethodChange = useCallback((method: EncryptionMethod) => {
    setEncryptionMethod(method)
    if (method === "none") {
      setEncryptionKey("")
    } else {
      // Generate fresh encryption key
      const bits = method === "aes256" ? 256 : 128
      setEncryptionKey(generateAesKey(bits))
    }
  }, [])

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
    setParsedError(null)

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
      if (claim169.locationCode) claimInput.locationCode = claim169.locationCode
      if (claim169.legalStatus) claimInput.legalStatus = claim169.legalStatus
      if (claim169.countryOfIssuance) claimInput.countryOfIssuance = claim169.countryOfIssuance
      if (claim169.photo) claimInput.photo = claim169.photo
      if (claim169.photoFormat) claimInput.photoFormat = claim169.photoFormat
      if (claim169.face) claimInput.face = claim169.face

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

      // Apply compression
      encoder = encoder.compression(compressionMode)

      const result = encoder.encode()
      const encoded = result.qrData

      // Build pipeline stages - only show actual values, not estimates
      stages.push({
        name: "Claim 169",
        inputSize: 0,
        outputSize: 0,
        status: "success",
        details: {
          fields: Object.keys(claimInput).length,
        },
      })

      stages.push({
        name: "CWT/CBOR",
        inputSize: 0,
        outputSize: 0,
        status: "success",
      })

      stages.push({
        name: "COSE_Sign1",
        inputSize: 0,
        outputSize: 0,
        status: "success",
        details: {
          algorithm: algName,
        },
      })

      stages.push({
        name: result.compressionUsed,
        inputSize: 0,
        outputSize: 0,
        status: "success",
      })

      stages.push({
        name: "Base45",
        inputSize: 0,
        outputSize: 0,
        status: "success",
        details: {
          characters: encoded.length,
        },
      })

      lastChangeSource.current = "encode"
      programmaticLoadRef.current = false
      setBase45Data(encoded)
      setPipelineStages(stages)
      setAlgorithm(algName)
      setVerificationStatus(signingMethod !== "unsigned" ? "verified" : "none")
    } catch (err) {
      programmaticLoadRef.current = false
      setError(err instanceof Error ? err.message : String(err))
      setVerificationStatus("invalid")
    } finally {
      setIsProcessing(false)
    }
  }, [claim169, cwtMeta, signingMethod, privateKey, encryptionMethod, encryptionKey, compressionMode])

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
    setParsedError(null)
      return
    }

    setIsProcessing(true)
    setError(null)
    setParsedError(null)

    try {
      let decoder = new Decoder(base45Data.trim())

      // Apply decryption if configured
      if (encryptionMethod !== "none" && encryptionKey.trim()) {
        // Auto-detect encryption key format (hex or base64)
        const expectedLength = encryptionMethod === "aes256" ? 32 : 16
        const keyBytes = parseEncryptionKey(encryptionKey.trim(), expectedLength as 16 | 32)
        if (encryptionMethod === "aes256") {
          decoder = decoder.decryptWithAes256(keyBytes)
        } else {
          decoder = decoder.decryptWithAes128(keyBytes)
        }
      }

      // Apply verification if configured - auto-detect hex vs PEM format
      if (publicKey.trim() && signingMethod !== "unsigned") {
        const keyFormat = detectPublicKeyFormat(publicKey.trim())
        if (signingMethod === "ed25519") {
          if (keyFormat === "pem") {
            decoder = decoder.verifyWithEd25519Pem(publicKey.trim())
          } else {
            const keyBytes = hexToBytes(publicKey.trim())
            decoder = decoder.verifyWithEd25519(keyBytes)
          }
        } else if (signingMethod === "ecdsa") {
          if (keyFormat === "pem") {
            decoder = decoder.verifyWithEcdsaP256Pem(publicKey.trim())
          } else {
            const keyBytes = hexToBytes(publicKey.trim())
            decoder = decoder.verifyWithEcdsaP256(keyBytes)
          }
        }
      } else {
        decoder = decoder.allowUnverified()
      }

      const result: DecodeResult = decoder.decode()

      // Build pipeline stages for decode - only show what we actually know
      const stages: PipelineStage[] = []

      stages.push({
        name: "Base45",
        inputSize: 0,
        outputSize: 0,
        status: "success",
        details: { characters: base45Data.length },
      })

      stages.push({
        name: "zlib",
        inputSize: 0,
        outputSize: 0,
        status: "success",
      })

      // Add encryption stage if encryption is configured
      if (encryptionMethod !== "none") {
        const hasDecryptionKey = encryptionKey.trim().length > 0
        stages.push({
          name: "COSE_Encrypt0",
          inputSize: 0,
          outputSize: 0,
          status: hasDecryptionKey ? "success" : "skipped",
          details: {
            algorithm: encryptionMethod === "aes256" ? "AES-256-GCM" : "AES-128-GCM",
          },
        })
      }

      stages.push({
        name: "COSE_Sign1",
        inputSize: 0,
        outputSize: 0,
        status: result.verificationStatus === "verified" ? "success" : "skipped",
        details: {
          verified: result.verificationStatus === "verified",
        },
      })

      stages.push({
        name: "CWT/CBOR",
        inputSize: 0,
        outputSize: 0,
        status: "success",
      })

      const fieldCount = Object.keys(result.claim169).filter(
        (k) => result.claim169[k as keyof typeof result.claim169] !== undefined
      ).length
      stages.push({
        name: "Claim 169",
        inputSize: 0,
        outputSize: 0,
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
      const errorMessage = err instanceof Error ? err.message : String(err)

      // Parse the error to determine stage and provide suggestions
      const parsed = parseDecodeError(errorMessage)
      setParsedError(parsed)

      // Build partial pipeline showing where the error occurred
      const isEncrypted = encryptionMethod !== "none"
      const partialStages = buildPartialPipeline(parsed.stage, isEncrypted)
      setPipelineStages(partialStages.map(s => ({
        name: s.name,
        inputSize: 0,
        outputSize: 0,
        status: s.status,
      })))

      // Set user-friendly error message
      setError(parsed.message)
      setVerificationStatus("invalid")
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

  // Load sample data with fresh keys
  const loadSampleData = async () => {
    const now = Math.floor(Date.now() / 1000)
    const keyPair = await generateEd25519KeyPair()

    // Prevent the stale decode effect from clearing claim169 before encode runs.
    // programmaticLoadRef guards against onClaim169Change (from async photo load)
    // resetting lastChangeSource before the debounced effects fire.
    lastChangeSource.current = "encode"
    programmaticLoadRef.current = true
    setClaim169({
      id: "ID-12345-DEMO",
      fullName: "Siriwan Chaiyaporn",
      dateOfBirth: "1990-05-15",
      gender: 2,
      nationality: "TH",
    })
    setCwtMeta({
      issuer: "https://id.example.org",
      subject: "ID-12345-DEMO",
      issuedAt: now,
      expiresAt: now + 365 * 24 * 60 * 60,
    })
    setSigningMethod("ed25519")
    setPrivateKey(keyPair.privateKey)
    setPublicKey(keyPair.publicKey)
    setEncryptionMethod("none")
    setEncryptionKey("")
    setError(null)
    setParsedError(null)
    // Photo loaded via PhotoUpload's samplePhotoUrl — enables compression sliders
    setSamplePhotoUrl(import.meta.env.BASE_URL + "sample_id_pictures/sample_id_4.png")
  }

  // Load refugee identity example with sample photo
  const loadRefugeeExample = () => {
    const now = Math.floor(Date.now() / 1000)

    lastChangeSource.current = "encode"
    programmaticLoadRef.current = true
    setClaim169({
      id: "3215489387",
      fullName: "Janardhan Bangalore Srinivas",
      language: "eng",
      secondaryLanguage: "ara",
      secondaryFullName: "\u062C\u0627\u0646\u0627\u0631\u062F\u0627\u0646 \u0628\u0646\u063A\u0627\u0644\u0648\u0631 \u0633\u0631\u064A\u0646\u064A\u0641\u0627\u0633",
      dateOfBirth: "1987-03-12",
      gender: 1,
      nationality: "IN",
      legalStatus: "refugee",
    })
    setCwtMeta({
      issuer: "https://unhcr.example.org",
      subject: "3215489387",
      issuedAt: now,
      expiresAt: now + 365 * 24 * 60 * 60,
    })
    setSigningMethod("unsigned")
    setPrivateKey("")
    setPublicKey("")
    setEncryptionMethod("none")
    setEncryptionKey("")
    setError(null)
    setParsedError(null)
    // Photo loaded via PhotoUpload's samplePhotoUrl — enables compression sliders
    setSamplePhotoUrl(import.meta.env.BASE_URL + "sample_id_pictures/sample_id_1.png")
  }

  // Load example by key
  const loadExample = (key: string) => {
    if (key === "refugee-identity") {
      loadRefugeeExample()
    }
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
        encodedSize={base45Data.length || undefined}
        onClaim169Change={(value) => {
          if (!programmaticLoadRef.current) {
            lastChangeSource.current = null
          }
          setClaim169(value)
        }}
        cwtMeta={cwtMeta}
        onCwtMetaChange={(value) => {
          lastChangeSource.current = null
          setCwtMeta(value)
        }}
        signingMethod={signingMethod}
        onSigningMethodChange={handleSigningMethodChange}
        privateKey={privateKey}
        onPrivateKeyChange={setPrivateKey}
        publicKey={publicKey}
        onPublicKeyChange={setPublicKey}
        encryptionMethod={encryptionMethod}
        onEncryptionMethodChange={handleEncryptionMethodChange}
        encryptionKey={encryptionKey}
        onEncryptionKeyChange={setEncryptionKey}
        compressionMode={compressionMode}
        onCompressionModeChange={setCompressionMode}
        onLoadSample={loadSampleData}
        onLoadExample={loadExample}
        samplePhotoUrl={samplePhotoUrl}
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
        parsedError={parsedError}
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
