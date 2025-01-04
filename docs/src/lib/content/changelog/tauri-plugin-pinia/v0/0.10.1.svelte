<script lang="ts">
  import data from './data/0.10.1.json';
  import { Ext } from '$lib/components/link';
  import { Changelog } from '$lib/components/container';

  const version = '0.10.1';
</script>

<Changelog.Version {version} />

<Changelog.Features {version}>
  <ul>
    <li>Expose <Ext href={data.url.State} code>State</Ext> type.</li>
  </ul>
</Changelog.Features>
