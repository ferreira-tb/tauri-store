<script lang="ts">
  import { Badge } from '../base/badge';

  interface Props {
    name: string;
  }

  const { name }: Props = $props();
</script>

<Badge variant="default" class="px-1 [&:not(:last-child)]:mr-2">{name}</Badge>
