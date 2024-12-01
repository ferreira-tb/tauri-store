import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: [
    'docs/tsconfig.json',
    'examples/nuxt/tsconfig.json',
    'examples/pinia/tsconfig.json',
    'examples/shared-vue/tsconfig.json',
    'packages/tauri-plugin-pinia/tsconfig.json',
  ],
  features: {
    vue: true,
  },
  overrides: {
    typescript: {
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/use-unknown-in-catch-callback-variable': 'off',
    },
    vue: {
      'vue/v-on-handler-style': 'off',
    },
  },
});
