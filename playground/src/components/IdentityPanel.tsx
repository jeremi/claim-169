import { useState, useCallback, useRef } from "react"
import { useTranslation } from "react-i18next"
import type { Claim169Input, CwtMetaInput } from "claim169"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Select } from "@/components/ui/select"
import { Textarea } from "@/components/ui/textarea"
import { ChevronDown, ChevronRight, Copy, Check, Eye, EyeOff, AlertTriangle, RefreshCw } from "lucide-react"
import { copyToClipboard, generateEd25519KeyPair, generateEcdsaP256KeyPair, generateAesKey, detectPublicKeyFormat, detectEncryptionKeyFormat } from "@/lib/utils"
import { PhotoUpload, type PhotoPlacement } from "@/components/PhotoUpload"
import type { SigningMethod, EncryptionMethod } from "@/components/UnifiedPlayground"

interface IdentityPanelProps {
  claim169: Claim169Input
  onClaim169Change: (value: Claim169Input) => void
  cwtMeta: CwtMetaInput
  onCwtMetaChange: (value: CwtMetaInput) => void
  encodedSize?: number
  signingMethod: SigningMethod
  onSigningMethodChange: (method: SigningMethod) => void
  privateKey: string
  onPrivateKeyChange: (key: string) => void
  publicKey: string
  onPublicKeyChange: (key: string) => void
  encryptionMethod: EncryptionMethod
  onEncryptionMethodChange: (method: EncryptionMethod) => void
  encryptionKey: string
  onEncryptionKeyChange: (key: string) => void
  onLoadSample: () => void
  onLoadExample: (key: string) => void
  samplePhotoUrl?: string | null
}

