<script lang="ts">
  import '../app.css';
  import { Layout } from '$layout';
  import { ModeWatcher } from 'mode-watcher';
  import * as Sidebar from '$layout/sidebar';
  import metadata from '$lib/data/metadata.json';
  import { currentPlugin } from '$stores/plugin';
  import { onMount, type Snippet, tick } from 'svelte';
  import { isHighlighterReady, loadHighlighter } from '$lib/code';

  const { children }: { children: Snippet } = $props();

  if (!isHighlighterReady()) {
    loadHighlighter();
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
