<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';
  import { Link } from '$lib/components/link';
  import * as Icon from '$lib/components/icons';

  interface Props {
    children?: Snippet;
    class?: string;
    size?: string;
  }

  const { size = '1.25rem', class: className, children }: Props = $props();

  const url = {
    discord: 'https://discord.gg/ARd7McmVNv',
    github: 'https://github.com/ferreira-tb/tauri-store',
  };
</script>

<div class={cn('flex items-center justify-end gap-4', className)}>
  <Link href={url.discord} external>
    <Icon.Discord {size} />
  </Link>
  <Link href={url.github} external>
    <Icon.Github {size} />
  </Link>

  {@render children?.()}
</div>
