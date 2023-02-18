import './app.css'
import App from './App.svelte'

import init from 'image-to-blueprint'

const load = async () => {
  const startTime = performance.now()
  await init()
  const endTime = performance.now()
  console.debug(`Call to wasm init took ${endTime - startTime} milliseconds`)

  const app = new App({
    target: document.getElementById('app')
  })
}

load()


// const app = new App({
//   target: document.getElementById('app'),
// })

// export default app
