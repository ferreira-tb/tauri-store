<script lang="ts">
  import { NPM_SHARED } from '$lib/url';
  import { Link } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '0.2.1';
</script>

<Changelog.Version {version} />

<Changelog.Dependencies {version}>
  <ul>
    <li>Update <Link href={NPM_SHARED} code>@tauri-store/shared</Link> to <code>0.4.1</code>.</li>
  </ul>
</Changelog.Dependencies>
