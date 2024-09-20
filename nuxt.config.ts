// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-04-03',
  devtools: { enabled: true },

  future: {
    compatibilityVersion: 4,
  },
  app: {
    head: {
      titleTemplate: '%s | MMatch',
    },
  },

  modules: [
    '@nuxtjs/tailwindcss',
    "@nuxt/image",
    "nuxt-svgo",
  ],

  svgo: {
    componentPrefix: 'i',
    defaultImport: 'component',
  },
});