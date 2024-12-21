import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: [
    'docs/tsconfig.json',
    'examples/pinia/tsconfig.json',
    'examples/pinia-nuxt/tsconfig.json',
    'examples/shared/javascript/tsconfig.json',
    'examples/shared/pinia/tsconfig.json',
    'packages/shared/tsconfig.json',
    'packages/tauri-plugin-pinia/tsconfig.json',
    'packages/tauri-plugin-svelte/tsconfig.json',
  ],
  features: {
    svelte: true,
    vue: true,
  },
  ignores: ['**/.nuxt/**', '**/tauri-store-cli/assets/**'],
  overrides: {
    typescript: {
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/use-unknown-in-catch-callback-variable': 'off',
    },
    vue: {
      'vue/enforce-style-attribute': 'off',
      'vue/v-on-handler-style': 'off',
    },
  },
});
