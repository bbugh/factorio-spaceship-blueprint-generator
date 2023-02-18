import './app.css'
import App from './App.svelte'

import init from 'image-to-blueprint'
import { getImageDimensions } from 'image-to-blueprint'

const load = async () => {
  const startTime = performance.now()
  await init()
  const endTime = performance.now()
  console.debug(`Call to wasm init took ${endTime - startTime} milliseconds`)

  const app = new App({
    target: document.getElementById('app')
  })

  document.addEventListener('paste', async (event) => {
    console.log("pasted");
    const items = event.clipboardData.items;
    for (let i = 0; i < items.length; i++) {
      if (items[i].type.indexOf("image") !== -1) {
        const blob = items[i].getAsFile();
        const arrayBuffer = await blob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        console.log(getImageDimensions(uint8Array));
        // console.log(uint8Array);
      }
    }
  });
}


load()


// const app = new App({
//   target: document.getElementById('app'),
// })

// export default app
