/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["*.html", "./src/**/*.rs",],
    theme: {
        extend: {
            colors: {
                primary: "#00224D",
                secondary: "#A0153E",
                accent: "#FF204E",
                text: "#45474B",
                "light": "#E8EEF4"
            },
            fontFamily: {
                "space-mono": ["Space Mono", "monospace"],
                "space-grotesk": ["Space Grotesk", "sans-serif"],
            }
        },
    },
    plugins: [],
}