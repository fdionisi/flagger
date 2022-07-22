/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: [
    "./www/**/*.tsx",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require("preline/plugin"),
    require("@tailwindcss/typography"),
  ],
};
