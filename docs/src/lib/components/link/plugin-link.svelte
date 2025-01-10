<script lang="ts" module>
  export const DEFAULT_HREF = '/tauri-store/guide/getting-started';
</script>

<script lang="ts">
  import Link from './link.svelte';
  import type { Snippet } from 'svelte';
  import { currentPlugin } from '$lib/stores/plugin';
  import type { HTMLAnchorAttributes } from 'svelte/elements';

  type Props = HTMLAnchorAttributes & {
    children: Snippet;
    class?: string;
    href?: string;
    plugin: TauriPlugin;
    title?: string;
  };

  const { href = DEFAULT_HREF, title, plugin, children, ...restProps }: Props = $props();

  const url = $derived.by(() => {
    return href.includes('?plugin') ? href : `${href}?plugin=${plugin}`;
  });

  function onClick() {
    currentPlugin.set(plugin);
  }
</script>

<Link {...restProps} href={url} {title} onclick={onClick}>
  {@render children()}
</Link>
