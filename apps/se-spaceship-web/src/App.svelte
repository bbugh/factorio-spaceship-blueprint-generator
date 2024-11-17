<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    generate,
    loadFile,
    maxSizeMax,
    resetStore,
    store,
    tiles,
  } from "./store";

  import Dropzone from "svelte-file-dropzone";
  import InputGroup from "./InputGroup.svelte";
  import SettingRow from "./SettingRow.svelte";
  import SettingRowSlider from "./SettingRowSlider.svelte";
  import SettingRowTextInput from "./SettingRowTextInput.svelte";
  import UIPanel from "./UIPanel.svelte";

  import trashIconBlack from "./assets/images/trash-icon-black.png";
  import trashIconWhite from "./assets/images/trash-icon-white.png";

  const defaultAlpha = 1;
  const defaultMaxSize = 50;

  let alpha = defaultAlpha;
  let error = "";
  let maxSize = defaultMaxSize;
  let queued = false;

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  function reset() {
    resetStore();

    alpha = defaultAlpha;
    error = "";
    maxSize = defaultMaxSize;
    queued = false;

    refreshCanvas();
  }

  function handleFilesSelect(
    e: CustomEvent<{
      acceptedFiles: File[];
      fileRejections: File[];
      event: MouseEvent;
    }>
  ) {
    const { acceptedFiles, fileRejections } = e.detail;

    if (fileRejections.length > 0) {
      alert(
        "File type not supported! Images must be in GIF, JPEG, PNG, or WEBP format."
      );
      return;
    }

    if (acceptedFiles.length === 0) {
      alert("No files selected!");
      return;
    }

    if (acceptedFiles.length > 1) {
      alert("Only one file can be processed at a time!");
      return;
    }

    loadFile(acceptedFiles[0]);
  }

  // Needs to happen after <img> has loaded the source so we can get the natural image dimensions
  function onSourceImageLoaded(e: Event) {
    console.log("onSourceImageLoaded");
    const source = e.target as HTMLImageElement;

    $store.sourceWidth = source.naturalWidth;
    $store.sourceHeight = source.naturalHeight;

    onActivity();
  }

  function refreshCanvas() {
    if (!$store.imageData) {
      return;
    }

    console.log("refreshCanvas");

    // This can't be done onMount because the canvas doesn't exist in the DOM
    // yet, and it is temporarily removed from the DOM when the image is loaded
    ctx = canvas.getContext("2d");

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    const { imageData, sourceWidth } = $store;

    canvas.width = imageData.width;
    canvas.height = imageData.height;

    canvas.style.width = `${sourceWidth}px`;
    canvas.style.height = `auto`;

    ctx.putImageData(imageData, 0, 0, 0, 0, imageData.width, imageData.height);
  }

  function onActivity() {
    console.log("onActivity");
    if (!queued) {
      queued = true;
      requestIdleCallback(() => {
        try {
          error = "";
          console.log("generate");
          generate(maxSize, alpha, $tiles.floor, $tiles.wall);
          refreshCanvas();
        } catch (e) {
          error = (e as Error).message.toString();
        }

        queued = false;
      });
    }
  }

  async function copyBlueprintToClipboard() {
    await navigator.clipboard.writeText($store.blueprint);
  }

  function onPaste(event: ClipboardEvent) {
    const items = event.clipboardData.items;

    for (let i = 0; i < items.length; i++) {
      if (
        items[i].type === "image/gif" ||
        items[i].type === "image/jpeg" ||
        items[i].type === "image/png" ||
        items[i].type === "image/webp"
      ) {
        loadFile(items[i].getAsFile());
        break;
      }
    }
  }

  onMount(() => {
    window.addEventListener("paste", onPaste);
  });

  onDestroy(() => {
    window.removeEventListener("paste", onPaste);
  });
</script>

<div class="mb-4 flex flex-col items-center justify-between md:flex-row">
  <div>
    <h1 class="text-5xl">Spaceship Generator</h1>
  </div>
  <div class="flex items-center">
    <a href="https://factorio.com/"
      ><img
        src="/images/factorio-logo-sm.png"
        alt="Factorio logo"
        class="w-60"
      /></a
    >
    <a href="https://www.factorio.com/space-age/overview">
      <img
        src="/images/space-age-logo-sm.png"
        alt="Space Age logo"
        class="w-60"
      />
    </a>
    <a href="https://mods.factorio.com/mod/space-exploration">
      <img
        src="/images/space-exploration-logo-sm.png"
        alt="Space Exploration logo"
        class="w-60"
      />
    </a>
  </div>
</div>

