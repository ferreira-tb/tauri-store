<script lang="ts">
  import type { Snippet } from 'svelte';
  import { kebabCase } from 'change-case';
  import Container from '../container.svelte';
  import type { HTMLAttributes } from 'svelte/elements';

  type Props = HTMLAttributes<HTMLElement> & {
    children?: Snippet;
    class?: string;
    version: string;
  };

  const { version, children, ...restProps }: Props = $props();
</script>

<Container {...restProps} title={version} id={`v${kebabCase(version)}`} level={2}>
  {@render children?.()}
</Container>
