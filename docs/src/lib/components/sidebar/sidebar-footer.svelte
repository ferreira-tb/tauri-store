<script lang="ts">
  import { plugins } from '$lib/data';
  import { ChevronUp } from 'lucide-svelte';
  import { resolvePluginIcon } from '$lib/icon';
  import { currentPlugin } from '$lib/stores/plugin';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
</script>

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
                {@const PluginIcon = resolvePluginIcon($currentPlugin)}
                {#key $currentPlugin}
                  <div class="flex items-center gap-1">
                    <PluginIcon />
                    <span>{$currentPlugin}</span>
                  </div>
                {/key}
              {:else}
                <span>Select a plugin</span>
              {/if}

              <ChevronUp class="ml-auto" />
            </Sidebar.MenuButton>
          {/snippet}
        </DropdownMenu.Trigger>

        <DropdownMenu.Content side="top" class="w-[--bits-dropdown-menu-anchor-width]">
          {#each plugins as plugin (plugin.name)}
            {@const PluginIcon = resolvePluginIcon(plugin.name as TauriPlugin)}
            <DropdownMenu.Item>
              <button
                type="button"
                class="flex w-full items-center justify-start gap-1"
                onclick={() => currentPlugin.set(plugin.name)}
              >
                <PluginIcon />
                <span>{plugin.name}</span>
              </button>
            </DropdownMenu.Item>
          {/each}
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    </Sidebar.MenuItem>
  </Sidebar.Menu>
</Sidebar.Footer>
