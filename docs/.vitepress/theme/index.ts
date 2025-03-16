import Layout from './Layout.vue';
import type { Theme } from 'vitepress';
import DefaultTheme from 'vitepress/theme';

const theme: Theme = {
  extends: DefaultTheme,
  Layout,
  enhanceApp() {
    // ...
  },
};

export default theme;
