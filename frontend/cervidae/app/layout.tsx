import type { Metadata } from "next";
import './globals.css'
import ClientProvider from "@/ui/client_provider";
import Link from 'next/link';

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <div className="min-h-screen flex flex-col justify-between">
        <div className="flex flex-col items-center justify-center h-10 bg-green-800 fixed w-full z-50">
          <Link href="/" className="text-white font-serif text-2xl tracking-wider">Cervidae</Link>
        </div>
        <ClientProvider>
          {children}
        </ClientProvider>
        <div className="flex flex-row gap-4 h-10 bg-green-800 w-full mt-10">
        This is a footer
        </div>
        </div>
      </body>
    </html>
  )
}