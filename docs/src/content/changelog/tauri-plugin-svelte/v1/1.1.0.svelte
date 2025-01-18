<script lang="ts">
  import type { Code } from '$lib/code';
  import { Ext } from '$components/link';
  import { CodeGroup } from '$components/code';
  import { Changelog } from '$components/container';

  const version = '1.1.0';

  const runes = 'https://svelte.dev/docs/svelte/what-are-runes';

  const code: Code = {
    id: 'custom-path',
    lang: 'typescript',
    label: 'src/lib/stores/counter.svelte.ts',
    value: `
import { RuneStore } from 'tauri-plugin-svelte';

const store = new RuneStore('counter', { count: 0 });

export function increment() {
  // "state" is reactive!
  store.state.count += 1;
}
    `,
  };
</script>

<Changelog.Version {version} />

<Changelog.Features {version}>
  <ul>
    <li>Support <Ext href={runes}>runes</Ext>.</li>
    <CodeGroup {code} accordion="Show example" />
  </ul>
</Changelog.Features>
