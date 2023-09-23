const defaultTheme = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
    theme: {
        extend: {
            fontFamily: {
                mono: ["JetBrains Mono", ...defaultTheme.fontFamily.mono],
            },
        },
    },
    plugins: [require("daisyui"), require("@tailwindcss/typography")],
    daisyui: {
        themes: [
            {
                light: {
                    primary: "#45d6ce",
                    secondary: "#5ed793",
                    accent: "#a7e1c1",
                    neutral: "#2a323c",
                    "base-100": "#ffffff",
                    info: "#78A9DC",
                    success: "#9FC27F",
                    warning: "#F6C975",
                    error: "#EF7172",
                    ".dark-only": {
                        display: "none",
                    },
                    ".light-only": {
                        display: "initial",
                    },
                },
            },
            {
                dark: {
                    primary: "#56BEB8",
                    secondary: "#56BE84",
                    accent: "#70A487",
                    neutral: "#2a323c",
                    "base-100": "#242424",
                    "base-200": "#1a1a1a",
                    "base-300": "#121212",
                    info: "#78A9DC",
                    success: "#9FC27F",
                    warning: "#F6C975",
                    error: "#EF7172",
                    ".dark-only": {
                        display: "initial",
                    },
                    ".light-only": {
                        display: "none",
                    },
                },
            },
        ],
    },
};
