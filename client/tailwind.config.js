/** @type {import('tailwindcss').Config} */
export default {
    darkMode: "class",
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
        extend: {
            fontFamily: {
                poppins: ["Poppins", "sans-serif"],
            },
            boxShadow: {
                inset: "inset 0 1px 2px rgba(0,0,0,.39), 0 -1px 1px #1f2937, 0 1px 0 #1f2937",
            },
        },
    },
    plugins: [
        require("@tailwindcss/forms"),
    ],
};
