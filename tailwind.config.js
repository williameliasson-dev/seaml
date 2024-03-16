/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamily: {
        sans: ["Outfit", "sans-serif"],
      },
      colors: {
        primary: "#F9F7F3",
        secondary: "#EDDEA4",
        tertiary: "#B5E2FA",
        tangerine: "#F7A072",
        moonstone: "#0FA3B1",
      },
    },
  },
  plugins: [],
};
