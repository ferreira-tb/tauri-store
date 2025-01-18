<script lang="ts">
  import type { Code } from '$lib/code';
  import { Ext } from '$components/link';
  import { CodeGroup } from '$components/code';
  import { Container } from '$components/container';

  const url = {
    runes: 'https://svelte.dev/docs/svelte/what-are-runes',
  };

  const runesExample: Code = {
    id: 'runes-example',
    label: 'src/lib/stores/counter.svelte.ts',
    lang: 'typescript',
    value: `
import { RuneStore } from 'tauri-plugin-svelte';

const store = new RuneStore('counter', { counter: 0 });
    `,
  };
</script>

<Container title="Runes">
  <p>You can also use <Ext href={url.runes}>runes</Ext> instead of conventional stores.</p>

  <CodeGroup code={runesExample} />
</Container>
