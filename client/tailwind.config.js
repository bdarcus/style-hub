/** @type {import('tailwindcss').Config} */
export default {
    darkMode: "class",
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                "primary": "#135bec",
                "background-light": "#f8f9fc",
                "background-dark": "#101622",
                "surface-light": "#ffffff",
                "border-light": "#e7ebf3",
                "text-main": "#0d121b",
                "text-secondary": "#4c669a"
            },
            fontFamily: {
                "display": ["Lexend", "sans-serif"],
                "serif": ["Merriweather", "serif"],
                "sans": ["Lexend", "sans-serif"], // Default sans to Lexend
            },
            borderRadius: {
                "DEFAULT": "0.25rem",
                "lg": "0.5rem",
                "xl": "0.75rem",
                "2xl": "1rem",
                "full": "9999px"
            },
        },
    },
    plugins: [],
}
