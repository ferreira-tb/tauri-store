<script lang="ts">
  import { Link } from '../link';
  import { cn } from '$lib/utils';
  import * as Icon from '../icons';
  import type { Snippet } from 'svelte';

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
