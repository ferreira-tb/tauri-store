<script lang="ts">
  import { mode } from './mode';
  import Footer from './sidebar-footer.svelte';
  import Header from './sidebar-header.svelte';
  import MenuItem from './sidebar-menu-item.svelte';
  import * as Sidebar from '$components/ui/sidebar';
  import { useSidebar } from '$components/ui/sidebar';
  import { changelogs, javascriptDocs, rustDocs } from '$lib/data';

  const sidebar = useSidebar();
  const isLearnMode = $derived.by(() => {
    return !sidebar.isMobile || !$mode || $mode === 'learn';
  });
</script>

<Sidebar.Root>
  <Header />

  <Sidebar.Content>
    {#if isLearnMode}
      <Sidebar.Group>
        <Sidebar.GroupLabel>Guide</Sidebar.GroupLabel>
        <Sidebar.GroupContent>
          <Sidebar.Menu>
            {@render guide('getting-started', 'Getting started')}
            {@render guide('persisting-state', 'Persisting state')}
            {@render guide('synchronization', 'Synchronization')}
            {@render guide('accessing-from-rust', 'Accessing from Rust')}
          </Sidebar.Menu>
        </Sidebar.GroupContent>
      </Sidebar.Group>
    {:else if $mode === 'reference'}
      <Sidebar.Group>
        <Sidebar.GroupLabel>Javascript</Sidebar.GroupLabel>
        <Sidebar.GroupContent>
          <Sidebar.Menu>
            {#each javascriptDocs as doc (doc.label)}
              <MenuItem href={doc.url} label={doc.label} external />
            {/each}
          </Sidebar.Menu>
        </Sidebar.GroupContent>
      </Sidebar.Group>

      <Sidebar.Group>
        <Sidebar.GroupLabel>Rust</Sidebar.GroupLabel>
        <Sidebar.GroupContent>
          <Sidebar.Menu>
            {#each rustDocs as doc (doc.label)}
              <MenuItem href={doc.url} label={doc.label} external />
            {/each}
          </Sidebar.Menu>
        </Sidebar.GroupContent>
      </Sidebar.Group>
    {:else if $mode === 'changelog'}
      <Sidebar.Group>
        <Sidebar.GroupLabel>Changelog</Sidebar.GroupLabel>
        <Sidebar.GroupContent>
          <Sidebar.Menu>
            {#each changelogs as changelog (changelog.label)}
              <MenuItem href={changelog.url} label={changelog.label} />
            {/each}
          </Sidebar.Menu>
        </Sidebar.GroupContent>
      </Sidebar.Group>
    {/if}
  </Sidebar.Content>

  <Footer />
</Sidebar.Root>

{#snippet guide(path: string, label: string)}
  {@const href = `/tauri-store/guide/${path}`}
  <MenuItem {href} {label} />
{/snippet}
