/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./components/**/*.tsx",
    "./pages/**/*.tsx",
  ],
  theme: {
    extend: {
      colors: {
        'navigation': '#272727',
        'button': '#FF652F',
        'background': '#747474',
        'green-theme': '#14A76C',
        'picture': '#FF652F',
      }
    }
  },
  plugins: [],
}
