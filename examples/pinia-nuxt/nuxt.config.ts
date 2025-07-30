export default defineNuxtConfig({
  compatibilityDate: 'latest',
  modules: ['@pinia/nuxt'],
  ssr: false,
  devServer: {
    port: 1422,
  },
  devtools: {
    enabled: false,
  },

  vite: {
    clearScreen: false,
    server: {
      strictPort: true,
      watch: {
        ignored: ['**/src-tauri/**'],
      },
    },
    build: {
      emptyOutDir: true,
      minify: false,
      target: 'esnext',
    },
  },
});
