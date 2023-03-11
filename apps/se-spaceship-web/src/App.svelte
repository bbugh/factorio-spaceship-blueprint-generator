<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  import Dropzone from "svelte-file-dropzone";
  // import Spinner from "./Spinner.svelte";
  import UIPanel from "./UIPanel.svelte";
  import { getBlueprintFromImage } from "image-to-blueprint";
  import SettingRow from "./SettingRow.svelte";
  import SettingRowSlider from "./SettingRowSlider.svelte";
  import SettingRowTextInput from "./SettingRowTextInput.svelte";
  import resetIconBlack from "./assets/reset-icon-black.png";
  import resetIconWhite from "./assets/reset-icon-white.png";
  import InputGroup from "./InputGroup.svelte";

  let blueprint = "";
  let error = "";
  let alpha = 1;
  let floorTile = "se-spaceship-floor";
  let wallTile = "se-spaceship-wall";

  let maxSize = 25;
  let maxSizeMax = 100;

  let queued = false;
  let generating = false;
  let inputSrc: string = null;
  let sourceWidth = 0;
  let sourceHeight = 0;
  let outputWidth = 0;
  let outputHeight = 0;
  let outputTileCount = 0;
  let outputEntityCount = 0;

  let imageBuffer: Uint8Array = null;

  function reset() {
    blueprint = "";
    error = "";
    alpha = 1;
    floorTile = "se-spaceship-floor";
    wallTile = "se-spaceship-wall";

    maxSize = 25;

    queued = false;
    generating = false;
    inputSrc = null;
    sourceWidth = 0;
    sourceHeight = 0;
    outputWidth = 0;
    outputHeight = 0;
    outputTileCount = 0;
    outputEntityCount = 0;

    imageBuffer = null;

    const canvas = document.getElementById("outputCanvas") as HTMLCanvasElement;
    const ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    canvas.style.width = `0`;
    canvas.style.height = `0`;
  }

  function handleFilesSelect(
    e: CustomEvent<{
      acceptedFiles: File[];
      fileRejections: File[];
      event: MouseEvent;
    }>
  ) {
    console.table(e);
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

  function loadFile(fileload: File) {
    const reader = new FileReader();

    reader.onload = () => {
      const arrayBuffer = reader.result as ArrayBuffer;
      imageBuffer = new Uint8Array(arrayBuffer);

      // Do this after the image buffer is set so generate doesn't run before the image is loaded
      inputSrc = URL.createObjectURL(fileload);
    };
    reader.readAsArrayBuffer(fileload);
  }

  // Needs to happen after <img> has loaded the source so we can get the natural image dimensions
  function onSourceImageLoaded(e: Event) {
    const source = e.target as HTMLImageElement;

    sourceWidth = source.naturalWidth;
    sourceHeight = source.naturalHeight;

    maxSizeMax = Math.max(sourceWidth, sourceHeight);

    onActivity();
  }

  function onActivity() {
    if (!queued) {
      queued = true;
      requestIdleCallback(() => {
        generate();
        queued = false;
      });
    }
  }

  function generate() {
    if (generating) {
      console.debug("Already generating!");
      return;
    }

    if (!inputSrc) {
      error = "No image selected!";
      return;
    }

    if (imageBuffer === null) {
      error = "Image not loaded!";
      return;
    }

    generating = true;
    error = "";
    try {
      const startTime = performance.now();
      const result = getBlueprintFromImage(
        imageBuffer,
        maxSize,
        alpha,
        floorTile,
        wallTile
      );
      blueprint = result.base64;
      outputTileCount = result.image.tileCount;
      outputEntityCount = result.image.entityCount;
      console.debug(
        `Generated blueprint in ${performance.now() - startTime}ms`
      );

      const canvas = document.getElementById(
        "outputCanvas"
      ) as HTMLCanvasElement;
      const ctx = canvas.getContext("2d");
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      outputWidth = result.image.width;
      outputHeight = result.image.height;

      // Canvas size must be set to the output image size
      canvas.width = result.image.width;
      canvas.height = result.image.height;

      // Set the actual canvas display width
      canvas.style.width = `${sourceWidth}px`;
      canvas.style.height = `${sourceHeight}px`;
      canvas.style.imageRendering = "pixelated";

      console.debug(`Set up Canvas in ${performance.now() - startTime}ms`);

      // Create a Uint8Array from the image data
      const imgData = new Uint8ClampedArray(result.image.data);

      // Create a new ImageData object
      const imageData = new ImageData(
        imgData,
        result.image.width,
        result.image.height
      );

      // Draw the image data onto the canvas
      ctx.putImageData(imageData, 0, 0);

      console.debug(
        `Wrote image data to canvas in ${performance.now() - startTime}ms`
      );
    } catch (e) {
      console.error(e);
      error = `ERROR: ${(e as Error).message}`;
    } finally {
      generating = false;
    }
  }

  async function copyBlueprintToClipboard() {
    await navigator.clipboard.writeText(blueprint);
  }

  function onPaste(event: ClipboardEvent) {
    const items = event.clipboardData.items;

    for (let i = 0; i < items.length; i++) {
      if (items[i].type.indexOf("image") !== -1) {
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

  // 	let  avatar, fileinput;
  // let fileinput = document.getElementById("fileinput") as HTMLInputElement;

  // const onFileSelected = (event: Event) => {
  //   const target = event.target as HTMLInputElement;

  //   if (target.files && target.files.length) {
  //     // const file = target.files[0];
  //     let image = target.files[0];
  //     let reader = new FileReader();
  //     reader.readAsDataURL(image);
  //     reader.onload = (e) => {
  //       console.log(e);
  //       //  avatar = e.target.result
  //     };
  //   }
  // };
</script>

<div class="mb-4 flex flex-col items-center justify-between md:flex-row">
  <div>
    <h1 class="text-5xl">Spaceship Generator</h1>
  </div>
  <div>
    <a href="https://factorio.com/"
      ><img
        src="/images/factorio-logo-sm.png"
        alt="Factorio logo"
        class="w-60"
      /></a
    >
    <a href="https://mods.factorio.com/mod/space-exploration">
      <img
        src="/images/space-exploration-logo-sm.png"
        alt="Space Exploration logo"
        class="w-60"
      />
    </a>
  </div>
</div>

<main class="flex flex-col gap-4 md:flex-row">
  <div class="flex w-full flex-col gap-4">
    <UIPanel title="Input">
      <div slot="title-actions">
        <button
          class="btn btn-icon btn-danger"
          on:click={() => reset()}
          disabled={!inputSrc}
        >
          <img src={inputSrc ? resetIconBlack : resetIconWhite} alt="Reset" />
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
            bind:value={alpha}
            on:change={onActivity}
          />
        </SettingRow>

        <SettingRow
          title="Floor Tile"
          hint="The game identifier of the floor tile you want to use, if you want something different than the Space Exploration default."
          let:inputId
        >
          <SettingRowTextInput
            id={inputId}
            bind:value={floorTile}
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
            bind:value={wallTile}
            on:change={onActivity}
          />
        </SettingRow>
      </InputGroup>
      <InputGroup title="Source">
        {#if inputSrc}
          <div class="flex flex-col items-center justify-center">
            <img
              id="inputImg"
              src={inputSrc}
              alt="Input source"
              class="max-w-full"
              on:load={onSourceImageLoaded}
            />
            <div>{sourceWidth}&times;{sourceHeight}</div>
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
            max={maxSizeMax}
            step="10"
            disabled={!inputSrc}
            bind:value={maxSize}
            on:input={onActivity}
          />
        </SettingRow>
      </InputGroup>
      {#if inputSrc}
        <InputGroup title="Blueprint Preview">
          <div class="w-full text-center">
            <canvas id="outputCanvas" class="mx-auto h-0 w-0 max-w-full" />
            <div>{outputWidth}&times;{outputHeight}</div>
          </div>
        </InputGroup>

        <InputGroup title="Components">
          <div class="flex w-full flex-row justify-between gap-2 p-2">
            <div class="inset-object flex w-full flex-auto justify-between p-2">
              <div>{floorTile}</div>
              <div style="text-shadow: 1px 1px 3px black; font-weight: bold">
                {outputTileCount}
              </div>
            </div>
            <div class="inset-object flex w-full flex-auto justify-between p-2">
              <div>{wallTile}</div>
              <div style="text-shadow: 1px 1px 3px black; font-weight: bold">
                {outputEntityCount}
              </div>
            </div>
          </div>
        </InputGroup>
      {/if}
      <div
        slot="bottom"
        class="flex justify-end pt-2"
        class:hidden={!blueprint}
      >
        <button
          class="btn-copy-blueprint"
          disabled={!blueprint}
          on:click={() => copyBlueprintToClipboard()}
          ><img src="/images/blueprint-icon.png" alt="icon" /> Copy Blueprint</button
        >
      </div>
    </UIPanel>
  </div>
</main>

<style>
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
