import { useTranslation } from "react-i18next"
import { Header } from "@/components/Header"
import { UnifiedPlayground } from "@/components/UnifiedPlayground"

function App() {
  const { t } = useTranslation()

  return (
    <div className="min-h-screen flex flex-col">
      <Header />
      <main className="flex-1 container px-4 py-6">
        <UnifiedPlayground />
      </main>
      <footer className="border-t py-4 text-center text-sm text-muted-foreground">
        <p>{t("app.footer.privacy")}</p>
        <p className="mt-1">
          {t("app.footer.builtWith")} |{" "}
          <a
            href="https://github.com/mosip/id-claim-169"
            target="_blank"
            rel="noopener noreferrer"
            className="underline hover:text-foreground"
          >
            {t("app.footer.spec")}
          </a>
        </p>
      </footer>
    </div>
  )
}

export default App
