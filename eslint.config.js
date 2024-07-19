import config from '@tb-dev/eslint-config';

export default config({
  vue: true,
  project: ['packages/guest-js/tsconfig.json', 'packages/playground/tsconfig.json'],
  overrides: {
    typescript: {
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/use-unknown-in-catch-callback-variable': 'off'
    }
  }
});
