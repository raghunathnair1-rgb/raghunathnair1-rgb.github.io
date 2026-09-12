import "./globals.css";

export const metadata = {
  title: "Formline — Movement Lab",
  description:
    "Calisthenics movement lab with simulated sessions, form trends, a move library, and session reports.",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
