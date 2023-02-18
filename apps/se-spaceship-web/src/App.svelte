<script lang="ts">
  import { helloWorld, upperCase } from 'image-to-blueprint';
  import { readImageFromClipboard } from './utils';
// 	let  avatar, fileinput;

// 	const onFileSelected =(e)=>{
//   let image = e.target.files[0];
//             let reader = new FileReader();
//             reader.readAsDataURL(image);
//             reader.onload = e => {
//                  avatar = e.target.result
//             };
// }
</script>

<main>
  <button on:click={() => helloWorld(upperCase('banana'))}>Greet</button>
  <button on:click={() => readImageFromClipboard()}>Read Image</button>
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
