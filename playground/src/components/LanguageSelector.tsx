import { useTranslation } from "react-i18next"
import { Select } from "@/components/ui/select"
import { supportedLanguages, isRtl } from "@/i18n"
import { Globe } from "lucide-react"

export function LanguageSelector() {
  const { i18n } = useTranslation()

  const handleLanguageChange = (lang: string) => {
    i18n.changeLanguage(lang)
    // Update document direction for RTL languages
    document.documentElement.dir = isRtl(lang) ? "rtl" : "ltr"
    document.documentElement.lang = lang
  }

  return (
    <div className="flex items-center gap-2">
      <Globe className="h-4 w-4 text-muted-foreground" />
      <Select
        value={i18n.language}
        onChange={(e) => handleLanguageChange(e.target.value)}
        className="w-28 text-sm h-8"
      >
        {supportedLanguages.map((lang) => (
          <option key={lang.code} value={lang.code}>
            {lang.nativeName}
          </option>
        ))}
      </Select>
    </div>
  )
}
