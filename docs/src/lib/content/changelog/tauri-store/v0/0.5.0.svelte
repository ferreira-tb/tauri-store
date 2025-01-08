<script lang="ts">
  import data from './data/0.5.0.json';
  import { Ext } from '$lib/components/link';
  import { Changelog } from '$lib/components/container';

  const version = '0.5.0';
</script>

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>Remove <code>unstable-async</code> feature.</li>
  </ul>
</Changelog.BreakingChanges>

<Changelog.Enhancements {version}>
  <ul>
    <li>Add <Ext href={data.url.prelude} code>prelude</Ext> module.</li>
  </ul>
</Changelog.Enhancements>
