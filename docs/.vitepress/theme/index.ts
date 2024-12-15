import '@/assets/index.css';
import Layout from '@/Layout.vue';
import type { Theme } from 'vitepress';
import DefaultTheme from 'vitepress/theme';
import DocsRs from '@/components/DocsRs.vue';
import DocsTs from '@/components/DocsTs.vue';
import Feature from '@/components/Feature.vue';
import { provideSymbols } from '@/utils/symbols';
import PluginLink from '@/components/PluginLink.vue';
import ExternalLink from '@/components/ExternalLink.vue';

const theme: Theme = {
  extends: DefaultTheme,
  Layout,
  enhanceApp({ app, router }) {
    app
      .component('DocsRs', DocsRs)
      .component('DocsTs', DocsTs)
      .component('ExternalLink', ExternalLink)
      .component('Feature', Feature)
      .component('PluginLink', PluginLink);

    provideSymbols(app, router);
  },
};

export default theme;
