import { Command } from "commander";

import { Server } from "./server";

new Command("flagger-serve-www")
  .requiredOption("--api-url <API_URL>", "flagger's APi url")
  .requiredOption("--www-port <WWW_PORT>", "www listening port")
  .action(async ({ apiUrl, wwwPort }) => {
    const server = await Server.create({ apiUrl });

    await server.listen(wwwPort);
  })
  .parseAsync()
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
