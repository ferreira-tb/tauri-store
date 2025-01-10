<script lang="ts">
  import { cn } from '$lib/utils';
  import { NAVBAR_HEIGHT } from './constants';
  import { Link } from '$lib/components/link';
  import { useSidebar } from '$lib/layout/sidebar';
  import NavbarMobile from './navbar-mobile.svelte';
  import NavbarPlugin from './navbar-plugin.svelte';
  import NavbarSocial from './navbar-social.svelte';
  import { currentPlugin } from '$lib/stores/plugin';
  import NavbarDesktop from './navbar-desktop.svelte';
  import type { Headings } from '$lib/layout/content/aside.svelte';

  interface Props {
    headings: Headings;
    height: string;
    left: string;
  }

  const { headings, height, left }: Props = $props();

  const sidebar = useSidebar();
</script>

<header
  style:left
  style:height
  class={cn(
    'fixed right-0 top-0 z-50',
    sidebar.isMobile
      ? 'bg-sidebar'
      : 'bg-background border-sidebar-border border-b shadow-sm transition-[left] duration-200 ease-linear'
  )}
>
  <div style:height={NAVBAR_HEIGHT} class="flex items-center justify-between whitespace-nowrap">
    {#if sidebar.isMobile || !sidebar.open}
      <div class="flex h-full select-none items-center gap-2 pl-4">
        <Link href="/tauri-store" class="text-xl font-semibold">tauri-store</Link>
      </div>
    {/if}
    <div class={cn('flex size-full items-center justify-end', sidebar.isMobile ? 'pr-4' : 'pr-8')}>
      {#if sidebar.isMobile}
        <NavbarSocial size="1.5rem">
          {#if $currentPlugin}
            <NavbarPlugin />
          {/if}
        </NavbarSocial>
      {:else if !sidebar.isMobile}
        <NavbarDesktop />
      {/if}
    </div>
  </div>

  {#if sidebar.isMobile}
    <NavbarMobile {headings} />
  {/if}
</header>

<style lang="postcss">
  :global(.navbar-button-mobile) {
    @apply text-muted-foreground hover:text-foreground flex items-center text-sm transition-colors duration-200 ease-linear;
  }
</style>
