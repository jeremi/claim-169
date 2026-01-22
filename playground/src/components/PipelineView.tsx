import { useState } from "react"
import { Card, CardContent, CardHeader } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { ChevronDown, ChevronRight, ArrowRight } from "lucide-react"
import { bytesToHex } from "@/lib/utils"

interface PipelineStage {
  name: string
  description: string
  data: string | Uint8Array | null
  format: "text" | "hex" | "json"
}

interface PipelineViewProps {
  qrData: string | null
  decodedStages?: {
    base45?: string
    compressed?: Uint8Array
    cose?: Uint8Array
    cbor?: unknown
    claim169?: unknown
  }
}

export function PipelineView({ qrData, decodedStages }: PipelineViewProps) {
  const [expanded, setExpanded] = useState(false)
  const [selectedStage, setSelectedStage] = useState<number | null>(null)

  if (!qrData) {
    return null
  }

  const stages: PipelineStage[] = [
    {
      name: "QR Code",
      description: "Base45 encoded string",
      data: qrData,
      format: "text",
    },
    {
      name: "Base45",
      description: "Decoded to binary",
      data: decodedStages?.compressed || null,
      format: "hex",
    },
    {
      name: "zlib",
      description: "Decompressed",
      data: decodedStages?.cose || null,
      format: "hex",
    },
    {
      name: "COSE",
      description: "COSE_Sign1 structure",
      data: decodedStages?.cbor ? JSON.stringify(decodedStages.cbor, null, 2) : null,
      format: "json",
    },
    {
      name: "Claim 169",
      description: "Final decoded data",
      data: decodedStages?.claim169 ? JSON.stringify(decodedStages.claim169, null, 2) : null,
      format: "json",
    },
  ]

  const formatData = (stage: PipelineStage): string => {
    if (!stage.data) return "Not available"
    if (stage.data instanceof Uint8Array) {
      return bytesToHex(stage.data)
    }
    return String(stage.data)
  }

  return (
    <Card>
      <CardHeader className="pb-3">
        <button
          onClick={() => setExpanded(!expanded)}
          className="flex items-center gap-2 text-lg font-semibold w-full text-left"
        >
          {expanded ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
          Encoding Pipeline
        </button>
      </CardHeader>
      {expanded && (
        <CardContent>
          <div className="flex items-center justify-between gap-2 mb-4 overflow-x-auto pb-2">
            {stages.map((stage, index) => (
              <div key={stage.name} className="flex items-center">
                <Button
                  variant={selectedStage === index ? "default" : "outline"}
                  size="sm"
                  onClick={() => setSelectedStage(selectedStage === index ? null : index)}
                  className="whitespace-nowrap"
                >
                  {stage.name}
                </Button>
                {index < stages.length - 1 && (
                  <ArrowRight className="h-4 w-4 mx-2 text-muted-foreground flex-shrink-0" />
                )}
              </div>
            ))}
          </div>

          {selectedStage !== null && (
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <h4 className="font-medium">{stages[selectedStage].name}</h4>
                <span className="text-xs text-muted-foreground">
                  {stages[selectedStage].description}
                </span>
              </div>
              <pre className="text-xs bg-muted p-4 rounded-md overflow-auto max-h-64 break-all whitespace-pre-wrap font-mono">
                {formatData(stages[selectedStage])}
              </pre>
              {stages[selectedStage].data && (
                <p className="text-xs text-muted-foreground">
                  {stages[selectedStage].data instanceof Uint8Array
                    ? `${stages[selectedStage].data.length} bytes`
                    : `${String(stages[selectedStage].data).length} characters`}
                </p>
              )}
            </div>
          )}

          {selectedStage === null && (
            <p className="text-sm text-muted-foreground text-center py-4">
              Click a stage to view its data
            </p>
          )}

          <div className="mt-4 p-4 bg-muted/50 rounded-md">
            <h4 className="font-medium text-sm mb-2">Pipeline Overview</h4>
            <p className="text-xs text-muted-foreground">
              <strong>Encoding:</strong> Identity Data → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
            </p>
            <p className="text-xs text-muted-foreground mt-1">
              <strong>Decoding:</strong> QR Code → Base45 → zlib → [COSE_Encrypt0] → COSE_Sign1 → CWT → CBOR → Identity Data
            </p>
          </div>
        </CardContent>
      )}
    </Card>
  )
}
