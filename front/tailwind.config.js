/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.html', './src/**/*.css', './src/**/*.rs', './*.html'],
  theme: {
    extend: {
      height: {
        screen: ['100vh /* fallback for Opera, IE and etc. */', '100dvh'],
      }
    }
  },
  plugins: [require('tailwindcss-animated')],
}