<main class="flex flex-col gap-4 lg:flex-row">
  <div class="flex w-full flex-col gap-4">
    <UIPanel title="Input">
      <div slot="insetActions">
        <button
          class="btn btn-icon btn-danger"
          on:click={() => reset()}
          disabled={!$store.inputSrc}
        >
          <img
            src={$store.inputSrc ? trashIconBlack : trashIconWhite}
            alt="Reset"
          />
        </button>
      </div>
      <InputGroup>
        <SettingRow
          title="Alpha Threshold"
          hint="Any pixel with an alpha channel (opacity) value less than the threshold will be removed. This can be helpful if the image you're importing has transparency but does not have sharp edges."
          let:inputId
        >
          <SettingRowSlider
            id={inputId}
            min={1}
            max={254}
            disabled={!$store.inputSrc}
            bind:value={alpha}
            on:change={onActivity}
          />
        </SettingRow>

        <SettingRow
          title="Blueprint Base"
          hint="The game or mod that will be used to create the blueprint."
        >
          <select bind:value={$store.module} on:change={onActivity}>
            <option value="space-age">Space Age</option>
            <option value="space-exploration">Space Exploration</option>
            <option value="custom">Custom</option>
          </select>
        </SettingRow>

        {#if $store.module === "custom"}
          <SettingRow
            title="Floor Tile"
            hint="The game identifier of the floor tile you want to use, if you want something different than the Space Exploration default."
            let:inputId
          >
            <SettingRowTextInput
              id={inputId}
              bind:value={$store.customFloorTile}
              on:change={onActivity}
            />
          </SettingRow>

          <SettingRow
            title="Wall Entity"
            hint="The game identifier of the wall entity you want to use, if you want something different than the Space Exploration default."
            let:inputId
          >
            <SettingRowTextInput
              id={inputId}
              bind:value={$store.customWallTile}
              on:change={onActivity}
            />
          </SettingRow>
        {/if}
      </InputGroup>
      <InputGroup title="Source">
        {#if $store.inputSrc}
          <div class="flex flex-col items-center justify-center">
            <img
              id="inputImg"
              src={$store.inputSrc}
              alt="Input source"
              class="max-w-full"
              on:load={onSourceImageLoaded}
            />
            <div>{$store.sourceWidth}&times;{$store.sourceHeight}</div>
          </div>
        {:else}
          <div class="w-full p-2">
            <Dropzone
              on:drop={handleFilesSelect}
              accept="image/png,image/jpeg,image/webp,image/gif"
              multiple="false"
            >
              <div class="mx-auto text-center">
                Click to choose a GIF, JPEG, PNG, or WEBP image<br />or drag and
                drop it here.<br /> Best results are achieved with pixel art.
              </div>
            </Dropzone>
          </div>
        {/if}
      </InputGroup>
    </UIPanel>
    <div class="error">{error}</div>
  </div>

  <div class="w-full">
    <UIPanel title="Output">
      <InputGroup>
        <SettingRow
          title="Max Size"
          hint="The maximum width or height in tiles that the blueprint will be. The input image will be scaled down. 10 is an early game ship, 25 is a mid-game ship, 50 is a standard end-game ship, 100 is a very large ship, and anything 200 or above is so massive that you won't be able to use without cheating."
          let:inputId
        >
          <SettingRowSlider
            id={inputId}
            min={10}
            max={$maxSizeMax}
            step={10}
            disabled={!$store.inputSrc}
            bind:value={maxSize}
            on:input={onActivity}
          />
        </SettingRow>
      </InputGroup>
      {#if $store.inputSrc}
        <InputGroup title="Blueprint Preview">
          <div class="w-full text-center">
            <canvas
              bind:this={canvas}
              id="outputCanvas"
              class="pixelated mx-auto max-w-full"
              style="width: 0px; height: 0px"
            />
            <div>{$store.outputWidth}&times;{$store.outputHeight}</div>
          </div>
        </InputGroup>

        <InputGroup title="Components">
          <div class="flex w-full flex-row justify-between gap-2 p-2">
            <div class="inset-object flex w-full flex-auto justify-between p-2">
              <div>{$tiles.floor}</div>
              <div style="text-shadow: 1px 1px 3px black; font-weight: bold">
                {$store.outputTileCount}
              </div>
            </div>
            <div class="inset-object flex w-full flex-auto justify-between p-2">
              <div>{$tiles.wall}</div>
              <div style="text-shadow: 1px 1px 3px black; font-weight: bold">
                {$store.outputEntityCount}
              </div>
            </div>
          </div>
        </InputGroup>
      {/if}
      <div
        slot="bottom"
        class="flex justify-end pt-2"
        class:hidden={!$store.blueprint}
      >
        <button
          class="btn-copy-blueprint"
          disabled={!$store.blueprint}
          on:click={() => copyBlueprintToClipboard()}
          ><img src="/images/blueprint-icon.png" alt="icon" /> Copy Blueprint</button
        >
      </div>
    </UIPanel>
  </div>
</main>

<style>
  .pixelated {
    image-rendering: pixelated;
  }

  .inset-object {
    background-color: var(--factorio-panel-bg-color-dark);
    box-shadow: var(--ui-inset-shadow);
    border-radius: 1px;
  }

  .btn-copy-blueprint {
    display: inline-flex;
    gap: 5px;
    align-items: center;
  }

  .btn-copy-blueprint img {
    width: 1em;
    height: 1em;
  }
  /* .blueprint-text {
    border: 1px solid darkgray;
    font-family: monospace;
    border-radius: 0.15em;
    width: 750px;
    height: 250px;
  } */
  .error {
    color: red;
  }
  /*
  .external-logo {
    width: 200px;
  } */
  /* .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  } */
</style>
