<script lang="ts">
  import { Ext } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '0.5.0';

  const prelude = 'https://docs.rs/tauri-store/0.5.0/tauri_store/prelude/index.html';
</script>

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>Remove <code>unstable-async</code> feature.</li>
  </ul>
</Changelog.BreakingChanges>

<Changelog.Enhancements {version}>
  <ul>
    <li>Add <Ext href={prelude} code>prelude</Ext> module.</li>
  </ul>
</Changelog.Enhancements>
