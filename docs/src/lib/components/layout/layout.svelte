<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';
  import { Navbar, NAVBAR_HEIGHT } from '../navbar';
  import { SIDEBAR_WIDTH, useSidebar } from '../sidebar';

  const { children }: { children: Snippet } = $props();

  const sidebar = useSidebar();
  const sidebarWidth = $derived.by(() => {
    if (!sidebar.open || sidebar.isMobile) return '0px';
    return SIDEBAR_WIDTH;
  });
</script>

<div class="flex min-h-screen w-full flex-col">
  <Navbar left={sidebarWidth} class="bg-sidebar fixed right-0 top-0 z-50 whitespace-nowrap" />
  <main
    style:padding-top={NAVBAR_HEIGHT}
    class={cn('z-0 m-0 w-full flex-shrink-0 flex-grow', sidebar.isMobile ? 'px-2' : 'px-8')}
  >
    {@render children()}
  </main>
</div>
