import { useState } from "react"
import { useTranslation } from "react-i18next"
import { ChevronDown, ChevronRight, Check, AlertCircle } from "lucide-react"
import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"

export interface PipelineStage {
  name: string
  inputSize: number
  outputSize: number
  status: "success" | "error" | "skipped" | "pending"
  details?: Record<string, string | number | boolean>
  hexData?: string
}

interface PipelineDetailsProps {
  stages: PipelineStage[]
  className?: string
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes}B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)}KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)}MB`
}

function StageRow({ stage, isLast }: { stage: PipelineStage; isLast: boolean }) {
  const [expanded, setExpanded] = useState(false)

  const hasExpandableContent = stage.details || stage.hexData

  return (
    <div className="relative">
      <div
        className={cn(
          "flex items-center gap-2 py-2 px-3 rounded-md transition-colors",
          hasExpandableContent && "cursor-pointer hover:bg-muted/50",
          expanded && "bg-muted/30"
        )}
        onClick={() => hasExpandableContent && setExpanded(!expanded)}
      >
        {hasExpandableContent ? (
          expanded ? (
            <ChevronDown className="h-4 w-4 text-muted-foreground shrink-0" />
          ) : (
            <ChevronRight className="h-4 w-4 text-muted-foreground shrink-0" />
          )
        ) : (
          <div className="w-4" />
        )}

        <div className="flex items-center gap-2 flex-1 min-w-0">
          <span
            className={cn(
              "font-medium text-sm",
              stage.status === "success" && "text-foreground",
              stage.status === "error" && "text-red-500",
              stage.status === "skipped" && "text-muted-foreground"
            )}
          >
            {stage.name}
          </span>

          {stage.status === "success" && (
            <Check className="h-3.5 w-3.5 text-green-500 shrink-0" />
          )}
          {stage.status === "error" && (
            <AlertCircle className="h-3.5 w-3.5 text-red-500 shrink-0" />
          )}
        </div>

        {stage.outputSize > 0 && (
          <div className="text-xs text-muted-foreground tabular-nums shrink-0">
            {formatBytes(stage.outputSize)}
          </div>
        )}

        {!isLast && (
          <span className="text-muted-foreground shrink-0">→</span>
        )}
      </div>

      {expanded && hasExpandableContent && (
        <div className="ml-6 pl-4 border-l border-muted mb-2">
          {stage.details && (
            <div className="py-2 space-y-1">
              {Object.entries(stage.details).map(([key, value]) => (
                <div key={key} className="flex gap-2 text-xs">
                  <span className="text-muted-foreground">{key}:</span>
                  <span className="font-mono">
                    {typeof value === "boolean" ? (value ? "true" : "false") : String(value)}
                  </span>
                </div>
              ))}
            </div>
          )}
          {stage.hexData && (
            <div className="py-2">
              <pre className="text-xs font-mono bg-muted/50 p-2 rounded overflow-x-auto max-h-32">
                {stage.hexData}
              </pre>
            </div>
          )}
        </div>
      )}
    </div>
  )
}

export function PipelineDetails({ stages, className }: PipelineDetailsProps) {
  const { t } = useTranslation()
  const [isOpen, setIsOpen] = useState(false)

  if (stages.length === 0) return null

  // Calculate compression ratio from the zlib stage (where actual compression happens)
  // Only show when we have actual sizes (not estimates)
  const zlibStage = stages.find(s => s.name === "zlib")
  let compressionRatio = 0
  if (zlibStage && zlibStage.inputSize > 0 && zlibStage.outputSize > 0) {
    const compressed = Math.min(zlibStage.inputSize, zlibStage.outputSize)
    const uncompressed = Math.max(zlibStage.inputSize, zlibStage.outputSize)
    compressionRatio = Math.round((1 - compressed / uncompressed) * 100)
  }

  return (
    <div className={cn("rounded-lg border bg-card", className)}>
      <Button
        variant="ghost"
        className="w-full justify-between px-4 py-3 h-auto"
        onClick={() => setIsOpen(!isOpen)}
      >
        <div className="flex items-center gap-2">
          {isOpen ? (
            <ChevronDown className="h-4 w-4" />
          ) : (
            <ChevronRight className="h-4 w-4" />
          )}
          <span className="font-medium">{t("pipeline.title")}</span>
        </div>
        <div className="flex items-center gap-3 text-xs text-muted-foreground">
          <span>{stages.length} {t("pipeline.stages")}</span>
          {compressionRatio > 0 && (
            <span className="text-green-600 dark:text-green-400">
              {compressionRatio}% {t("pipeline.compression")}
            </span>
          )}
        </div>
      </Button>

      {isOpen && (
        <div className="px-4 pb-4 border-t">
          <div className="flex items-center gap-1 py-3 text-xs text-muted-foreground overflow-x-auto">
            {stages.map((stage, index) => (
              <span key={stage.name} className="flex items-center gap-1 whitespace-nowrap">
                <span className={cn(
                  stage.status === "success" && "text-foreground",
                  stage.status === "error" && "text-red-500"
                )}>
                  {stage.name}
                </span>
                {index < stages.length - 1 && <span>→</span>}
              </span>
            ))}
          </div>

          <div className="space-y-1">
            {stages.map((stage, index) => (
              <StageRow
                key={stage.name}
                stage={stage}
                isLast={index === stages.length - 1}
              />
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
