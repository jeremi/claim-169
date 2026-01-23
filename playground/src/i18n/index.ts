import i18n from "i18next"
import { initReactI18next } from "react-i18next"

import en from "./locales/en.json"
import fr from "./locales/fr.json"

// Detect browser language
const getBrowserLanguage = (): string => {
  const lang = navigator.language.split("-")[0]
  return ["en", "fr"].includes(lang) ? lang : "en"
}

i18n
  .use(initReactI18next)
  .init({
    resources: {
      en: { translation: en },
      fr: { translation: fr },
    },
    lng: getBrowserLanguage(),
    fallbackLng: "en",
    interpolation: {
      escapeValue: false, // React already escapes values
    },
  })

export default i18n

// RTL languages for future Arabic support
export const RTL_LANGUAGES = ["ar", "he", "fa", "ur"]

export const isRtl = (lang: string): boolean => {
  return RTL_LANGUAGES.includes(lang)
}

export const supportedLanguages = [
  { code: "en", name: "English", nativeName: "English" },
  { code: "fr", name: "French", nativeName: "Français" },
  // Ready for Arabic when translations are added:
  // { code: "ar", name: "Arabic", nativeName: "العربية", rtl: true },
]
