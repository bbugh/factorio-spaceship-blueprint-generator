<script lang="ts">
  export let title: string | undefined = undefined;
</script>

<div class="ui-input-group-outer">
  <div class="ui-input-group-inner">
    {#if title}
      <div class="ui-input-group-title pt-2 pl-2 font-semibold">
        {title}
      </div>
    {/if}
    <div class="flex flex-col gap-3 empty:hidden">
      <slot />
    </div>
  </div>
</div>

<style>
  :global(:root) {
    --ui-input-group-border-color: #2f2e2f;
    --ui-input-group-border-dark: #242221;
    --ui-input-group-border-light: #5a5858;

    --ui-input-group-border: 1px solid var(--ui-input-group-border-color);
  }

  .ui-input-group-outer {
    border: var(--ui-input-group-border);
    border-top-color: var(--ui-input-group-border-dark);
    border-bottom-color: var(--ui-input-group-border-light);
  }

  .ui-input-group-inner {
    border: var(--ui-input-group-border);
    border-top-color: var(--ui-input-group-border-light);
    border-bottom-color: var(--ui-input-group-border-dark);
  }

  .ui-input-group-title {
    color: var(--factorio-panel-title-font-color);
  }
</style>
