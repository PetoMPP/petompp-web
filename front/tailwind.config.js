/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.html', './src/**/*.css', './src/**/*.rs', './*.html'],
  theme: {
    fontFamily: {
      'sans': ['Montserrat', 'ui-sans-serif', 'system-ui', 'sans-serif'],
      'mono': ['Roboto Mono', 'ui-monospace', 'SFMono-Regular', 'monospace'],
    },
    extend: {
      height: {
        screen: ['100vh /* fallback for Opera, IE and etc. */', '100dvh'],
      }
    }
  },
  plugins: [require('tailwindcss-animated'), require('@tailwindcss/typography'), require("daisyui")],
  daisyui: {
    themes: [
      {
        winter: {
          ...require("daisyui/src/theming/themes")["[data-theme=winter]"],
        },
        night: {
          ...require("daisyui/src/theming/themes")["[data-theme=night]"],
          "primary": "#38bdf8"
        },
      }
    ],
    darkTheme: "night",
  },
}

