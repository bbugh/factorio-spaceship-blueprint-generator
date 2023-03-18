<script lang="ts">
  import InfoPopoverIcon from "./InfoPopoverIcon.svelte";
  import { hyphenate } from "./utils";

  export let title: string;
  export let hint = "";

  $: inputId = hyphenate(title);
</script>

<div
  class="flex w-full items-center justify-between gap-3 px-2 first:pt-2 last:pb-2"
>
  <label for={inputId} class="flex items-center gap-1 whitespace-nowrap">
    {title}
    <InfoPopoverIcon title={hint} />
  </label>
  <slot {inputId} />
</div>
