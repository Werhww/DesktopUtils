export default defineNuxtConfig({
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],
  ssr: false,
  modules: [
    "nuxt-quasar-ui",
    "@vueuse/nuxt"
  ],

  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
      hmr: {
        protocol: 'ws',
        host: '0.0.0.0',        
        port: 5183,
      },
    },
  },

  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },

  },


  quasar: {
    plugins: [
      "Loading",
      "Notify"
    ],

    iconSet: "material-symbols-rounded",
    extras: {
      fontIcons: [
        "material-symbols-rounded",
        "material-symbols-outlined",
        "material-symbols-sharp",
        "material-icons-outlined",
        "material-icons-round",
        "material-icons-sharp",
        "material-icons",
        "line-awesome",
        "fontawesome-v6"
      ],

      svgIcons: [
        "material-symbols-rounded",
        "material-symbols-outlined",
        "material-symbols-sharp",
        "material-icons-outlined",
        "material-icons-round",
        "material-icons-sharp",
        "material-icons",
        "line-awesome",
        "fontawesome-v6"
      ],

      animations: [
        "fadeOutUp",
        "fadeInDown"
      ],
    },

    sassVariables: "assets/css/quasar.variables.scss",
    config: {
      dark: true,
      brand: {
        primary: '#252525',
        secondary: '#FFFFFF',
        accent: '#2E2E2E',
        dark: '#252525',
        "dark-page": '#252525',

        positive: '#21BA45',
        negative: '#C10015',
        info: '#31CCEC',
        warning: '#F2C037'
      },
    },
  },
});