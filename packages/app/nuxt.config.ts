const PORT = process.env.PORT;
const APP_URL = process.env.APP_URL;
const SITE_URL = process.env.SITE_URL;
const BACKEND_URL = process.env.BACKEND_URL;

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

  runtimeConfig: {
    public: {
      appUrl: APP_URL,
      siteUrl: SITE_URL,
      backendUrl: BACKEND_URL,
    }
  },

  devServer: {
    port: PORT ? +PORT : 3000,
  },

  modules: [
    "@mmatch/common/module",
    '@nuxtjs/tailwindcss',
    "@nuxt/image",
    "nuxt-svgo",
  ],

  svgo: {
    componentPrefix: 'i',
    defaultImport: 'component',
    autoImportPath: '~/assets/icons/'
  },
  tailwindcss: {
    configPath: '../../tailwind.config.ts',
    config: {
      content: [
        './app/**/*',
        './pages/**/*',
        './components/**/*',
        './modules/**/*',
        '../common/components/**/*',
      ]
    }
  }

});