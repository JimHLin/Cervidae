import type { Metadata } from "next";
import './globals.css'
import ClientProvider from "@/ui/client-provider";
import { AuthProvider } from "@/ui/auth-provider";
import Header from "@/ui/header";
import { TanstackProvider } from "@/ui/tanstack-provider";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
          <TanstackProvider>
          <ClientProvider>
          <AuthProvider>
            <Header />
        <div className="flex flex-col justify-between min-h-screen">
        <div className="pt-10">
            {children}
          
        </div>
        <div className="flex flex-row gap-4 h-10 bg-green-800 w-full mt-10">
        This is a footer
        </div>
        </div>
        </AuthProvider>
    </ClientProvider>
    </TanstackProvider>
      </body>
    </html>
    
  )
}