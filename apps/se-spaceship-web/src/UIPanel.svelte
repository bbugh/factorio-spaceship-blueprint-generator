<script lang="ts">
  export let title = "Title";
</script>

<div class="ui-panel">
  <div class="ui-panel-title">
    <span class="ui-panel-title-text">{title}</span>
    <!-- eslint-disable-next-line @typescript-eslint/no-unsafe-member-access https://github.com/sveltejs/rfcs/pull/38 -->
    {#if $$slots.titleActions}
      <div class="ui-panel-title-actions">
        <slot name="titleActions" />
      </div>
    {/if}
  </div>
  <!-- eslint-disable-next-line @typescript-eslint/no-unsafe-member-access https://github.com/sveltejs/rfcs/pull/38 -->
  {#if $$slots.insetActions}
    <div class="ui-inset-actions">
      <slot name="insetActions" />
    </div>
  {/if}
  <div class="ui-inset flex flex-col gap-3">
    <slot />
  </div>
  <!-- eslint-disable-next-line @typescript-eslint/no-unsafe-member-access https://github.com/sveltejs/rfcs/pull/38 -->
  {#if $$slots.bottom}
    <slot name="bottom" />
  {/if}
</div>

<style>
  .ui-panel {
    --shadow-distance: 2px;
    background-color: var(--ui-header-background);
    padding: var(--ui-padding-size);
    padding-top: var(--ui-padding-half);
    box-shadow: inset 0 var(--shadow-distance) 1px #5a5653,
      inset var(--shadow-distance) 0 1px rgba(0, 0, 0, 0.5),
      inset calc(-1 * var(--shadow-distance)) 0 var(--shadow-distance)
        rgba(0, 0, 0, 0.5),
      inset 0px calc(-1 * var(--shadow-distance)) var(--shadow-distance)
        rgba(0, 0, 0, 0.75);
    border-radius: 2px;
    /* box-shadow: inset 0 2px 0 #5A5653, inset 0 2px 2px rgba(0,0,0,0.3), inset 0 0 4px 1px rgba(0,0,0,0.2), inset 0 -3px 2px rgba(0,0,0,.15), inset 0 0 4px 1px rgba(0,0,0,.1), inset 0 3px 2px rgba(0,0,0,.2) */
  }

  .ui-panel-title {
    color: var(--factorio-panel-title-font-color);
    font-weight: bold;
    margin-bottom: var(--ui-padding-half);
    text-align: left;
    background-image: url("/images/panel-vent.png");
    background-position: center;
    background-size: 4px 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .ui-panel-title-text {
    background-color: var(--ui-header-background);
    padding-right: 15px;
    font-size: 18px;
  }

  .ui-panel-title-actions {
    background-color: var(--ui-header-background);
    padding-left: 15px;
  }

  .ui-inset {
    background-color: var(--ui-inset-background);
    box-shadow: var(--ui-inset-shadow);
    padding: var(--ui-padding-half);
    border-radius: 1px;
  }

  .ui-inset-actions {
    --shadow: inset 2px 2px 3px rgba(0, 0, 0, 0.75),
      inset -2px 0 2px rgba(0, 0, 0, 0.5);
    background-color: #242324;
    box-shadow: var(--shadow);
    text-align: right;
    display: flex;
    justify-content: flex-end;
    padding: var(--ui-padding-half);
  }

  .ui-panel-title-actions:empty,
  .ui-inset-actions:empty {
    display: none;
  }
</style>
