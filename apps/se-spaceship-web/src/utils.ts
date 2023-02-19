// // receives an image from the clipboard, converts it into a uint8array, and then
// // calls the wasm function getImageDimensions to get the dimensions of the
// // image. It then calls the wasm function convertImageToBlueprint to convert the
// // image into a blueprint. The blueprint is then displayed in the browser.
// function getImageDimensions(imageData: Uint8Array): { width: number; height: number } {
//   const width = wasm.getImageWidth(imageData)
//   const height = wasm.getImageHeight(imageData)
//   return { width, height }
// }

// import { getImageDimensions } from 'image-to-blueprint'


/// Reads an image from the clipboard and converts it into a uint8array.
// export async function readImageFromClipboard(): Promise<Uint8Array> {
//   const clipboardItems = await navigator.clipboard.read()
//   const clipboardItem = clipboardItems[0]
//   const blob = await clipboardItem.getType('image/png')
//   const arrayBuffer = await blob.arrayBuffer()
//   const uint8Array = new Uint8Array(arrayBuffer)
//   return uint8Array
// }

// export async function readImageFromClipboard2(): Promise<Uint8Array | null> {
//   const clipboardItems = await navigator.clipboard.read()
//   for (const clipboardItem of clipboardItems) {
//     const types = await clipboardItem.types
//     if (types.includes('image/png') || types.includes('image/jpeg') || types.includes('image/gif') || types.includes('image/webp') || types.includes('image/bmp') || types.includes('image/tiff')) {
//       const blob = await clipboardItem.getType(types.find(type => type.startsWith('image/')))
//       const arrayBuffer = await blob.arrayBuffer()
//       const uint8Array = new Uint8Array(arrayBuffer)
//       return uint8Array
//     }
//   }
//   return null
// }

export async function readImageFromClipboard(): Promise<Uint8Array> {
  const clipboardItems = await navigator.clipboard.read();
  for (const clipboardItem of clipboardItems) {
    const type = await clipboardItem.types.find(type => type.startsWith('image/'))
    if (type) {
      const blob = await clipboardItem.getType(type);
      const arrayBuffer = await blob.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);
      return uint8Array;
    }
  }
  throw new Error('No image found in clipboard');
}

// /// Converts an image into a base64 blueprint string.
// export async function convertImageToBlueprint(): Promise<string> {
//   return readImageFromClipboard().then((imageData) => {
//     const dimensions = getImageDimensions(imageData)
//     console.log(dimensions)
//     return dimensions
//     // const blueprint = wasm.convertImageToBlueprint(imageData, width, height)
//     // return blueprint
//   })
// }