export function IdentityPanel({
  claim169,
  onClaim169Change,
  cwtMeta,
  onCwtMetaChange,
  encodedSize,
  signingMethod,
  onSigningMethodChange,
  privateKey,
  onPrivateKeyChange,
  publicKey,
  onPublicKeyChange,
  encryptionMethod,
  onEncryptionMethodChange,
  encryptionKey,
  onEncryptionKeyChange,
  onLoadSample,
  onLoadExample,
  samplePhotoUrl,
}: IdentityPanelProps) {
  const { t } = useTranslation()
  const [showCwtMeta, setShowCwtMeta] = useState(false)
  const [showAdvanced, setShowAdvanced] = useState(false)
  const [photoPlacement, setPhotoPlacement] = useState<PhotoPlacement>("photo")
  const [showPrivateKey, setShowPrivateKey] = useState(false)
  const [publicKeyCopied, setPublicKeyCopied] = useState(false)
  const [isGeneratingKeys, setIsGeneratingKeys] = useState(false)

  const handleExampleChange = (value: string) => {
    if (value === "demo") {
      onLoadSample()
    } else if (value) {
      onLoadExample(value)
    }
  }

  const updateClaim = (field: keyof Claim169Input, value: string | number | undefined) => {
    onClaim169Change({ ...claim169, [field]: value })
  }

  const claim169Ref = useRef(claim169)
  claim169Ref.current = claim169
  const photoPlacementRef = useRef(photoPlacement)
  photoPlacementRef.current = photoPlacement
  const handlePhotoChange = useCallback(
    (photo: Uint8Array | undefined, format: number | undefined) => {
      if (photoPlacementRef.current === "face") {
        // Route to face biometric field, clearing demographic photo fields
        const face = photo && photo.length > 0
          ? [{ data: photo, format: 0, subFormat: format }]
          : undefined
        onClaim169Change({
          ...claim169Ref.current,
          photo: undefined,
          photoFormat: undefined,
          face: face as Claim169Input["face"],
        })
      } else {
        // Route to demographic photo field, clearing face biometric
        onClaim169Change({
          ...claim169Ref.current,
          photo,
          photoFormat: format,
          face: undefined,
        })
      }
    },
    [onClaim169Change],
  )

  const handlePhotoPlacementChange = useCallback(
    (placement: PhotoPlacement) => {
      setPhotoPlacement(placement)
      // Re-route existing photo data to the selected field
      const current = claim169Ref.current
      const existingPhoto = current.photo
      const existingFormat = current.photoFormat
      const existingFace = current.face
      if (placement === "face" && existingPhoto && existingPhoto.length > 0) {
        // Move from photo field to face biometric
        const face = [{ data: existingPhoto, format: 0, subFormat: existingFormat }]
        onClaim169Change({
          ...current,
          photo: undefined,
          photoFormat: undefined,
          face: face as Claim169Input["face"],
        })
      } else if (placement === "photo" && existingFace && existingFace.length > 0) {
        // Move from face biometric to photo field
        const faceEntry = existingFace[0]
        onClaim169Change({
          ...current,
          photo: faceEntry.data,
          photoFormat: faceEntry.subFormat,
          face: undefined,
        })
      }
    },
    [onClaim169Change],
  )

  const updateMeta = (field: keyof CwtMetaInput, value: string | number | undefined) => {
    onCwtMetaChange({ ...cwtMeta, [field]: value })
  }

  const setNow = () => {
    updateMeta("issuedAt", Math.floor(Date.now() / 1000))
  }

  const setExpires = (days: number) => {
    updateMeta("expiresAt", Math.floor(Date.now() / 1000) + days * 24 * 60 * 60)
  }

  const handleCopyPublicKey = async () => {
    if (publicKey) {
      await copyToClipboard(publicKey)
      setPublicKeyCopied(true)
      setTimeout(() => setPublicKeyCopied(false), 2000)
    }
  }

  const handleGenerateKeys = async () => {
    setIsGeneratingKeys(true)
    try {
      if (signingMethod === "ed25519") {
        const keyPair = await generateEd25519KeyPair()
        onPrivateKeyChange(keyPair.privateKey)
        onPublicKeyChange(keyPair.publicKey)
      } else if (signingMethod === "ecdsa") {
        const keyPair = await generateEcdsaP256KeyPair()
        onPrivateKeyChange(keyPair.privateKey)
        onPublicKeyChange(keyPair.publicKey)
      }
    } finally {
      setIsGeneratingKeys(false)
    }
  }

  const handleGenerateEncryptionKey = () => {
    const bits = encryptionMethod === "aes256" ? 256 : 128
    const key = generateAesKey(bits)
    onEncryptionKeyChange(key)
  }

  return (
    <div className="space-y-4">
      {/* Identity Data Section - Always expanded */}
      <div className="space-y-3 p-4 rounded-lg border-l-4 border-l-green-500 bg-green-50/50 dark:bg-green-950/20">
        <div className="flex items-center justify-between">
          <h3 className="font-semibold text-green-800 dark:text-green-200">{t("identity.title")}</h3>
          <Select
            value=""
            onChange={(e) => handleExampleChange(e.target.value)}
            className="w-44 text-sm"
          >
            <option value="">{t("examples.loadExample")}</option>
            <option value="demo">{t("examples.demoIdentity")}</option>
            <option value="refugee-identity">{t("examples.refugeeIdentity")}</option>
          </Select>
        </div>

        {/* Essential fields - always visible */}
        <div className="grid grid-cols-2 gap-3">
          <div className="space-y-1">
            <Label htmlFor="id" className="text-xs">{t("identity.id")}</Label>
            <Input
              id="id"
              placeholder={t("identity.idPlaceholder")}
              value={claim169.id || ""}
              onChange={(e) => updateClaim("id", e.target.value || undefined)}
              className="text-sm h-8"
            />
          </div>
          <div className="space-y-1">
            <Label htmlFor="fullName" className="text-xs">{t("identity.fullName")}</Label>
            <Input
              id="fullName"
              placeholder={t("identity.fullNamePlaceholder")}
              value={claim169.fullName || ""}
              onChange={(e) => updateClaim("fullName", e.target.value || undefined)}
              className="text-sm h-8"
            />
          </div>
          <div className="space-y-1">
            <Label htmlFor="dob" className="text-xs">{t("identity.dateOfBirth")}</Label>
            <Input
              id="dob"
              type="date"
              value={claim169.dateOfBirth || ""}
              onChange={(e) => updateClaim("dateOfBirth", e.target.value || undefined)}
              className="text-sm h-8"
            />
          </div>
          <div className="space-y-1">
            <Label htmlFor="gender" className="text-xs">{t("identity.gender")}</Label>
            <Select
              id="gender"
              value={String(claim169.gender || 0)}
              onChange={(e) => updateClaim("gender", Number(e.target.value) || undefined)}
              className="text-sm h-8"
            >
              <option value="0">{t("identity.genderOptions.notSpecified")}</option>
              <option value="1">{t("identity.genderOptions.male")}</option>
              <option value="2">{t("identity.genderOptions.female")}</option>
              <option value="3">{t("identity.genderOptions.other")}</option>
            </Select>
          </div>
        </div>

        {/* Photo upload — resolve effective photo bytes from either field */}
        <PhotoUpload
          photo={
            photoPlacement === "face" && claim169.face && claim169.face.length > 0
              ? claim169.face[0].data
              : claim169.photo
          }
          photoFormat={
            photoPlacement === "face" && claim169.face && claim169.face.length > 0
              ? claim169.face[0].subFormat
              : claim169.photoFormat
          }
          onPhotoChange={handlePhotoChange}
          encodedSize={encodedSize}
          samplePhotoUrl={samplePhotoUrl}
          photoPlacement={photoPlacement}
          onPhotoPlacementChange={handlePhotoPlacementChange}
        />

        {/* Advanced fields - collapsible */}
        <Button
          variant="ghost"
          size="sm"
          className="w-full justify-start px-0 h-8 text-muted-foreground hover:text-foreground"
          onClick={() => setShowAdvanced(!showAdvanced)}
        >
          {showAdvanced ? <ChevronDown className="h-4 w-4 mr-2" /> : <ChevronRight className="h-4 w-4 mr-2" />}
          {t("identity.moreFields")}
        </Button>

        {showAdvanced && (
          <div className="grid grid-cols-2 gap-3 pt-2 border-t border-green-200 dark:border-green-800">
            <div className="space-y-1">
              <Label htmlFor="firstName" className="text-xs">{t("identity.firstName")}</Label>
              <Input
                id="firstName"
                value={claim169.firstName || ""}
                onChange={(e) => updateClaim("firstName", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="lastName" className="text-xs">{t("identity.lastName")}</Label>
              <Input
                id="lastName"
                value={claim169.lastName || ""}
                onChange={(e) => updateClaim("lastName", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="language" className="text-xs">{t("identity.language")}</Label>
              <Input
                id="language"
                placeholder={t("identity.languagePlaceholder")}
                value={claim169.language || ""}
                onChange={(e) => updateClaim("language", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="secondaryLanguage" className="text-xs">{t("identity.secondaryLanguage")}</Label>
              <Input
                id="secondaryLanguage"
                placeholder={t("identity.secondaryLanguagePlaceholder")}
                value={claim169.secondaryLanguage || ""}
                onChange={(e) => updateClaim("secondaryLanguage", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="col-span-2 space-y-1">
              <Label htmlFor="secondaryFullName" className="text-xs">{t("identity.localName")}</Label>
              <Input
                id="secondaryFullName"
                placeholder={t("identity.localNamePlaceholder")}
                value={claim169.secondaryFullName || ""}
                onChange={(e) => updateClaim("secondaryFullName", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="email" className="text-xs">{t("identity.email")}</Label>
              <Input
                id="email"
                type="email"
                value={claim169.email || ""}
                onChange={(e) => updateClaim("email", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="phone" className="text-xs">{t("identity.phone")}</Label>
              <Input
                id="phone"
                value={claim169.phone || ""}
                onChange={(e) => updateClaim("phone", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="nationality" className="text-xs">{t("identity.nationality")}</Label>
              <Input
                id="nationality"
                placeholder={t("identity.nationalityPlaceholder")}
                value={claim169.nationality || ""}
                onChange={(e) => updateClaim("nationality", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="maritalStatus" className="text-xs">{t("identity.maritalStatus")}</Label>
              <Select
                id="maritalStatus"
                value={String(claim169.maritalStatus || 0)}
                onChange={(e) => updateClaim("maritalStatus", Number(e.target.value) || undefined)}
                className="text-sm h-8"
              >
                <option value="0">{t("identity.maritalOptions.notSpecified")}</option>
                <option value="1">{t("identity.maritalOptions.unmarried")}</option>
                <option value="2">{t("identity.maritalOptions.married")}</option>
                <option value="3">{t("identity.maritalOptions.divorced")}</option>
              </Select>
            </div>
            <div className="col-span-2 space-y-1">
              <Label htmlFor="address" className="text-xs">{t("identity.address")}</Label>
              <Textarea
                id="address"
                placeholder={t("identity.addressPlaceholder")}
                value={claim169.address || ""}
                onChange={(e) => updateClaim("address", e.target.value || undefined)}
                className="text-sm min-h-[60px] resize-none"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="locationCode" className="text-xs">{t("identity.locationCode")}</Label>
              <Input
                id="locationCode"
                placeholder={t("identity.locationCodePlaceholder")}
                value={claim169.locationCode || ""}
                onChange={(e) => updateClaim("locationCode", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="legalStatus" className="text-xs">{t("identity.legalStatus")}</Label>
              <Input
                id="legalStatus"
                placeholder={t("identity.legalStatusPlaceholder")}
                value={claim169.legalStatus || ""}
                onChange={(e) => updateClaim("legalStatus", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="countryOfIssuance" className="text-xs">{t("identity.countryOfIssuance")}</Label>
              <Input
                id="countryOfIssuance"
                placeholder={t("identity.countryOfIssuancePlaceholder")}
                value={claim169.countryOfIssuance || ""}
                onChange={(e) => updateClaim("countryOfIssuance", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
          </div>
        )}
      </div>

      {/* Credential Settings - groups Token and Crypto */}
      <div className="rounded-lg border-2 border-dashed border-muted-foreground/25 p-3 space-y-3">
        <h3 className="text-xs font-medium text-muted-foreground uppercase tracking-wide">{t("credential.title")}</h3>

        {/* CWT Metadata Section - Collapsed by default */}
        <div className="rounded-lg border border-blue-200 bg-blue-50/50 dark:border-blue-900 dark:bg-blue-950/20">
        <Button
          variant="ghost"
          className="w-full justify-between px-4 py-3 h-auto"
          onClick={() => setShowCwtMeta(!showCwtMeta)}
        >
          <div className="flex items-center gap-2">
            {showCwtMeta ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
            <span className="font-medium text-blue-800 dark:text-blue-200">{t("cwt.title")}</span>
          </div>
          <span className="text-xs text-muted-foreground">
            {cwtMeta.issuer ? t("cwt.configured") : t("cwt.optional")}
          </span>
        </Button>

        {showCwtMeta && (
          <div className="px-4 pb-4 space-y-3 border-t border-blue-200 dark:border-blue-800">
            <div className="pt-3 space-y-1">
              <Label htmlFor="issuer" className="text-xs">{t("cwt.issuer")}</Label>
              <Input
                id="issuer"
                placeholder={t("cwt.issuerPlaceholder")}
                value={cwtMeta.issuer || ""}
                onChange={(e) => updateMeta("issuer", e.target.value || undefined)}
                className="text-sm h-8"
              />
            </div>
            <div className="grid grid-cols-2 gap-3">
              <div className="space-y-1">
                <Label htmlFor="subject" className="text-xs">{t("cwt.subject")}</Label>
                <Input
                  id="subject"
                  placeholder={t("cwt.subjectPlaceholder")}
                  value={cwtMeta.subject || ""}
                  onChange={(e) => updateMeta("subject", e.target.value || undefined)}
                  className="text-sm h-8"
                />
              </div>
              <div className="space-y-1">
                <Label htmlFor="issuedAt" className="text-xs">{t("cwt.issuedAt")}</Label>
                <div className="flex gap-1">
                  <Input
                    id="issuedAt"
                    type="number"
                    placeholder="Unix timestamp"
                    value={cwtMeta.issuedAt || ""}
                    onChange={(e) => updateMeta("issuedAt", e.target.value ? Number(e.target.value) : undefined)}
                    className="text-sm h-8 font-mono"
                  />
                  <Button variant="outline" size="sm" onClick={setNow} className="h-8 px-2 text-xs shrink-0">
                    {t("cwt.now")}
                  </Button>
                </div>
              </div>
              <div className="col-span-2 space-y-1">
                <Label htmlFor="expiresAt" className="text-xs">{t("cwt.expiresAt")}</Label>
                <div className="flex gap-1">
                  <Input
                    id="expiresAt"
                    type="number"
                    placeholder="Unix timestamp"
                    value={cwtMeta.expiresAt || ""}
                    onChange={(e) => updateMeta("expiresAt", e.target.value ? Number(e.target.value) : undefined)}
                    className="text-sm h-8 font-mono flex-1"
                  />
                  <Button variant="outline" size="sm" onClick={() => setExpires(30)} className="h-8 px-2 text-xs shrink-0">
                    {t("cwt.plus30d")}
                  </Button>
                  <Button variant="outline" size="sm" onClick={() => setExpires(365)} className="h-8 px-2 text-xs shrink-0">
                    {t("cwt.plus1y")}
                  </Button>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Cryptography Section */}
      <div className="space-y-3 p-4 rounded-lg border border-orange-200 bg-orange-50/50 dark:border-orange-900 dark:bg-orange-950/20">
        <h3 className="font-medium text-orange-800 dark:text-orange-200">{t("crypto.title")}</h3>

        {/* Signing Method */}
        <div className="space-y-2">
          <Label className="text-xs">{t("crypto.signing")}</Label>
          <div className="flex flex-wrap gap-3">
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="radio"
                name="signing"
                checked={signingMethod === "ed25519"}
                onChange={() => onSigningMethodChange("ed25519")}
                className="w-4 h-4"
              />
              <span className="text-sm">{t("crypto.ed25519")}</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="radio"
                name="signing"
                checked={signingMethod === "ecdsa"}
                onChange={() => onSigningMethodChange("ecdsa")}
                className="w-4 h-4"
              />
              <span className="text-sm">{t("crypto.ecdsaP256")}</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="radio"
                name="signing"
                checked={signingMethod === "unsigned"}
                onChange={() => onSigningMethodChange("unsigned")}
                className="w-4 h-4"
              />
              <span className="text-sm text-muted-foreground">{t("crypto.unsigned")}</span>
            </label>
          </div>
        </div>

        {/* Private Key */}
        {signingMethod !== "unsigned" && (
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label htmlFor="privateKey" className="text-xs">{t("crypto.privateKey")}</Label>
              <Button
                variant="outline"
                size="sm"
                onClick={handleGenerateKeys}
                disabled={isGeneratingKeys}
                className="h-6 px-2"
              >
                <RefreshCw className={`h-3 w-3 mr-1 ${isGeneratingKeys ? "animate-spin" : ""}`} />
                <span className="text-xs">{t("crypto.generateKeys")}</span>
              </Button>
            </div>
            <div className="relative">
              <Input
                id="privateKey"
                type={showPrivateKey ? "text" : "password"}
                placeholder={t("crypto.privateKeyPlaceholder")}
                value={privateKey}
                onChange={(e) => onPrivateKeyChange(e.target.value)}
                className="font-mono text-xs pr-10"
              />
              <Button
                variant="ghost"
                size="sm"
                className="absolute right-1 top-1/2 -translate-y-1/2 h-7 w-7 p-0"
                onClick={() => setShowPrivateKey(!showPrivateKey)}
              >
                {showPrivateKey ? <EyeOff className="h-3 w-3" /> : <Eye className="h-3 w-3" />}
              </Button>
            </div>
          </div>
        )}

        {/* Public Key (for signing or verification) */}
        {signingMethod !== "unsigned" && (
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Label htmlFor="publicKey" className="text-xs">{t("crypto.publicKey")}</Label>
                {publicKey && (
                  <span className={`inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-medium ${
                    detectPublicKeyFormat(publicKey) === 'pem'
                      ? 'bg-purple-100 text-purple-800 dark:bg-purple-900/50 dark:text-purple-300'
                      : detectPublicKeyFormat(publicKey) === 'hex'
                      ? 'bg-blue-100 text-blue-800 dark:bg-blue-900/50 dark:text-blue-300'
                      : 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300'
                  }`}>
                    {t(`keyFormat.${detectPublicKeyFormat(publicKey)}`)}
                  </span>
                )}
              </div>
              {publicKey && (
                <Button variant="ghost" size="sm" onClick={handleCopyPublicKey} className="h-6 px-2">
                  {publicKeyCopied ? <Check className="h-3 w-3 mr-1" /> : <Copy className="h-3 w-3 mr-1" />}
                  <span className="text-xs">{t("common.copy")}</span>
                </Button>
              )}
            </div>
            <Textarea
              id="publicKey"
              placeholder={t("crypto.publicKeyPlaceholder")}
              value={publicKey}
              onChange={(e) => onPublicKeyChange(e.target.value)}
              className="font-mono text-xs min-h-[60px] resize-y"
            />
            <p className="text-xs text-muted-foreground">{t("crypto.publicKeyHelpVerify")}</p>
          </div>
        )}

        {/* Encryption */}
        <div className="space-y-2">
          <Label className="text-xs">{t("crypto.encryption")}</Label>
          <div className="flex flex-wrap gap-3">
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="radio"
                name="encryption"
                checked={encryptionMethod === "none"}
                onChange={() => onEncryptionMethodChange("none")}
                className="w-4 h-4"
              />
              <span className="text-sm">{t("crypto.none")}</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="radio"
                name="encryption"
                checked={encryptionMethod === "aes256"}
                onChange={() => onEncryptionMethodChange("aes256")}
                className="w-4 h-4"
              />
              <span className="text-sm">{t("crypto.aes256")}</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="radio"
                name="encryption"
                checked={encryptionMethod === "aes128"}
                onChange={() => onEncryptionMethodChange("aes128")}
                className="w-4 h-4"
              />
              <span className="text-sm">{t("crypto.aes128")}</span>
            </label>
          </div>
        </div>

        {encryptionMethod !== "none" && (
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Label htmlFor="encryptionKey" className="text-xs">{t("crypto.encryptionKey")}</Label>
                {encryptionKey && (
                  <span className={`inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-medium ${
                    detectEncryptionKeyFormat(encryptionKey) === 'base64'
                      ? 'bg-green-100 text-green-800 dark:bg-green-900/50 dark:text-green-300'
                      : detectEncryptionKeyFormat(encryptionKey) === 'hex'
                      ? 'bg-blue-100 text-blue-800 dark:bg-blue-900/50 dark:text-blue-300'
                      : 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300'
                  }`}>
                    {t(`keyFormat.${detectEncryptionKeyFormat(encryptionKey)}`)}
                  </span>
                )}
              </div>
              <Button
                variant="outline"
                size="sm"
                onClick={handleGenerateEncryptionKey}
                className="h-6 px-2"
              >
                <RefreshCw className="h-3 w-3 mr-1" />
                <span className="text-xs">{t("crypto.generateEncryptionKey")}</span>
              </Button>
            </div>
            <Input
              id="encryptionKey"
              type="password"
              placeholder={encryptionMethod === "aes256" ? "32 bytes (64 hex or 44 base64)" : "16 bytes (32 hex or 24 base64)"}
              value={encryptionKey}
              onChange={(e) => onEncryptionKeyChange(e.target.value)}
              className="font-mono text-xs"
            />
          </div>
        )}

        {/* Security Warning */}
        <div className="flex items-start gap-2 p-2 rounded bg-amber-100 dark:bg-amber-900/30 text-amber-800 dark:text-amber-200 text-xs">
          <AlertTriangle className="h-4 w-4 shrink-0 mt-0.5" />
          <span>{t("crypto.securityWarning")}</span>
        </div>
        </div>
      </div>
    </div>
  )
}
