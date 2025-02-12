<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';
  import { Link } from '$components/link';
  import * as Icon from '$components/icon';
  import { DISCORD, GITHUB } from '$lib/url';

  type Props = {
    children?: Snippet;
    class?: string;
    size?: string;
  };

  const { size = '1.25rem', class: className, children }: Props = $props();
</script>

<div class={cn('flex items-center justify-end gap-4', className)}>
  <Link href={DISCORD} external>
    <Icon.Discord {size} />
  </Link>
  <Link href={GITHUB} external>
    <Icon.Github {size} />
  </Link>

  {@render children?.()}
</div>
