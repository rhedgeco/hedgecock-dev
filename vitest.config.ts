/// <reference types="vitest" />
import { getViteConfig } from "astro/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default getViteConfig({
    plugins: [svelte({ hot: !process.env.VITEST })],
    test: {
        globals: true,
        environment: "jsdom",
        setupFiles: ["./src/tests/vitest/setup.ts"],
        include: ["./src/tests/vitest/**/*.spec.ts"],
    },
});
