<script lang="ts">
  import type { Snippet } from 'svelte';
  import Container from '../container.svelte';
  import type { HTMLAttributes } from 'svelte/elements';

  type Props = HTMLAttributes<HTMLElement> & {
    children?: Snippet;
    class?: string;
  };

  const { children, ...restProps }: Props = $props();
</script>

<Container {...restProps} title="Next" id="next" level={2}>
  {#if children}
    {@render children()}
  {:else}
    <p class="italic">Nothing yet.</p>
  {/if}
</Container>
