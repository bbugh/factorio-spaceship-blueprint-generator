<script lang="ts">
  import { getBlueprintFromImage } from 'image-to-blueprint';
  // import { readImageFromClipboard } from './utils';

  let blueprint = '';
  let error = '';
  let alpha=1;
  let floorTile = "se-spaceship-floor";
  let wallTile = "se-spaceship-wall";

  let maxSize = 200;

  let loading = 'none';

  function copyBlueprintToClipboard() {
    navigator.clipboard.writeText(blueprint);
  }

  document.addEventListener('paste', async (event) => {
    loading = 'block';
    error = '';
    const items = event.clipboardData.items;
    for (let i = 0; i < items.length; i++) {
      if (items[i].type.indexOf("image") !== -1) {
        const blob = items[i].getAsFile();
        const arrayBuffer = await blob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        (document.getElementById('inputImg') as HTMLImageElement).src = URL.createObjectURL(blob);
        try {
          const result = getBlueprintFromImage(uint8Array, maxSize, alpha, floorTile, wallTile);
          blueprint = result.base64
          console.log(blueprint);

          const canvas = document.getElementById('outputCanvas') as HTMLCanvasElement;
          const ctx = canvas.getContext('2d');
          ctx.clearRect(0, 0, canvas.width, canvas.height);

          // const canvas = document.createElement('canvas');
          // const ctx = canvas.getContext('2d');
          canvas.width = result.image.width;
          canvas.height = result.image.height;

          // Create a Uint8Array from the image data
          const imgData = new Uint8ClampedArray(result.image.data);

          // Create a new ImageData object
          const imageData = new ImageData(imgData, result.image.width, result.image.height);

          // Draw the image data onto the canvas
          ctx.putImageData(imageData, 0, 0);
        } catch (e) {
          error = `ERROR: ${e.message}`;
        } finally {
          loading = 'none';
        }
      }
    }
    loading = 'none';
  });

// 	let  avatar, fileinput;
let fileinput;

	const onFileSelected =(e)=>{
  let image = e.target.files[0];
            let reader = new FileReader();
            reader.readAsDataURL(image);
            reader.onload = e => {
              console.log(e);
                //  avatar = e.target.result
            };
}
</script>

<main>
  <h1 id="loading" style="color: red; display: {loading};">Loading</h1>

  <div>
    <h1>Options</h1>

    <div style="display: flex; flex-direction: row;">
      <label for="alpha">Alpha</label>
      <input type="range" min="1" max="254" class="slider" bind:value={alpha}>
      <div>{alpha}</div>
    </div>

    <div style="display: flex; flex-direction: row;">
      <label for="maxSize">MaxSize</label>
      <input type="range" min="10" max="500" class="slider" bind:value={maxSize}>
      <div>{maxSize}</div>
    </div>

    <div style="display: flex; flex-direction: row;">
      <label for="floorTile">Floor Tile</label>
      <input type="text" bind:value={floorTile}>
    </div>

    <div style="display: flex; flex-direction: row;">
      <label for="wallTile">Wall Tile</label>
      <input type="text" bind:value={wallTile}>
    </div>
    <!-- <input type="range" min="1" max="254" class="slider" bind:value={alpha}>
    <div>{alpha}</div> -->

    <!-- <input type="text" bind:value={floorTile}>
    <input type="text" bind:value={wallTile}> -->
  </div>

  <div class="chan" on:click={()=>{fileinput.click();}} on:keydown={()=>{fileinput.click();}} >Choose Image</div>
  <div class="error">{error}</div>
  <input style="display:none" type="file" accept=".jpg, .jpeg, .png" on:change={(e)=>onFileSelected(e)} bind:this={fileinput} >

  <div style="display: flex; flex-direction: row;">
    <div style="display: flex; flex-direction: column;">
      <h1>Input</h1>
      <img id="inputImg" src="" alt="what" style="max-width: {maxSize}px; max-height: {maxSize}px">
    </div>
    <div style="display: flex; flex-direction: column;">
      <h1>Output</h1>
      <canvas id="outputCanvas"></canvas>
    </div>
  </div>

  <input disabled={!blueprint} type="text" readonly bind:value={blueprint} />

  <button disabled={!blueprint} on:click={() => copyBlueprintToClipboard()}>Copy Blueprint String</button>

  <!-- <button on:click={() => readImageFromClipboard()}>Read Image</button>
  <br />
  <textarea class="blueprint-text" bind:value={blueprint} /> -->


  <!-- {#if avatar}
  <img class="avatar" src="{avatar}" alt="d" />
  {:else}
  <img class="avatar" src="https://cdn4.iconfinder.com/data/icons/small-n-flat/24/user-alt-512.png" alt="" />
  {/if}
  <img class="upload" src="https://static.thenounproject.com/png/625182-200.png" alt="" on:click={()=>{fileinput.click();}} />
  <div class="chan" on:click={()=>{fileinput.click();}}>Choose Image</div>
  <input style="display:none" type="file" accept=".jpg, .jpeg, .png" on:change={(e)=>onFileSelected(e)} bind:this={fileinput} > -->

</main>

<style>
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
