<script lang="ts" module>
  import type { HTMLAttributes } from 'svelte/elements';

  export type ChangeKind =
    | 'Breaking changes'
    | 'Bug fixes'
    | 'Dependencies'
    | 'Documentation'
    | 'Enhancements'
    | 'Features'
    | 'Performance';

  export type BaseProps = HTMLAttributes<HTMLElement> & {
    children?: Snippet;
    class?: string;
    version: string;
  };
</script>

<script lang="ts">
  import type { Snippet } from 'svelte';
  import { kebabCase } from 'change-case';
  import Container from '../../container.svelte';

  interface Props extends BaseProps {
    kind: ChangeKind;
  }

  const { kind, version, children, ...restProps }: Props = $props();

  const id = $derived.by(() => `v${kebabCase(version)}-${kebabCase(kind)}`);
</script>

<Container {...restProps} title={kind} {id} level={3}>
  {@render children?.()}
</Container>
