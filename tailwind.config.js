/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: "#BB86FC",
        "primary-variant": "#3700B3",
        secondary: "#03DAC6",
        background: "#121212",
        surface: "#121212",
        error: "#CF6679",
        "on-primary": "#000000",
        "on-secondary": "#000000",
        "on-background": "#FFFFFF",
        "on-surface": "#FFFFFF",
        "on-error": "#000000",
      },
    },
  },
  plugins: [],
};
