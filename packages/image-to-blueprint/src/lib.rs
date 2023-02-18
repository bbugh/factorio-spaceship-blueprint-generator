mod utils;

// use image::prelude::*;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, image-to-blueprint!");
}

// receives a string from JavaScript and displays it in an alert box, interpolating it with "HElooe world"
#[wasm_bindgen(js_name = helloWorld)]
pub fn hello_world(input_string: &str) {
    let mut s = String::from("Hello, ");
    s.push_str(input_string);
    s.push_str("!");
    alert(&s);
    // s
}

// receives a string from JavaScript and returns the same string all uppercased
#[wasm_bindgen(js_name = upperCase)]
pub fn upper_case(s: &str) -> String {
    s.to_uppercase()
}

// receives a UInt8Array from JavaScript, converts it into an ImageBuffer, and returns a string with the dimensions of the image
#[wasm_bindgen(js_name = getImageDimensions)]
pub fn get_image_dimensions(image_data: &[u8]) -> String {
    let image = image::load_from_memory(image_data).unwrap();
    let (width, height) = image.dimensions();
    format!("{}x{}", width, height)
}
