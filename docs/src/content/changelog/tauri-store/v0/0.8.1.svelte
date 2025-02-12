<script lang="ts">
  import { Ext } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '0.8.1';

  const url = 'https://docs.rs/tauri-store/0.8.1/tauri_store';
</script>

<Changelog.Version {version} />

<Changelog.Documentation {version}>
  <ul>
    <li>
      Ensure that Cargo features are properly documented on <Ext href={url} code>docs.rs</Ext>.
    </li>
  </ul>
</Changelog.Documentation>
