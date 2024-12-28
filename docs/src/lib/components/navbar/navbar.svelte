<script lang="ts">
  import { cn } from '$lib/utils';
  import * as Icon from '../icons';
  import Link from '../link.svelte';
  import NavbarMenu from './navbar-menu.svelte';
  import { Separator } from '$lib/components/ui/separator';
  import { DISCORD, GITHUB, NAVBAR_HEIGHT } from './constants';
  import { changelogs, javascriptDocs, rustDocs } from '$lib/data';
  import { Trigger as SidebarTrigger, useSidebar } from '../sidebar';

  interface Props {
    class?: string;
    left: string;
  }

  const { class: className, left }: Props = $props();

  const sidebar = useSidebar();
</script>

<header style:left style:height={NAVBAR_HEIGHT} class={cn('flex justify-between', className)}>
  {#if sidebar.isMobile || !sidebar.open}
    <div class="flex h-full select-none items-center pl-4 pt-2">
      <a href="/tauri-store" class="text-xl font-semibold">tauri-store</a>
    </div>
  {/if}

  <div class={cn('flex w-full items-center justify-end', sidebar.isMobile ? 'pr-2' : 'pr-8')}>
    {#if sidebar.isMobile}
      <SidebarTrigger />
    {:else}
      <nav class="flex items-center gap-6 font-semibold">
        <a href="/tauri-store/guide/getting-started">Guide</a>
        <NavbarMenu label="JavaScript" items={javascriptDocs} external />
        <NavbarMenu label="Rust" items={rustDocs} external />
        <NavbarMenu label="Changelog" items={changelogs} />
      </nav>
      <div class="h-[calc(100%-2rem)]">
        <Separator orientation="vertical" class="mx-4" />
      </div>
      <div class="flex items-center justify-end gap-4">
        <Link href={GITHUB} external>
          <Icon.Github size="1.25rem" />
        </Link>
        <Link href={DISCORD} external>
          <Icon.Discord size="1.25rem" />
        </Link>
      </div>
    {/if}
  </div>
</header>
