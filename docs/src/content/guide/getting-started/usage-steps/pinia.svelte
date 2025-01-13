<script lang="ts">
  import type { Code } from '$lib/code';
  import * as Alert from '$components/alert';
  import { CodeBlock, CodeGroup } from '$components/code';
  import StartIt from '$content/guide/getting-started/start-it.svelte';

  const enablePlugin: Code = {
    id: 'enable-plugin',
    label: 'src/index.ts',
    lang: 'typescript',
    value: `
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia';

const app = createApp(App);

const pinia = createPinia();
pinia.use(createPlugin());

app.use(pinia)
app.mount('#app');
    `,
  };

  const defineStore: Code = {
    id: 'define-store',
    label: 'src/stores/counter.ts',
    lang: 'typescript',
    value: `
import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useCounterStore = defineStore('counter', () => {
  const counter = ref(0);
  return { counter };
});
    `,
  };

  const startStore = `
import { useCounterStore } from '@/stores/counter';

const counterStore = useCounterStore();
await counterStore.$tauri.start();
`;
</script>

<li>
  <span>Enable the plugin for Pinia:</span>

  <CodeGroup code={enablePlugin} />

  <Alert.Root class="pb-4">
    <Alert.Title>Tip</Alert.Title>
    <Alert.Description>
      <code>createPlugin</code> is also exported as <code>TauriPluginPinia</code>.
    </Alert.Description>
  </Alert.Root>
</li>

<li>
  <span>Define your Pinia store:</span>

  <CodeGroup code={defineStore} />
</li>

<li>
  <span>Start it:</span>

  <CodeBlock lang="typescript" code={startStore} />

  <StartIt />
</li>
