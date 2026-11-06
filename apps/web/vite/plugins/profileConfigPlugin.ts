import fs from "node:fs/promises";
import type { Plugin } from "vite";

const PUBLIC_PROFILE_PATH = "/config/profiles.json";
const BUILD_PROFILE_PATH = "config/profiles.json";

/** Serves local profile metadata to the browser during development and build. */
export function profileConfigPlugin(sourcePath: string): Plugin {
  return {
    name: "doseark-profile-config",

    configureServer(server) {
      server.middlewares.use(PUBLIC_PROFILE_PATH, async (_request, response) => {
        try {
          const source = await fs.readFile(sourcePath, "utf8");
          response.statusCode = 200;
          response.setHeader("Content-Type", "application/json; charset=utf-8");
          response.setHeader("Cache-Control", "no-store");
          response.end(source);
        } catch (error) {
          response.statusCode = 500;
          response.setHeader("Content-Type", "text/plain; charset=utf-8");
          response.end(error instanceof Error ? error.message : "Could not load profile config.");
        }
      });
    },

    async generateBundle() {
      const source = await fs.readFile(sourcePath, "utf8");
      this.emitFile({
        type: "asset",
        fileName: BUILD_PROFILE_PATH,
        source,
      });
    },
  };
}
