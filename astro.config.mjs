import { defineConfig } from "astro/config";
import tailwind from "@astrojs/tailwind";

import svelte from "@astrojs/svelte";

// https://astro.build/config
export default defineConfig({
    outDir: "server/static",
    trailingSlash: "always",
    integrations: [tailwind(), svelte()],
    vite: {
        server: {
            watch: {
                ignored: ["**/server/**"],
            },
            proxy: {
                "/api": {
                    target: "http://127.0.0.1:8000",
                    changeOrigin: true,
                    ws: true,
                },
            },
        },
    },
});
