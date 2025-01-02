<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    children: Snippet;
    class?: string;
    external?: boolean;
    href: string;
    onclick?: (e: MouseEvent) => void;
    title?: string;
  }

  const p: Props = $props();
</script>

<a
  href={p.href}
  title={p.title}
  target={p.external ? '_blank' : null}
  rel={p.external ? 'noopener noreferrer' : null}
  onclick={(e) => p.onclick?.(e)}
  class={p.class}
>
  {@render p.children()}
</a>
