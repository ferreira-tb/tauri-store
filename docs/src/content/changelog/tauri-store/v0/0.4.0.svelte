<script lang="ts">
  import data from './data/0.4.0.json';
  import { Ext } from '$components/link';
  import { Feature } from '$components/badge';
  import { Changelog } from '$components/container';

  const version = '0.4.0';
</script>

{#snippet ext(key: keyof typeof data.url)}
  <Ext href={data.url[key]} code>{key}</Ext>
{/snippet}

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>
      Take <code>&mut self</code> in
      {@render ext('Store::watch')} and {@render ext('Store::unwatch')}.
    </li>
    <li>
      Return <code>T</code> instead of <code>Result&ltT, Error&gt</code>
      from {@render ext('with_store')} functions.
    </li>
    <li>Remove <code>ahash</code> feature.</li>
    <li>
      <Feature name="unstable-async" />Remove <code>boxed</code> and <code>boxed_ok</code> macros.
    </li>
    <li>
      <Feature name="unstable-async" />{@render ext('Store::set')},
      {@render ext('Store::patch')}, and {@render ext('Store::patch_with_source')} are now async.
    </li>
  </ul>
</Changelog.BreakingChanges>

<Changelog.Features {version}>
  <ul>
    <li>
      Add {@render ext('StoreCollection::default_save_strategy')} and
      {@render ext('StoreCollectionBuilder::default_save_strategy')}.
    </li>
    <li>
      Add {@render ext('StoreCollection::save_now')},
      {@render ext('StoreCollection::save_some_now')}, and
      {@render ext('StoreCollection::save_all_now')}, to save a store immediately, ignoring the save
      strategy.
    </li>
    <li>Add {@render ext('Store::save_on_change')}.</li>
    <li>Add {@render ext('Store::save_now')}.</li>
    <li>Add {@render ext('Store::set_options')}.</li>
    <li>
      Add {@render ext('Store::save_strategy')} and {@render ext('Store::set_save_strategy')}.
    </li>
    <li>Allow debouncing and throttling when saving the stores.</li>
    <li>
      <Feature name="unstable-async" /> Add <code>boxed</code> function.
    </li>
  </ul>
</Changelog.Features>

<Changelog.Enhancements {version}>
  <ul>
    <li>
      {@render ext('StoreCollectionBuilder::autosave')},
      {@render ext('StoreCollection::clear_autosave')}, and
      {@render ext('StoreCollection::set_autosave')} are no longer gated by the
      <code>unstable-async</code> feature.
    </li>
  </ul>
</Changelog.Enhancements>

<Changelog.BugFixes {version}>
  <ul>
    <li>Consume the first autosave tick immediately before starting the interval.</li>
  </ul>
</Changelog.BugFixes>

<Changelog.Performance {version}>
  <ul>
    <li>
      Use the {@render ext('ResourceTable')} to manage each store independently, instead of using a single
      hash map for all of them.
    </li>
  </ul>
</Changelog.Performance>
