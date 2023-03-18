<script lang="ts">
  import Slider from "./Slider.svelte";
  import SettingRowTextInput from "./SettingRowTextInput.svelte";

  export let id = "";
  export let value: number | undefined = undefined;
  export let min: number | undefined = undefined;
  export let max: number | undefined = undefined;
  export let step: number | undefined = undefined;
  export let disabled = false;
</script>

<div class="flex gap-2">
  <Slider {id} {min} {max} {step} {disabled} bind:value on:input on:change />
  <SettingRowTextInput class="w-16" bind:value on:change {disabled} />
</div>
