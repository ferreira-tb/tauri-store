<script lang="ts">
  import { resolveIcon } from './header';
  import { ChevronsUpDown } from 'lucide-svelte';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import { DEFAULT_MODE, type Mode, mode } from './mode';
  import { useSidebar } from '$lib/components/ui/sidebar';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

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
              <Sidebar.MenuButton
                size="lg"
                class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                {...props}
              >
                {#key $mode}
                  <div class="flex w-full items-center gap-2 text-lg font-semibold leading-none">
                    <span>{find($mode).label}</span>
                  </div>
                {/key}
                <ChevronsUpDown class="ml-auto" />
              </Sidebar.MenuButton>
            {/snippet}
          </DropdownMenu.Trigger>

          <DropdownMenu.Content align="start" class="w-[--bits-dropdown-menu-anchor-width]">
            {#each items as item (item.mode)}
              {@const HeaderIcon = resolveIcon(item.mode)}
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
