<script lang="ts">
  import { tick } from 'svelte';
  import { cn } from '$lib/utils';
  import * as Icon from '../icons';
  import Link from '../link.svelte';
  import { NAVBAR_HEIGHT } from './constants';
  import NavbarMenu from './navbar-menu.svelte';
  import { resolvePluginIcon } from '$lib/icon';
  import NavbarMobile from './navbar-mobile.svelte';
  import { currentPlugin } from '$lib/stores/plugin';
  import NavbarDesktop from './navbar-desktop.svelte';
  import type { Headings } from '../content/aside.svelte';
  import { Separator } from '$lib/components/ui/separator';
  import { changelogs, javascriptDocs, rustDocs } from '$lib/data';
  import { Trigger as SidebarTrigger, useSidebar } from '../sidebar';

  interface Props {
    headings: Headings;
    height: string;
    left: string;
  }

  const { headings, height, left }: Props = $props();

  const sidebar = useSidebar();

  async function openSidebar() {
    await tick();
    sidebar.toggle();
  }
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
        {#if !sidebar.isMobile}
          <SidebarTrigger />
        {/if}

        <a href="/tauri-store" class="text-xl font-semibold">tauri-store</a>
      </div>
    {/if}
    <div class={cn('flex size-full items-center justify-end', sidebar.isMobile ? 'pr-4' : 'pr-8')}>
      {#if sidebar.isMobile && $currentPlugin}
        {@const PluginIcon = resolvePluginIcon($currentPlugin)}
        {#key $currentPlugin}
          <button type="button" onclick={openSidebar} class="text-muted-foreground">
            <PluginIcon size="1.5rem" />
          </button>
        {/key}
      {:else if !sidebar.isMobile}
        <NavbarDesktop />
      {/if}
    </div>
  </div>

  {#if sidebar.isMobile}
    <NavbarMobile {headings} {openSidebar} />
  {/if}
</header>

<style lang="postcss">
  :global(.navbar-button-mobile) {
    @apply text-muted-foreground hover:text-foreground flex items-center text-sm transition-colors duration-200 ease-linear;
  }
</style>
