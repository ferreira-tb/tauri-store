<script lang="ts">
  import { ChevronUp } from 'lucide-svelte';
  import metadata from '$lib/data/metadata.json';
  import { currentPlugin } from '$lib/stores/plugin';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

  const plugins = metadata.filter((it) => it.name.startsWith('tauri-plugin'));
</script>

<Sidebar.Root>
  <Sidebar.Header>
    <div class="select-none pl-2 pt-2">
      <a href="/tauri-store" class="text-xl font-semibold">tauri-store</a>
    </div>
  </Sidebar.Header>

  <Sidebar.Content>
    <nav class="p-4">
      <ul class="space-y-2">
        {#snippet guide(path: string, label: string)}
          <li>
            <a href={`/tauri-store/guide/${path}`}>{label}</a>
          </li>
        {/snippet}

        {@render guide('getting-started', 'Getting started')}
        {@render guide('persisting-state', 'Persisting state')}
        {@render guide('synchronization', 'Synchronization')}
        {@render guide('accessing-from-rust', 'Accessing from Rust')}
      </ul>
    </nav>
  </Sidebar.Content>

  <Sidebar.Footer>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <DropdownMenu.Root>
          <DropdownMenu.Trigger>
            {#snippet child({ props })}
              <Sidebar.MenuButton
                {...props}
                class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
              >
                {#if $currentPlugin}
                  <span>{$currentPlugin}</span>
                {:else}
                  <span>Select a plugin</span>
                {/if}

                <ChevronUp class="ml-auto" />
              </Sidebar.MenuButton>
            {/snippet}
          </DropdownMenu.Trigger>

          <DropdownMenu.Content side="top" class="w-[--bits-dropdown-menu-anchor-width]">
            {#each plugins as plugin (plugin.name)}
              <DropdownMenu.Item>
                <button
                  type="button"
                  class="w-full text-start"
                  onclick={() => currentPlugin.set(plugin.name)}
                >
                  <span>{plugin.name}</span>
                </button>
              </DropdownMenu.Item>
            {/each}
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Footer>
</Sidebar.Root>
