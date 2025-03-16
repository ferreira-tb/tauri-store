import type { Theme } from 'vitepress';
import DefaultTheme from 'vitepress/theme';

const theme: Theme = {
  extends: DefaultTheme,
  enhanceApp() {
    // ...
  },
};

export default theme;
