import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: [
    'tsconfig.json',
    'docs/tsconfig.json',
    'examples/migration/tsconfig.json',
    'examples/pinia/tsconfig.json',
    'examples/pinia-mobile/tsconfig.json',
    'examples/pinia-nuxt/tsconfig.json',
    'examples/playground/tsconfig.json',
    'examples/svelte/tsconfig.json',
    'examples/svelte-runes/tsconfig.json',
    'examples/valtio/tsconfig.json',
    'examples/vue/tsconfig.json',
    'examples/zustand/tsconfig.json',
    'packages/shared/tsconfig.json',
    'packages/plugin-pinia/tsconfig.json',
    'packages/plugin-svelte/tsconfig.json',
    'packages/plugin-valtio/tsconfig.json',
    'packages/plugin-vue/tsconfig.json',
    'packages/plugin-zustand/tsconfig.json',
    'packages/tauri-store/tsconfig.json',
  ],
  features: {
    vue: true,
  },
  ignores: ['**/.nuxt', '**/.svelte-kit', '**/assets', '**/nuxt.config.ts'],
  moduleBoundaryTypesFiles: ['packages/**/*.ts'],
  overrides: {
    typescript: {
      'consistent-type-definitions': 'off',
      'naming-convention': 'off',
      'no-confusing-void-expression': 'off',
      'no-explicit-any': 'off',
      'no-non-null-assertion': 'off',
      'use-unknown-in-catch-callback-variable': 'off',
    },
    vue: {
      'enforce-style-attribute': 'off',
      'v-on-handler-style': 'off',
    },
  },
});
