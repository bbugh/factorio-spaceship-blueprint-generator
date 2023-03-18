<script lang="ts">
  export let id = "";
  export let value: number | string | undefined = undefined;
  export let disabled = false;
  let klass = "";
  export { klass as class };
</script>

<!-- eslint-disable @typescript-eslint/no-unsafe-member-access -->
<input
  {id}
  class="ui-input {klass}"
  type="text"
  {disabled}
  bind:value
  on:change
/>

<style>
  .ui-input {
    --shadow-spread: 2px;
    --shadow-color: rgba(0, 0, 0, 0.75);
    background-color: #828282;
    font-family: var(--ui-font-family);
    color: black; /*! border: 0; */
    box-shadow: inset 0px -1px 0px HSLA(300, 0.5%, 42.2%, 0%),
      inset 1px 0px 1px rgba(0, 0, 0, 0.5), inset 0px 1px 1px rgba(0, 0, 0, 0.5),
      inset -1px 0px 0px rgba(0, 0, 0, 0.5);
    padding-left: 5px;
    border-radius: 3px;
    /*! box-shadow: inset 0.2em 0.2em 0.2em 0 rgba(255,255,255,0.5), inset -0.2em -0.2em 0.2em 0 rgba(0,0,0,0.5); */
    border: 1px;
    /*! border-left: 1px solid #2E2C2B; */ /*! border-right: 1px solid #2E2C2B; */ /*! border-top: 1px solid #312F2E; */ /*! border-bottom: 1px solid #676566; */
    border-bottom: var(--inset-border-bottom);
    /*! border: 0; */ /*! border-bottom: 1px solid #44403F; */ /*! border-bottom-color: #312F2E; */ /*! border: 0; */
    /* box-shadow: inset 1px 0px var(--shadow-spread) var(--shadow-color), inset 0px 1px var(--shadow-spread) var(--shadow-color), inset -1px 0px var(--shadow-spread) var(--shadow-color), 0px 0px 1px #312F2E; */
    box-shadow: var(--inset-shadow);
    /*! border: 1px solid black; */ /*! border-inline: solid; */ /*! border-collapse: collapse; */ /*! border-spacing: 1px; */ /*! box-shadow: inset 1px 0px 1px rgba(0, 0, 0, 0.75), inset 0px 1px 1px rgba(0, 0, 0, 0.75), inset -1px 0px 1px rgba(0, 0, 0, 0.75); */
    height: 1.75em;
  }

  .ui-input:focus {
    background-color: #f6d9af;
    outline: 2px solid var(--ui-highlight-color);
  }

  .ui-input:disabled {
    background-color: #303030;
    color: #4c4b4b;
  }
</style>
