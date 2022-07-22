import Document, {
  DocumentContext,
  Head,
  Html,
  Main,
  NextScript,
} from "next/document";
import React from "react";

export default class MyDocument extends Document {
  static async getInitialProps(ctx: DocumentContext) {
    return await Document.getInitialProps(ctx);
  }

  render() {
    return (
      <Html lang="en" className="h-full">
        <Head />
        <body className="flex h-full flex-col leading-relaxed antialiased">
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}
