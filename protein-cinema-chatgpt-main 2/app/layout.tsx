import type { Metadata, Viewport } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "ProteinTrace - O Teatro da Descoberta Científica",
  description: "Laboratório virtual onde cada simulação é uma performance científica, e cada resultado um manifesto assinado de evidência. Predição de estrutura proteica com AlphaFold, análise pLDDT, e visualização 3D interativa.",
  keywords: ["proteína", "biologia computacional", "AlphaFold", "estrutura proteica", "pLDDT", "CRISPR", "mutação"],
  authors: [{ name: "Daniel Camarilho" }],
  openGraph: {
    title: "ProteinTrace - O Teatro da Descoberta Científica",
    description: "Laboratório virtual onde cada simulação é uma performance científica",
    type: "website",
  },
};

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
  maximumScale: 5,
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="pt-BR">
      <body className="font-sans antialiased">
        {children}
      </body>
    </html>
  );
}
