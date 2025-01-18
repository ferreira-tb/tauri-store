<script lang="ts">
  import { npm } from '$lib/url';
  import { Link } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '1.1.1';
</script>

<Changelog.Version {version} />

<Changelog.Dependencies {version}>
  <ul>
    <li>Update <Link href={npm.shared} code>@tauri-store/shared</Link> to <code>0.4.1</code>.</li>
  </ul>
</Changelog.Dependencies>
