<script lang="ts">
  import { plugins } from '$lib/data';
  import { resolveIcon } from '$lib/plugin';
  import { currentPlugin } from '$stores/plugin';
  import * as Sidebar from '$components/base/sidebar';
  import * as DropdownMenu from '$components/base/dropdown-menu';
</script>

<Sidebar.Footer>
  <Sidebar.Menu>
    <Sidebar.MenuItem>
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          {#snippet child({ props })}
            <Sidebar.MenuButton
              size="lg"
              class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground h-16 justify-start"
              {...props}
            >
              {#if $currentPlugin}
                {@const PluginIcon = resolveIcon($currentPlugin)}
                {#key $currentPlugin}
                  <div class="mr-1 flex size-8 items-center">
                    <PluginIcon size="2rem" />
                  </div>
                  <div class="flex flex-col items-start gap-1">
                    <span class="text-muted-foreground text-xs">Select a plugin</span>
                    <span class="text-base font-semibold">{$currentPlugin}</span>
                  </div>
                {/key}
              {/if}
            </Sidebar.MenuButton>
          {/snippet}
        </DropdownMenu.Trigger>

        <DropdownMenu.Content side="top" class="w-[--bits-dropdown-menu-anchor-width]">
          {#each plugins as plugin (plugin.name)}
            {@const PluginIcon = resolveIcon(plugin.name as TauriPlugin)}
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
