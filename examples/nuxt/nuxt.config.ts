export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  ssr: false,
  modules: ['@pinia/nuxt'],
  devServer: {
    port: 1422,
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

  devtools: { enabled: true },
  telemetry: false,
});
