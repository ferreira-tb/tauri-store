<script lang="ts">
  import data from './data/0.9.0.json';
  import { Ext } from '$lib/components/link';
  import { Changelog } from '$lib/components/container';

  const version = '0.9.0';
</script>

{#snippet ext(key: keyof typeof data.url)}
  <Ext href={data.url[key]} code>{key}</Ext>
{/snippet}

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>Update {@render ext('tauri-store')} to <code>0.4.0</code>.</li>
  </ul>
</Changelog.BreakingChanges>

<Changelog.Features {version}>
  <ul>
    <li>
      Add {@render ext('saveInterval')}, {@render ext('saveStrategy')}, and
      {@render ext('saveOnChange')} options.
    </li>
  </ul>
</Changelog.Features>

<Changelog.BugFixes {version}>
  <ul>
    <li>Merge options from each store with the global options.</li>
  </ul>
</Changelog.BugFixes>
