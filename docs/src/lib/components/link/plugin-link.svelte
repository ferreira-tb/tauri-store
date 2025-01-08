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

  function onClick() {
    currentPlugin.set(plugin);
  }
</script>

<Link {...restProps} {href} {title} onclick={onClick}>
  {@render children()}
</Link>
