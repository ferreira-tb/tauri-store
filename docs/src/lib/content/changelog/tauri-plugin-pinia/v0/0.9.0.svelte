<script lang="ts">
  import { Ext } from '$lib/components/link';
  import { Changelog } from '$lib/components/container';

  const version = '0.9.0';

  const url = {
    'tauri-store': 'https://docs.rs/tauri-store/0.4.0/tauri_store/',
  };
</script>

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>
      Update <Ext href={url['tauri-store']} code>tauri-store</Ext> to <code>0.4.0</code>.
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
