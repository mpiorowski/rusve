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
                inset: "inset 0 2px 2px 0px hsla(0, 0%, 0%, 0.4), inset 0 0px 2px 2px hsla(0, 0%, 0%, 0.3)",
            },
        },
    },
    plugins: [require("@tailwindcss/forms")],
};
