<script lang="ts">
  import '../app.css';
  import { Layout } from '$lib/layout';
  import { ModeWatcher } from 'mode-watcher';
  import metadata from '$lib/data/metadata.json';
  import * as Sidebar from '$lib/layout/sidebar';
  import { currentPlugin } from '$lib/stores/plugin';
  import { onMount, type Snippet, tick } from 'svelte';
  import { isHighlighterReady, loadHighlighter } from '$lib/code';

  const { children }: { children: Snippet } = $props();

  if (!isHighlighterReady()) {
    void loadHighlighter();
  }

  onMount(async () => {
    await tick();
    const search = new URLSearchParams(location.search);
    const plugin = search.get('plugin');
    if (plugin) {
      const data = metadata.find((item) => item.name === plugin);
      if (data?.isPlugin) currentPlugin.set(data.name);
    }
  });
</script>

<Sidebar.Provider>
  <Sidebar.Root />
  <ModeWatcher defaultMode="dark" />

  <Layout>
    {@render children()}
  </Layout>
</Sidebar.Provider>
