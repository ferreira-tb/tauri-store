<script lang="ts">
  import { Changelog } from '$components/container';

  const version = '0.8.0';
</script>

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>
      Add <code>plugin</code> feature. Now it's possible use <code>tauri-store</code> directly as a
      Tauri plugin. This is enabled by default, so anyone developing a plugin based on the
      <code>tauri-store</code> crate should make sure to disable this feature.
    </li>
  </ul>
</Changelog.BreakingChanges>
