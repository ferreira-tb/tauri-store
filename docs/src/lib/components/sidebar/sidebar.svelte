<script lang="ts">
  import { mode } from './mode';
  import Link from '../link.svelte';
  import Footer from './sidebar-footer.svelte';
  import Header from './sidebar-header.svelte';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import { changelogs, javascriptDocs, rustDocs } from '$lib/data';
  import { useSidebar } from '$lib/components/ui/sidebar/index.js';

  const sidebar = useSidebar();
  const isLearnMode = $derived.by(() => {
    return !sidebar.isMobile || !$mode || $mode === 'learn';
  });

  function closeMobileSidebar() {
    sidebar.openMobile &&= false;
  }
</script>

{#snippet menuItem(href: string, label: string, external = false)}
  <Sidebar.MenuItem>
    <Sidebar.MenuButton>
      {#snippet child({ props })}
        <Link {...props} {href} {external} onclick={closeMobileSidebar}>
          {label}
        </Link>
      {/snippet}
    </Sidebar.MenuButton>
  </Sidebar.MenuItem>
{/snippet}

{#snippet guide(path: string, label: string)}
  {@const href = `/tauri-store/guide/${path}`}
  {@render menuItem(href, label)}
{/snippet}

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
              {@render menuItem(doc.url, doc.label, true)}
            {/each}
          </Sidebar.Menu>
        </Sidebar.GroupContent>
      </Sidebar.Group>

      <Sidebar.Group>
        <Sidebar.GroupLabel>Rust</Sidebar.GroupLabel>
        <Sidebar.GroupContent>
          <Sidebar.Menu>
            {#each rustDocs as doc (doc.label)}
              {@render menuItem(doc.url, doc.label, true)}
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
              {@render menuItem(changelog.url, changelog.label)}
            {/each}
          </Sidebar.Menu>
        </Sidebar.GroupContent>
      </Sidebar.Group>
    {/if}
  </Sidebar.Content>

  <Footer />
</Sidebar.Root>
