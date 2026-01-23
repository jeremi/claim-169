import { useTranslation } from "react-i18next"
import { Github, BookOpen, Shield } from "lucide-react"
import { buttonVariants } from "@/components/ui/button"
import { version } from "claim169"
import { cn } from "@/lib/utils"
import { LanguageSelector } from "@/components/LanguageSelector"

export function Header() {
  const { t } = useTranslation()

  return (
    <header className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container flex h-14 items-center justify-between px-4">
        <div className="flex items-center gap-2">
          <Shield className="h-6 w-6" />
          <span className="font-bold text-lg">{t("app.title")}</span>
          <span className="text-xs text-muted-foreground bg-muted px-2 py-0.5 rounded">
            v{version()}
          </span>
        </div>
        <nav className="flex items-center gap-2">
          <LanguageSelector />
          <a
            href="https://github.com/jeremi/claim-169"
            target="_blank"
            rel="noopener noreferrer"
            className={cn(buttonVariants({ variant: "ghost", size: "sm" }), "flex items-center gap-2")}
          >
            <Github className="h-4 w-4" />
            <span className="hidden sm:inline">GitHub</span>
          </a>
          <a
            href="./docs/"
            className={cn(buttonVariants({ variant: "ghost", size: "sm" }), "flex items-center gap-2")}
          >
            <BookOpen className="h-4 w-4" />
            <span className="hidden sm:inline">Docs</span>
          </a>
        </nav>
      </div>
    </header>
  )
}
