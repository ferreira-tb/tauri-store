<script lang="ts">
  import data from './data/0.10.0.json';
  import { Ext } from '$lib/components/link';
  import { Changelog } from '$lib/components/container';

  const version = '0.10.0';
</script>

{#snippet ext(key: keyof typeof data.url, label?: string)}
  <Ext href={data.url[key]} code>{label ?? key}</Ext>
{/snippet}

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>Update {@render ext('tauri-store')} to <code>0.5.0</code>.</li>
    <li>
      Change the default directory name for stores from <code>pinia</code> to
      <code>tauri-plugin-pinia</code>. If you’re using a {@render ext('path', 'custom path')}, this
      change won’t affect you. Otherwise, you’ll need to either move your existing stores to the new
      default directory or manually set the path to match the previous configuration.
    </li>
  </ul>
</Changelog.BreakingChanges>
