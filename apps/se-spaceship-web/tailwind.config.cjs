/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{svelte,ts}"],
  theme: {
    extend: {
      minWidth: ({ theme }) => ({
        ...theme("width"),
      }),
      minHeight: ({ theme }) => ({
        ...theme("height"),
      }),
    },
  },
  plugins: [],
};
