<script lang="ts">
  import { resolveHeaderIcon } from './icon';
  import { ChevronDown } from 'lucide-svelte';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import { DEFAULT_MODE, type Mode, mode } from './mode';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
  import { useSidebar } from '$lib/components/ui/sidebar/index.js';

  const sidebar = useSidebar();

  interface MenuItem {
    label: string;
    mode: Mode;
  }

  const items: MenuItem[] = [
    { mode: 'learn', label: 'Learn' },
    { mode: 'reference', label: 'Reference' },
    { mode: 'changelog', label: 'Changelog' },
  ];

  function find(value?: Mode | null): MenuItem {
    const item = items.find((it) => it.mode === value);
    return item ?? find(DEFAULT_MODE);
  }
</script>

<Sidebar.Header>
  {#if sidebar.isMobile}
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <DropdownMenu.Root>
          <DropdownMenu.Trigger>
            {#snippet child({ props })}
              {@const HeaderIcon = resolveHeaderIcon($mode)}
              <Sidebar.MenuButton {...props}>
                {#key $mode}
                  <div class="flex w-full items-center gap-2">
                    <HeaderIcon class="size-3" />
                    <span>{find($mode).label}</span>
                  </div>
                {/key}
                <ChevronDown class="ml-auto" />
              </Sidebar.MenuButton>
            {/snippet}
          </DropdownMenu.Trigger>
          <DropdownMenu.Content class="w-[--bits-dropdown-menu-anchor-width]">
            {#each items as item (item.mode)}
              {@const HeaderIcon = resolveHeaderIcon(item.mode)}
              <DropdownMenu.Item>
                <button
                  type="button"
                  class="flex w-full items-center gap-2"
                  onclick={() => mode.set(item.mode)}
                >
                  <HeaderIcon class="size-3" />
                  <span>{item.label}</span>
                </button>
              </DropdownMenu.Item>
            {/each}
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  {:else}
    <div class="select-none pl-2 pt-2">
      <a href="/tauri-store" class="text-xl font-semibold">tauri-store</a>
    </div>
  {/if}
</Sidebar.Header>
