/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ["./src/**/*.{html,js,jsx,ts,tsx}","./src/components/Canvas.tsx"],
  theme: {
    extend: {},
  },
  plugins: [],
}
