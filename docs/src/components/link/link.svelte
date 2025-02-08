<script lang="ts" module>
  export type LinkProps = HTMLAnchorAttributes & {
    children: Snippet;
    class?: string;
    code?: boolean;
    external?: boolean;
    href: string;
    onclick?: (e: MouseEvent) => void;
    title?: string;
  };
</script>

<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLAnchorAttributes } from 'svelte/elements';

  const { title, href, code, external, children, ...restProps }: LinkProps = $props();
</script>

<a
  {...restProps}
  {href}
  {title}
  target={external ? '_blank' : null}
  rel={external ? 'noopener noreferrer' : null}
>
  {#if code}
    <code>{@render children()}</code>
  {:else}
    {@render children()}
  {/if}
</a>
