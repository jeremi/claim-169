import { useState } from "react"
import type { DecodeResult } from "claim169"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Copy, Check, ChevronDown, ChevronRight } from "lucide-react"
import {
  formatTimestamp,
  isExpired,
  GENDER_MAP,
  MARITAL_STATUS_MAP,
  copyToClipboard,
} from "@/lib/utils"

interface ResultViewerProps {
  result: DecodeResult | null
  error: string | null
}

export function ResultViewer({ result, error }: ResultViewerProps) {
  const [copied, setCopied] = useState(false)
  const [showJson, setShowJson] = useState(false)

  if (error) {
    return (
      <Card className="border-destructive">
        <CardHeader>
          <CardTitle className="text-destructive flex items-center gap-2">
            Decode Error
          </CardTitle>
        </CardHeader>
        <CardContent>
          <pre className="text-sm bg-destructive/10 p-4 rounded-md overflow-auto whitespace-pre-wrap">
            {error}
          </pre>
        </CardContent>
      </Card>
    )
  }

  if (!result) {
    return (
      <Card>
        <CardContent className="py-12 text-center text-muted-foreground">
          <p>Enter QR data and click Decode to see results</p>
        </CardContent>
      </Card>
    )
  }

  const { claim169, cwtMeta, verificationStatus } = result
  const expired = isExpired(cwtMeta.expiresAt)

  const handleCopy = async () => {
    await copyToClipboard(JSON.stringify(result, null, 2))
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader className="pb-3">
          <div className="flex items-center justify-between">
            <CardTitle className="text-lg">Verification Status</CardTitle>
            <Badge
              variant={
                verificationStatus === "verified"
                  ? "success"
                  : verificationStatus === "skipped"
                  ? "warning"
                  : "destructive"
              }
            >
              {verificationStatus === "verified" && "✓ "}
              {verificationStatus}
            </Badge>
          </div>
        </CardHeader>
        {expired && (
          <CardContent className="pt-0">
            <Badge variant="destructive">Credential Expired</Badge>
          </CardContent>
        )}
      </Card>

      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-lg">Identity Data</CardTitle>
        </CardHeader>
        <CardContent>
          <dl className="grid grid-cols-2 gap-x-4 gap-y-2 text-sm">
            {claim169.id && (
              <>
                <dt className="text-muted-foreground">ID</dt>
                <dd className="font-mono">{claim169.id}</dd>
              </>
            )}
            {claim169.fullName && (
              <>
                <dt className="text-muted-foreground">Full Name</dt>
                <dd>{claim169.fullName}</dd>
              </>
            )}
            {claim169.firstName && (
              <>
                <dt className="text-muted-foreground">First Name</dt>
                <dd>{claim169.firstName}</dd>
              </>
            )}
            {claim169.lastName && (
              <>
                <dt className="text-muted-foreground">Last Name</dt>
                <dd>{claim169.lastName}</dd>
              </>
            )}
            {claim169.dateOfBirth && (
              <>
                <dt className="text-muted-foreground">Date of Birth</dt>
                <dd>{claim169.dateOfBirth}</dd>
              </>
            )}
            {claim169.gender !== undefined && (
              <>
                <dt className="text-muted-foreground">Gender</dt>
                <dd>{GENDER_MAP[claim169.gender] || claim169.gender}</dd>
              </>
            )}
            {claim169.email && (
              <>
                <dt className="text-muted-foreground">Email</dt>
                <dd className="break-all">{claim169.email}</dd>
              </>
            )}
            {claim169.phone && (
              <>
                <dt className="text-muted-foreground">Phone</dt>
                <dd>{claim169.phone}</dd>
              </>
            )}
            {claim169.address && (
              <>
                <dt className="text-muted-foreground">Address</dt>
                <dd className="col-span-2 mt-1">{claim169.address}</dd>
              </>
            )}
            {claim169.nationality && (
              <>
                <dt className="text-muted-foreground">Nationality</dt>
                <dd>{claim169.nationality}</dd>
              </>
            )}
            {claim169.maritalStatus !== undefined && (
              <>
                <dt className="text-muted-foreground">Marital Status</dt>
                <dd>{MARITAL_STATUS_MAP[claim169.maritalStatus] || claim169.maritalStatus}</dd>
              </>
            )}
            {claim169.guardian && (
              <>
                <dt className="text-muted-foreground">Guardian</dt>
                <dd>{claim169.guardian}</dd>
              </>
            )}
          </dl>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-lg">CWT Metadata</CardTitle>
        </CardHeader>
        <CardContent>
          <dl className="grid grid-cols-2 gap-x-4 gap-y-2 text-sm">
            {cwtMeta.issuer && (
              <>
                <dt className="text-muted-foreground">Issuer</dt>
                <dd className="break-all font-mono text-xs">{cwtMeta.issuer}</dd>
              </>
            )}
            {cwtMeta.subject && (
              <>
                <dt className="text-muted-foreground">Subject</dt>
                <dd className="font-mono">{cwtMeta.subject}</dd>
              </>
            )}
            {cwtMeta.issuedAt && (
              <>
                <dt className="text-muted-foreground">Issued At</dt>
                <dd>{formatTimestamp(cwtMeta.issuedAt)}</dd>
              </>
            )}
            {cwtMeta.expiresAt && (
              <>
                <dt className="text-muted-foreground">Expires At</dt>
                <dd className={expired ? "text-destructive" : ""}>
                  {formatTimestamp(cwtMeta.expiresAt)}
                </dd>
              </>
            )}
            {cwtMeta.notBefore && (
              <>
                <dt className="text-muted-foreground">Not Before</dt>
                <dd>{formatTimestamp(cwtMeta.notBefore)}</dd>
              </>
            )}
          </dl>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-3">
          <div className="flex items-center justify-between">
            <button
              onClick={() => setShowJson(!showJson)}
              className="flex items-center gap-2 text-lg font-semibold"
            >
              {showJson ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
              Raw JSON
            </button>
            <Button variant="ghost" size="sm" onClick={handleCopy}>
              {copied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
            </Button>
          </div>
        </CardHeader>
        {showJson && (
          <CardContent>
            <pre className="text-xs bg-muted p-4 rounded-md overflow-auto max-h-96">
              {JSON.stringify(result, null, 2)}
            </pre>
          </CardContent>
        )}
      </Card>
    </div>
  )
}
