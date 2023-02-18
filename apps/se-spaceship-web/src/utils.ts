// // receives an image from the clipboard, converts it into a uint8array, and then
// // calls the wasm function getImageDimensions to get the dimensions of the
// // image. It then calls the wasm function convertImageToBlueprint to convert the
// // image into a blueprint. The blueprint is then displayed in the browser.
// function getImageDimensions(imageData: Uint8Array): { width: number; height: number } {
//   const width = wasm.getImageWidth(imageData)
//   const height = wasm.getImageHeight(imageData)
//   return { width, height }
// }

import { getImageDimensions } from 'image-to-blueprint'


/// Reads an image from the clipboard and converts it into a uint8array.
export async function readImageFromClipboard(): Promise<Uint8Array> {
  const clipboardItems = await navigator.clipboard.read()
  const clipboardItem = clipboardItems[0]
  const blob = await clipboardItem.getType('image/png')
  const arrayBuffer = await blob.arrayBuffer()
  const uint8Array = new Uint8Array(arrayBuffer)
  return uint8Array
}

/// Converts an image into a base64 blueprint string.
export async function convertImageToBlueprint(): Promise<string> {
  return readImageFromClipboard().then((imageData) => {
    const dimensions = getImageDimensions(imageData)
    console.log(dimensions)
    return dimensions
    // const blueprint = wasm.convertImageToBlueprint(imageData, width, height)
    // return blueprint
  })
}
