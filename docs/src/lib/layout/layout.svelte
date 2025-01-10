<script lang="ts">
  import { Content } from './content';
  import type { Snippet } from 'svelte';
  import type { Action } from 'svelte/action';
  import { useHeadings } from './content/aside.svelte';
  import { SIDEBAR_WIDTH, useSidebar } from './sidebar';
  import { Navbar, NAVBAR_HEIGHT, NAVBAR_HEIGHT_MOBILE } from './navbar';

  const { children }: { children: Snippet } = $props();

  const sidebar = useSidebar();
  const sidebarWidth = $derived.by(() => {
    if (!sidebar.open || sidebar.isMobile) return '0px';
    return SIDEBAR_WIDTH;
  });

  const navbarHeight = $derived.by(() => {
    return sidebar.isMobile ? NAVBAR_HEIGHT_MOBILE : NAVBAR_HEIGHT;
  });

  const headings = useHeadings();

  const mobile: Action = (node) => {
    $effect(() => {
      node.classList.toggle('ts-mobile', sidebar.isMobile);
    });
  };
</script>

<div use:mobile class="flex min-h-screen w-full flex-col">
  <Navbar height={navbarHeight} left={sidebarWidth} {headings} />
  <Content {headings} {navbarHeight}>
    {@render children()}
  </Content>
</div>
