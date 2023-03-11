<script lang="ts">
  import SettingRowTextInput from "./SettingRowTextInput.svelte";

  export let id = "";
  export let value: number | string | undefined = undefined;
  export let min: number | string | undefined = undefined;
  export let max: number | string | undefined = undefined;
  export let step: number | string | undefined = undefined;
  export let disabled = false;
</script>

<div class="flex gap-2">
  <input
    {id}
    type="range"
    {min}
    {max}
    {step}
    {disabled}
    class="slider"
    bind:value
    on:input
    on:change
  />
  <SettingRowTextInput class="w-16" bind:value on:change {disabled} />
</div>
