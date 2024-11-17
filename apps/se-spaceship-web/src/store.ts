import { getBlueprintFromImage } from "image-to-blueprint";
import { derived, get, writable } from "svelte/store";

const moduleTiles = {
  "space-exploration": {
    floor: "se-spaceship-floor",
    wall: "se-spaceship-wall",
  },
  "space-age": {
    floor: "space-platform-foundation",
    wall: "stone-wall",
  },
} as const;

type Store = {
  module: "space-age" | "space-exploration" | "custom";
  customWallTile: string;
  customFloorTile: string;

  blueprint: string;
  generating: boolean;
  imageBuffer: Uint8Array | null;
  imageData: ImageData | null;
  inputSrc: string | null;
  outputEntityCount: number;
  outputHeight: number;
  outputTileCount: number;
  outputWidth: number;
  sourceHeight: number;
  sourceWidth: number;
};

const initialStore: Store = {
  module: "space-age",
  customWallTile: "stone-wall",
  customFloorTile: "space-platform-foundation",
  blueprint: "",
  generating: false,
  imageBuffer: null,
  imageData: null,
  inputSrc: null,
  outputEntityCount: 0,
  outputHeight: 0,
  outputTileCount: 0,
  outputWidth: 0,
  sourceHeight: 0,
  sourceWidth: 0,
};

export const store = writable<Store>(initialStore);

function update(values: Partial<Store>) {
  store.update((updater) => ({ ...updater, ...values }));
}

export function resetStore() {
  store.set(initialStore);
}

export const tiles = derived(store, ($store) =>
  $store.module === "custom"
    ? { floor: $store.customFloorTile, wall: $store.customWallTile }
    : moduleTiles[$store.module]
);

export const maxSizeMax = derived(store, ($store) =>
  $store.module === "space-age"
    ? 200
    : Math.max($store.sourceWidth, $store.sourceHeight)
);

export function loadFile(fileload: File) {
  // check if the type is png, jpeg, webp, or gif
  if (
    fileload.type !== "image/gif" &&
    fileload.type !== "image/jpeg" &&
    fileload.type !== "image/png" &&
    fileload.type !== "image/webp"
  ) {
    throw new Error("Invalid file type!");
    return;
  }

  const reader = new FileReader();

  reader.onload = () => {
    const arrayBuffer = reader.result as ArrayBuffer;
    const imageBuffer = new Uint8Array(arrayBuffer);

    update({
      imageBuffer,
      // Do this after the image buffer is set so generate doesn't run before the image is loaded
      inputSrc: URL.createObjectURL(fileload),
    });
  };
  reader.readAsArrayBuffer(fileload);
}

export function generate(
  maxSize: number,
  alpha: number,
  floorTile: string,
  wallTile: string
) {
  const { generating, imageBuffer, inputSrc } = get(store);

  if (generating) {
    console.debug("Already generating!");
    return;
  }

  if (!inputSrc) {
    throw new Error("No image selected!");
    return;
  }

  if (imageBuffer === null) {
    throw new Error("Image not loaded!");
    return;
  }

  update({ generating: true });

  try {
    const result = getBlueprintFromImage(
      imageBuffer,
      maxSize,
      alpha,
      floorTile,
      wallTile
    );
    update({
      blueprint: result.base64,
      outputTileCount: result.image.tileCount,
      outputEntityCount: result.image.entityCount,
      outputWidth: result.image.width,
      outputHeight: result.image.height,
    });

    // Create a Uint8Array from the image data
    const arrayData = new Uint8ClampedArray(result.image.data);

    // Create a new ImageData object
    const imageData = new ImageData(
      arrayData,
      result.image.width,
      result.image.height
    );

    update({ imageData });
  } catch (e) {
    console.error(e);

    throw new Error(`ERROR: ${(e as Error).message}`);
  } finally {
    update({ generating: false });
  }
}
