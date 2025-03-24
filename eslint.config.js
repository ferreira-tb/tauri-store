import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: [
    'docs/tsconfig.json',
    'examples/migration/tsconfig.json',
    'examples/pinia/tsconfig.json',
    'examples/pinia-nuxt/tsconfig.json',
    'examples/playground/tsconfig.json',
    'examples/svelte/tsconfig.json',
    'examples/svelte-runes/tsconfig.json',
    'examples/valtio/tsconfig.json',
    'examples/zustand/tsconfig.json',
    'packages/shared/tsconfig.json',
    'packages/plugin-pinia/tsconfig.json',
    'packages/plugin-svelte/tsconfig.json',
    'packages/plugin-valtio/tsconfig.json',
    'packages/plugin-zustand/tsconfig.json',
    'packages/tauri-store/tsconfig.json',
  ],
  features: {
    react: true,
    reactCompiler: false,
    reactHooks: false,
    svelte: true,
    vue: true,
  },
  ignores: ['**/.nuxt', '**/.svelte-kit', '**/assets'],
  moduleBoundaryTypesFiles: ['packages/**/*.ts'],
  overrides: {
    svelte: {
      'svelte/block-lang': 'off',
      'svelte/no-at-html-tags': 'off',
      'svelte/no-ignored-unsubscribe': 'off',
      'svelte/no-inline-styles': 'off',
    },
    typescript: {
      '@typescript-eslint/consistent-type-definitions': 'off',
      '@typescript-eslint/naming-convention': 'off',
      '@typescript-eslint/no-confusing-void-expression': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-non-null-assertion': 'off',
      '@typescript-eslint/use-unknown-in-catch-callback-variable': 'off',
    },
    vue: {
      'vue/enforce-style-attribute': 'off',
      'vue/v-on-handler-style': 'off',
    },
  },
});
