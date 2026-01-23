import { useState, useEffect, useCallback, useRef } from "react"
import { Encoder, Decoder, type Claim169Input, type CwtMetaInput, type DecodeResult } from "claim169"
import { IdentityPanel } from "@/components/IdentityPanel"
import { ResultPanel } from "@/components/ResultPanel"
import { QrScanner } from "@/components/QrScanner"
import { hexToBytes, generateEd25519KeyPair, generateEcdsaP256KeyPair, generateAesKey } from "@/lib/utils"
import type { PipelineStage } from "@/components/PipelineDetails"
import type { VerificationStatus } from "@/components/VerificationBadge"

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
        name: "zlib",
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

      // Improve error messages for algorithm mismatches
      let friendlyError = errorMessage
      if (errorMessage.includes("unsupported algorithm: EdDSA") && signingMethod === "ecdsa") {
        friendlyError = "This QR code was signed with EdDSA (Ed25519). Switch to Ed25519 to verify."
      } else if (errorMessage.includes("unsupported algorithm: ES256") && signingMethod === "ed25519") {
        friendlyError = "This QR code was signed with ECDSA P-256. Switch to ECDSA P-256 to verify."
      } else if (errorMessage.includes("signature verification failed")) {
        friendlyError = "Signature verification failed. Make sure you're using the correct public key."
      }

      setError(friendlyError)
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

  // Load sample data with fresh keys
  const loadSampleData = async () => {
    const now = Math.floor(Date.now() / 1000)

    // Generate fresh keys for each session
    const keyPair = await generateEd25519KeyPair()

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
    setPrivateKey(keyPair.privateKey)
    setPublicKey(keyPair.publicKey)
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
        onSigningMethodChange={handleSigningMethodChange}
        privateKey={privateKey}
        onPrivateKeyChange={setPrivateKey}
        publicKey={publicKey}
        onPublicKeyChange={setPublicKey}
        encryptionMethod={encryptionMethod}
        onEncryptionMethodChange={handleEncryptionMethodChange}
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
