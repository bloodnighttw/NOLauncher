/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      animation:{
        fade: 'fade .2s ease-in-out'
      },
      keyframes: {
        fade: {
          from: { opacity: 0 },
          to: { opacity: 1 },
        },
			},

    },
  },
  plugins: [
    require('daisyui')
  ],
};
