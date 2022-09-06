import { AppProps } from "next/app";
import { ThemeProvider } from "next-themes";
import { FlaggerClientProvider } from "../context";

import "../styles/globals.css";

export default function FlaggerUI(
  { Component, pageProps }: AppProps,
): JSX.Element {
  return (
    <ThemeProvider attribute="class">
      <FlaggerClientProvider baseUrl="http://localhost:3333">
        <Component {...pageProps} />
      </FlaggerClientProvider>
    </ThemeProvider>
  );
}
