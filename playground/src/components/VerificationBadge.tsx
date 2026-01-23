import { useTranslation } from "react-i18next"
import { CheckCircle2, XCircle, AlertCircle, ShieldQuestion } from "lucide-react"
import { cn } from "@/lib/utils"

export type VerificationStatus = "verified" | "unverified" | "invalid" | "none"

interface VerificationBadgeProps {
  status: VerificationStatus
  algorithm?: string
  keyId?: string
  error?: string
  className?: string
}

export function VerificationBadge({
  status,
  algorithm,
  keyId,
  error,
  className,
}: VerificationBadgeProps) {
  const { t } = useTranslation()

  const config = {
    verified: {
      icon: CheckCircle2,
      labelKey: "verification.verified",
      bgColor: "bg-green-500/10",
      borderColor: "border-green-500/30",
      textColor: "text-green-600 dark:text-green-400",
      iconColor: "text-green-500",
    },
    unverified: {
      icon: ShieldQuestion,
      labelKey: "verification.unverified",
      bgColor: "bg-yellow-500/10",
      borderColor: "border-yellow-500/30",
      textColor: "text-yellow-600 dark:text-yellow-400",
      iconColor: "text-yellow-500",
    },
    invalid: {
      icon: XCircle,
      labelKey: "verification.invalid",
      bgColor: "bg-red-500/10",
      borderColor: "border-red-500/30",
      textColor: "text-red-600 dark:text-red-400",
      iconColor: "text-red-500",
    },
    none: {
      icon: AlertCircle,
      labelKey: "verification.unsigned",
      bgColor: "bg-gray-500/10",
      borderColor: "border-gray-500/30",
      textColor: "text-gray-600 dark:text-gray-400",
      iconColor: "text-gray-500",
    },
  }

  const { icon: Icon, labelKey, bgColor, borderColor, textColor, iconColor } = config[status]

  return (
    <div
      className={cn(
        "rounded-lg border p-4",
        bgColor,
        borderColor,
        className
      )}
    >
      <div className="flex items-center gap-3">
        <Icon className={cn("h-6 w-6", iconColor)} />
        <div className="flex-1">
          <p className={cn("font-semibold", textColor)}>{t(labelKey)}</p>
          {algorithm && (
            <p className="text-sm text-muted-foreground">
              {t("verification.algorithm")}: {algorithm}
              {keyId && ` • Key ID: ${keyId}`}
            </p>
          )}
          {error && status === "invalid" && (
            <p className="text-sm text-red-500 mt-1">{error}</p>
          )}
        </div>
      </div>
    </div>
  )
}
