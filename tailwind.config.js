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
        primary: "#F7FFF7",
        secondary: "#FF6B6B",
        tertiary: "#FFE66D",
        egg_blue: "#4ECDC4",
        gun_metal: "#292F36",
      },
    },
  },
  plugins: [],
};
