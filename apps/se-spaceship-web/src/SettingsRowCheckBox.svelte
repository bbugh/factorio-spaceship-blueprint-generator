<script lang="ts">
  export let checked: boolean;
</script>

<div class="checkbox-label">
  <input type="checkbox" bind:checked on:input />
  <div class="checkbox" />
</div>

<style>
  /**
  Taken directly from https://www.factorio.com/static/css/main.css?v=5762b6ba
  */

  .checkbox-label {
    margin-top: 8px;
    margin-bottom: 8px;
    display: flex;
    align-content: center;
    position: relative;
    cursor: pointer;
    line-height: 1.2;
    break-inside: avoid;
  }
  .checkbox-label > div {
    width: 100%;
    padding-left: 8px;
  }
  .checkbox-label input {
    position: relative;
    top: 0px;
    left: 0px;
    width: 16px;
    height: 16px;
    padding: 6px;
    position: absolute;
    opacity: 0;
    cursor: pointer;
  }
  .checkbox-label .checkbox {
    margin: auto;
    position: relative;
    top: 0px;
    left: 0px;
    width: 16px;
    height: 16px;
    padding: 6px;
    background: #8e8e8e;
    box-shadow: inset 0px 4px 2.5px -2.5px #111, inset 2px 0px 2px 0px #323232,
      inset -2px 0px 2px 0px #323232, inset 0px -1px 1px 0px #fff,
      inset 0px -4px 2px -2px #8e8e8e, 0px 0px 4px 1px #2e2521;
  }
  .checkbox-label .checkbox:hover,
  .checkbox-label .checkbox:active,
  .checkbox-label input:focus ~ .checkbox {
    background: #e39827;
    box-shadow: inset 0px 4px 2px -2px #412a07, inset 2px 0px 2px 0px #5a3c10,
      inset -2px 0px 2px 0px #5a3c10, inset 0px -2px 2px 0px #d0ae79,
      inset 0px -2px 4px 0px #c78627, 0px 0px 4px 1px #786b4f;
  }
  .checkbox-label .checkbox:active {
    background: #f3c98e;
    box-shadow: inset 0px 4px 2px -2px #412a07, inset 2px 0px 2px 0px #5a3c10,
      inset -2px 0px 2px 0px #5a3c10, inset 0px -2px 2px 0px #d0ae79,
      inset 0px -2px 4px 0px #c78627, inset 0px 1px 2.5px 4.5px #e39827,
      0px 0px 4px 1px #786b4f;
  }
  .checkbox-label .checkbox:disabled {
    background: #313031;
    box-shadow: inset 0px 4px 2.5px -2.5px #111, inset 2px 0px 2px 0px #323232,
      inset -2px 0px 2px 0px #323232, inset 0px -1px 1px 0px #666,
      inset 0px -4px 2px -2px #8e8e8e, 0px 0px 4px 1px #2e2521;
  }
  .checkbox-label input:checked ~ .checkbox {
    background: #e39827;
  }
  .checkbox-label input:checked ~ .checkbox:active {
    background: #f3c98e;
  }
  .checkbox-label .checkbox::after {
    position: absolute;
    content: "";
  }
  .checkbox-label input:checked ~ .checkbox::after {
    -webkit-transform: rotate(45deg) scale(1);
    -ms-transform: rotate(45deg) scale(1);
    transform: rotate(45deg) scale(1);
    opacity: 1;
    left: 6px;
    top: 3px;
    width: 2px;
    height: 6px;
    border: solid #282728;
    border-width: 0 2.5px 2.5px 0;
    background-color: transparent;
    border-radius: 0;
  }

  /** override checkbox-label margin because that's handled by flexbox */
  .checkbox-label {
    margin-top: 0;
    margin-bottom: 0;
  }

  /** override the checkbox label position because it doesn't work with tailwind by default */
  .checkbox-label input:checked ~ .checkbox::after {
    left: 6px;
    top: 3px;
    width: 5px;
    height: 9px;
  }

  /** checkbox isn't clickable without front z-index, not sure why it's different */
  .checkbox-label input {
    z-index: 1;
  }
</style>
