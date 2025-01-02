<script lang="ts" module>
  export const DEFAULT_HREF = '/tauri-store/guide/getting-started';
</script>

<script lang="ts">
  import Link from './link.svelte';
  import type { Snippet } from 'svelte';
  import { currentPlugin } from '$lib/stores/plugin';

  interface Props {
    children: Snippet;
    class?: string;
    href?: string;
    plugin: TauriPlugin;
    title?: string;
  }

  const { href = DEFAULT_HREF, ...p }: Props = $props();
</script>

<Link {href} title={p.title} class={p.class} onclick={() => ($currentPlugin = p.plugin)}>
  {@render p.children()}
</Link>
