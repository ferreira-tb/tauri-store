<script lang="ts">
  import { Changelog } from '$lib/components/container';

  const version = '0.9.1';
</script>

<Changelog.Version {version} />

<Changelog.Documentation {version}>
  <ul>
    <li>Fix broken link in the README.</li>
  </ul>
</Changelog.Documentation>
