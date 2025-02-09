<script lang="ts">
  import { Changelog } from '$components/container';

  const version = '0.7.2';
</script>

<Changelog.Version {version} />

<Changelog.Documentation {version}>
  <ul>
    <li>Update README file.</li>
  </ul>
</Changelog.Documentation>
