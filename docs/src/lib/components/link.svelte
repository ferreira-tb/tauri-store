<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    children: Snippet;
    class?: string;
    external?: boolean;
    href: string;
    onclick?: () => void;
  }

  const { href, external, onclick, class: className, children }: Props = $props();
</script>

<a
  {href}
  class={className}
  target={external ? '_blank' : null}
  rel={external ? 'noopener noreferrer' : null}
  {onclick}
>
  {@render children()}
</a>
