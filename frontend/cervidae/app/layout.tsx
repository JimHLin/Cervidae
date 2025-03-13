import type { Metadata } from "next";
import './globals.css'
import ClientProvider from "@/ui/client-provider";
import { AuthProvider } from "@/ui/auth-provider";
import Header from "@/ui/header";
export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
        <html lang="en">
          <body>
          <ClientProvider>
          <AuthProvider>
            <Header />
        <div className="pt-10">
            {children}
          
        </div>
        <div className="flex flex-row gap-4 h-10 bg-green-800 w-full mt-10">
        This is a footer
        </div>
        </AuthProvider>
    </ClientProvider>
      </body>
    </html>
    
  )
}