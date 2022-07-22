import * as http from "http";
import * as url from "url";

import KoaRouter from "@koa/router";
import { createProxy } from "http-proxy";
import Koa from "koa";
import next from "next";

export interface ServerConfig {
  apiUrl: string;
}

export class Server {
  /** The native Node.js HTTP server reference. */
  server: http.Server;

  /**
   * # Not supported initialization method
   * Use `Server.create` instead.
   */
  private constructor(server: http.Server) {
    this.server = server;
  }

  /**
   * Start listening on HTTP request on a certain port.
   *
   * @param port the port for incoming HTTP requests.
   */
  async listen(port?: number): Promise<void> {
    return new Promise<void>((resolve) => {
      this.server.listen(port, () => {
        resolve();
      });
    });
  }

  /**
   * Gracefully close all open connections and stop listening for HTTP requests.
   */
  async shutdown(): Promise<void> {
    if (!this.server.listening) {
      return;
    }

    return new Promise<void>((resolve) => {
      this.server.close(() => {
        resolve();
      });
    });
  }

  /**
   * Create a new instance of the `www` Server.
   *
   * @param config server configuration
   * @returns a new `Server` instance
   */
  static async create(config: ServerConfig): Promise<Server> {
    const environment = process.env.NODE_ENV || "development";
    const website = next({
      dir: __dirname,
      dev: environment === "development",
      quiet: true,
    });

    await website.prepare();

    const handle = website.getRequestHandler();
    const proxy = createProxy({
      secure: false,
      changeOrigin: true,
    });

    const apiRouter = new KoaRouter({ prefix: "/api" })
      .all("Management API proxy", "/:path*", async function proxyAll(context) {
        return await new Promise<void>((resolve, reject) => {
          const path = context.path.split("/api").pop() || "/";
          const target =
            `${config.apiUrl}${path}?${context.request.querystring}`;

          proxy.web(
            context.req,
            context.res,
            {
              ignorePath: true,
              target,
            },
            (error) => {
              if (error) {
                reject(error);
              } else {
                resolve();
              }
            },
          );
        });
      });

    const healthRouter = new KoaRouter().get(
      "healthCheck",
      "/healthz",
      async function healthz(context) {
        context.status = 204;
      },
    );

    const app = new Koa()
      .use(apiRouter.routes())
      .use(apiRouter.allowedMethods())
      .use(healthRouter.routes())
      .use(healthRouter.allowedMethods())
      .use(async (context) => {
        context.status = context.status >= 300 && context.status <= 399
          ? context.status
          : 200;

        const parsedUrl = url.parse(context.url || "/", true);
        await handle(context.req, context.res, parsedUrl);

        context.respond = false;
      });

    return new Server(http.createServer(app.callback()));
  }
}
