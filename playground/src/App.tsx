import { useState } from "react"
import { Header } from "@/components/Header"
import { DecodePanel } from "@/components/DecodePanel"
import { EncodePanel } from "@/components/EncodePanel"
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs"

function App() {
  const [activeTab, setActiveTab] = useState("decode")

  return (
    <div className="min-h-screen flex flex-col">
      <Header />
      <main className="flex-1 container px-4 py-6">
        <Tabs value={activeTab} onValueChange={setActiveTab}>
          <TabsList className="mb-6">
            <TabsTrigger value="decode">Decode</TabsTrigger>
            <TabsTrigger value="encode">Encode</TabsTrigger>
          </TabsList>
          <TabsContent value="decode">
            <DecodePanel />
          </TabsContent>
          <TabsContent value="encode">
            <EncodePanel />
          </TabsContent>
        </Tabs>
      </main>
      <footer className="border-t py-4 text-center text-sm text-muted-foreground">
        <p>
          Built with Rust + WebAssembly |{" "}
          <a
            href="https://github.com/mosip/id-claim-169"
            target="_blank"
            rel="noopener noreferrer"
            className="underline hover:text-foreground"
          >
            MOSIP Claim 169 Spec
          </a>
        </p>
      </footer>
    </div>
  )
}

export default App
