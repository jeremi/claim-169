import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { AlertCircle, ChevronDown, ChevronRight, Lightbulb } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'
import { type ParsedError, getStageDisplayName } from '@/lib/errors'

interface ErrorDisplayProps {
  error: ParsedError
  className?: string
}

export function ErrorDisplay({ error, className }: ErrorDisplayProps) {
  const { t } = useTranslation()
  const [showDetails, setShowDetails] = useState(false)

  return (
    <div
      className={cn(
        'rounded-lg border-2 border-red-200 dark:border-red-900 overflow-hidden',
        className
      )}
    >
      {/* Error header with stage badge */}
      <div className="bg-red-50 dark:bg-red-950/30 px-4 py-3 flex items-start gap-3">
        <AlertCircle className="h-5 w-5 text-red-500 mt-0.5 flex-shrink-0" />
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 flex-wrap">
            <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900/50 dark:text-red-300">
              {getStageDisplayName(error.stage)}
            </span>
            <span className="text-sm font-medium text-red-700 dark:text-red-400">
              {t('error.stageFailed', { stage: getStageDisplayName(error.stage) })}
            </span>
          </div>
          <p className="mt-1 text-sm text-red-600 dark:text-red-400">{error.message}</p>
        </div>
      </div>

      {/* Suggestion box */}
      {error.suggestion && (
        <div className="bg-amber-50 dark:bg-amber-950/30 px-4 py-3 border-t border-amber-200 dark:border-amber-900/50">
          <div className="flex items-start gap-2">
            <Lightbulb className="h-4 w-4 text-amber-500 mt-0.5 flex-shrink-0" />
            <div>
              <span className="text-xs font-medium text-amber-700 dark:text-amber-400 uppercase tracking-wide">
                {t('error.suggestion')}
              </span>
              <p className="text-sm text-amber-700 dark:text-amber-300 mt-0.5">
                {error.suggestion}
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Expandable technical details */}
      <div className="border-t border-red-200 dark:border-red-900/50">
        <Button
          variant="ghost"
          className="w-full justify-start px-4 py-2 h-auto text-xs text-muted-foreground hover:text-foreground"
          onClick={() => setShowDetails(!showDetails)}
        >
          {showDetails ? (
            <ChevronDown className="h-3 w-3 mr-1" />
          ) : (
            <ChevronRight className="h-3 w-3 mr-1" />
          )}
          {t('error.technicalDetails')}
        </Button>

        {showDetails && (
          <div className="px-4 pb-3">
            <pre className="text-xs bg-muted/50 dark:bg-muted/20 p-2 rounded overflow-x-auto whitespace-pre-wrap break-all font-mono text-muted-foreground">
              {error.originalMessage}
            </pre>
          </div>
        )}
      </div>
    </div>
  )
}
