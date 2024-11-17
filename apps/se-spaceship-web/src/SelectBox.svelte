<script lang="ts">
  export let value: number | string | undefined = undefined;
</script>

<select class="button" bind:value on:input>
  <slot />
</select>

<style>
  select.button {
    vertical-align: middle;
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;
    background-image: url("data:image/svg+xml;utf8,<svg fill='black' height='24' viewBox='0 0 24 24' width='24' xmlns='http://www.w3.org/2000/svg'><path d='M7 10l5 5 5-5z'/><path d='M0 0h24v24H0z' fill='none'/></svg>");
    background-repeat: no-repeat;
    padding-right: 26px;
    height: 5.5ex;
    background-position-x: calc(100% - 5px);
    background-position-y: 6px;
    font-weight: normal;
    font-family: "Titillium Web", sans-serif;
  }

  .button {
    background-color: #8e8e8e;
    padding: 10px 12px 10px 12px;
    font-size: 100%;
    text-align: left;
    color: #000;
    font-weight: 600;
    display: inline-block;
    vertical-align: baseline;
    min-width: 128px;
    border: none;
    line-height: inherit;
    white-space: nowrap;
    box-shadow: inset 8px 0px 4px -8px #000, inset -8px 0px 4px -8px #000,
      inset 0px 10px 2px -8px #e3e3e3, inset 0px 10px 2px -8px #282828,
      inset 0px -9px 2px -8px #000, 0px 0px 4px 0px #000;
    position: relative;
    margin-right: 14px;
    cursor: pointer;
    -webkit-user-select: none;
    -moz-user-select: none;
    user-select: none;
    height: 36px;
    text-align: left;
  }

  .button:hover,
  .button:focus {
    color: #000;
    text-decoration: none;
    outline: 0;
    box-shadow: inset 8px 0px 4px -8px #000, inset -8px 0px 4px -8px #000,
      inset 0px 9px 2px -8px #fff, inset 0px 8px 4px -8px #000,
      inset 0px -8px 4px -8px #000, inset 0px -9px 2px -8px #432400,
      0px 0px 4px 0px #000, inset 0px 0px 4px 2px #f9b44b;
    background-color: #e39827;
    filter: drop-shadow(0 0 2px #f9b44b);
  }
  .button:active {
    position: relative;
    padding-top: 12px;
    padding-bottom: 8px;
    vertical-align: -2px;
    box-shadow: inset 0px 10px 2px -8px #000, inset 0px 9px 2px -8px #000,
      inset 8px 0px 4px -8px #563a10, inset 8px 0px 4px -8px #563a10,
      inset -8px 0px 4px -8px #563a10, inset -8px 0px 4px -8px #563a10,
      inset 0px 9px 2px -8px #563a10, inset 0px -9px 2px -8px #563a10,
      inset 0px -8.5px 0px -8px #563a10, 0px 0px 4px 0px #000;
    background-color: #f1be64;
    filter: none;
    outline: 0;
  }
  /* .button.disabled,
  .button.disabled:hover,
  .button.disabled:active {
    padding-top: 10px;
    padding-bottom: 10px;
    cursor: default;
    vertical-align: 0;
    background-color: #3d3d3d;
    color: #818181;
    box-shadow: inset 8px 0px 4px -8px #000, inset -8px 0px 4px -8px #000,
      inset 0px 8px 4px -8px #000, inset 0px -6px 4px -8px #818181,
      inset 0px -8px 4px -8px #000, 0px 0px 4px 0px #000;
    filter: none;
  } */

  /* why are the buttons so huge compared to other elements */
  select.button {
    height: auto;
    padding: 0.25rem 26px 0.25rem 0.5rem;
    background-position-y: 3px;
  }

  select.button:active {
    padding-top: 0.35rem;
    padding-bottom: 0.15rem;
  }
</style>
