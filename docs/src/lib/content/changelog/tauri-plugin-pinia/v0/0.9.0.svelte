<script lang="ts">
  import data from './data/0.9.0.json';
  import { Ext } from '$lib/components/link';
  import { Changelog } from '$lib/components/container';

  const version = '0.9.0';
</script>

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>
      Update <Ext href={data.url['tauri-store']} code>tauri-store</Ext> to <code>0.4.0</code>.
    </li>
  </ul>
</Changelog.BreakingChanges>

<Changelog.Features {version}>
  <ul>
    <li>
      Add <code>saveInterval</code>, <code>saveStrategy</code>, and
      <code>saveOnChange</code> options.
    </li>
  </ul>
</Changelog.Features>

<Changelog.BugFixes {version}>
  <ul>
    <li>Merge options from each store with the global options.</li>
  </ul>
</Changelog.BugFixes>
