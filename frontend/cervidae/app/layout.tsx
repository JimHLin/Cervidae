import type { Metadata } from "next";
import './globals.css'

export const metadata: Metadata = {
  title: "Cervidae",
  description: "Cervidae",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <div className="flex flex-col items-center justify-center h-10 bg-green-600">
          <h1 className="text-white">Cervidae</h1>
        </div>
        {children}
      </body>
    </html>
  )
}