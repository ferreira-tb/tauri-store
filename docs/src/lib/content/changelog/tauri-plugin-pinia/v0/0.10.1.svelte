<script lang="ts">
  import { Changelog } from '$lib/components/container';

  const version = '0.10.1';
</script>

<Changelog.Version {version} />

<Changelog.Features {version}>
  <ul>
    <li>Expose <code>State</code> type.</li>
  </ul>
</Changelog.Features>
