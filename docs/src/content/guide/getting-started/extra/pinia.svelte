<script lang="ts">
  import type { Code } from '$lib/code';
  import { Ext } from '$components/link';
  import { CodeGroup } from '$components/code';
  import { Container } from '$components/container';

  const nuxtPlugin: Code = {
    id: 'nuxt-plugin',
    label: 'plugins/pinia.ts',
    lang: 'typescript',
    value: `
import type { Pinia } from 'pinia';
import { TauriPluginPinia } from 'tauri-plugin-pinia';

// See: https://pinia.vuejs.org/core-concepts/plugins.html#Nuxt-js
export default defineNuxtPlugin(({ $pinia }) => {
  ($pinia as Pinia).use(TauriPluginPinia());
});
    `,
  };

  const url = {
    nuxt: 'https://nuxt.com/',
    nuxtPlugin: 'https://nuxt.com/docs/guide/directory-structure/plugins',
  };
</script>

<Container title="Nuxt">
  <p>
    If you are using <Ext href={url.nuxt}>Nuxt</Ext>, you can create a
    <Ext href={url.nuxtPlugin}>Nuxt plugin</Ext> to enable it for Pinia:
  </p>

  <CodeGroup code={nuxtPlugin} />
</Container>
