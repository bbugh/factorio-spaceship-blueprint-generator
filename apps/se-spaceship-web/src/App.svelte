<script lang="ts">
  import { getBlueprintFromImage } from 'image-to-blueprint';
  import { readImageFromClipboard } from './utils';

  let blueprint = '';
  let error = '';

  document.addEventListener('paste', async (event) => {
    error = '';
    const items = event.clipboardData.items;
    for (let i = 0; i < items.length; i++) {
      if (items[i].type.indexOf("image") !== -1) {
        const blob = items[i].getAsFile();
        const arrayBuffer = await blob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        try {
          const result = getBlueprintFromImage(uint8Array);
          blueprint = result.base64
          console.log(blueprint);

          const canvas = document.createElement('canvas');
          const ctx = canvas.getContext('2d');
          canvas.width = result.image.width;
          canvas.height = result.image.height;

          // Create a Uint8Array from the image data
          const imgData = new Uint8ClampedArray(result.image.data);

          // Create a new ImageData object
          const imageData = new ImageData(imgData, result.image.width, result.image.height);

          // Draw the image data onto the canvas
          ctx.putImageData(imageData, 0, 0);

          // Add the canvas to the page
          document.body.appendChild(canvas);


        } catch (e) {
          error = `ERROR: ${e.message}`;
        }
      }
    }
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

  <div class="chan" on:click={()=>{fileinput.click();}} on:keydown={()=>{fileinput.click();}} >Choose Image</div>
  <div class="error">{error}</div>
  <input style="display:none" type="file" accept=".jpg, .jpeg, .png" on:change={(e)=>onFileSelected(e)} bind:this={fileinput} >

  <button on:click={() => readImageFromClipboard()}>Read Image</button>
  <br />
  <textarea class="blueprint-text" bind:value={blueprint} />


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
  .blueprint-text {
    border: 1px solid darkgray;
    font-family: monospace;
    border-radius: 0.15em;
    width: 750px;
    height: 250px;
  }
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
