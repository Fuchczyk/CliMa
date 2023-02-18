/** @type {import('tailwindcss').Config} */
module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
    theme: {
    extend: {
      colors: {
        'navigation': '#272727',
        'button': '#FF652F',
      },
    },
  },
    variants: {},
    plugins: [],
}
