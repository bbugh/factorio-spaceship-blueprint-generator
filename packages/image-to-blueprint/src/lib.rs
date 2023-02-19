mod utils;

// use image::prelude::*;
use std::num::NonZeroUsize;
use wasm_bindgen::prelude::*;

use factorio_blueprint::{
    objects::{Blueprint, Color, Entity, Position, Tile},
    BlueprintCodec, Container,
};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use noisy_float::types::{R32, R64};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, image-to-blueprint!");
// }

// // receives a string from JavaScript and displays it in an alert box, interpolating it with "HElooe world"
// #[wasm_bindgen(js_name = helloWorld)]
// pub fn hello_world(input_string: &str) {
//     let mut s = String::from("Hello, ");
//     s.push_str(input_string);
//     s.push_str("!");
//     alert(&s);
//     // s
// }

// // receives a string from JavaScript and returns the same string all uppercased
// #[wasm_bindgen(js_name = upperCase)]
// pub fn upper_case(s: &str) -> String {
//     s.to_uppercase()
// }

// #[derive(Debug, Clone)]
// enum ErrorType {
//     InvalidImageFormatError,
// }

// use core::fmt;
// impl std::error::Error for ErrorType {}
// impl fmt::Display for ErrorType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "display implementation becomes the error message")
//     }
// }

// // receives a UInt8Array from JavaScript, converts it into an ImageBuffer, and returns a string with the dimensions of the image
// // #[wasm_bindgen(js_name = getBlueprintFromImage)]
// // pub fn get_blueprint_from_image(image_data: &[u8]) -> Result<String, JsError> {
// //     let image = match image::load_from_memory(image_data) {
// //         Ok(img) => img,
// //         Err(e) => return Err(e.into()),
// //     };

// //     return match create_blueprint_from_image(image) {
// //         Ok(s) => Ok(s),
// //         Err(e) => Err(JsError::new("could not do something")),
// //     };
// // }

// // fn convert_error(error: Err) -> JsValue {
// //     JsValue::from_str(&format!("{}", error))
// // }

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BlueprintImage {
    pub width: u32,
    pub height: u32,
    #[tsify(type = "Uint8Array")]
    pub data: Vec<u8>,
}

// #[derive(Serialize, Deserialize)]
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BlueprintResult {
    pub base64: String,
    pub image: BlueprintImage,
}

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// HACK: This is a workaround for wasm_bindgen not supporting custom return types
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export function getBlueprintFromImage(image_data: Uint8Array, max_dimension: number, max_alpha: number, floor_tile_name: string, wall_tile_name: string): BlueprintResult;
"#;

#[wasm_bindgen(js_name = getBlueprintFromImage, skip_typescript)]
pub fn get_blueprint_from_image(
    image_data: &[u8],
    max_dimension: u32,
    max_alpha: u8,
    floor_tile_name: &str,
    wall_tile_name: &str,
) -> Result<JsValue, JsValue> {
    let image = match image::load_from_memory(image_data) {
        Ok(img) => img,
        Err(e) => return Err(JsError::new(&format!("{}", e)).into()),
    };

    let max_dimension = max_dimension.clamp(2, 500);

    let image = image.resize(
        max_dimension,
        max_dimension,
        image::imageops::FilterType::Nearest,
    );

    // let (width, height) = image.dimensions();
    // log(&format!("width: {}, height: {}", width, height));

    let blueprint =
        match create_blueprint_from_image(image, max_alpha, floor_tile_name, wall_tile_name) {
            Ok(blueprint) => blueprint,
            Err(e) => return Err(JsError::new(&format!("{}", e)).into()),
        };

    let blueprint_image = blueprint_image_from_blueprint(&blueprint);
    let (width, height) = blueprint_image.dimensions();

    let image_result = BlueprintImage {
        width,
        height,
        data: blueprint_image.into_bytes(),
    };

    let blueprint_result = BlueprintResult {
        base64: blueprint_string_from_blueprint(&blueprint).unwrap(),
        image: image_result,
    };

    Ok(serde_wasm_bindgen::to_value(&blueprint_result)?)
    // let BlueprintResult {
    //     base64,
    //     image: _,
    // } = match create_blueprint_from_image(image) {
    //     Ok(result) => result,
    //     Err(e) => return Err(JsError::new(&format!("{}", e)).into()),
    // };

    // let blueprint = create_blueprint_from_image(image).unwrap();
    // let container = Container::from(blueprint);
    // BlueprintCodec::encode_string(&container).expect("failed to encode blueprint");

    // Ok("test".to_string())

    // let blueprint = match create_blueprint_from_image(image) {
    //     Ok(blueprint) => {

    //     }
    //     Err(e) => Err(JsError::new(&format!("{}", e)).into()),
    // };
}

fn blueprint_string_from_blueprint(
    blueprint: &Blueprint,
) -> Result<String, Box<dyn std::error::Error>> {
    let container = Container::from(blueprint.clone());
    let blueprint_string =
        BlueprintCodec::encode_string(&container).expect("failed to encode blueprint");
    Ok(blueprint_string)
}

fn create_blueprint_from_image(
    img: DynamicImage,
    maximum_alpha: u8,
    floor_tile_name: &str,
    wall_tile_name: &str,
) -> Result<Blueprint, Box<dyn std::error::Error>> {
    let (width, height) = img.dimensions();

    let mut tiles: Vec<Tile> = Vec::new();
    let mut entities: Vec<Entity> = Vec::new();
    let mut entity_index = 1;
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if pixel[3] <= maximum_alpha {
                continue;
            }

            tiles.push(Tile {
                position: Position {
                    x: R64::new(x as f64),
                    y: R64::new(y as f64),
                },
                name: floor_tile_name.to_string(),
            });

            let mut edge = false;
            for j in -1..=1 {
                for i in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let nx = x as i32 + i;
                    let ny = y as i32 + j;
                    if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                        let npixel = img.get_pixel(nx as u32, ny as u32);
                        if npixel[3] < maximum_alpha {
                            edge = true;
                            break;
                        }
                    }
                }
            }
            if edge {
                entities.push(new_entity(
                    wall_tile_name.to_string(),
                    std::num::NonZeroUsize::new(entity_index).unwrap(),
                    Position {
                        x: R64::new(x as f64),
                        y: R64::new(y as f64),
                    },
                ));
                entity_index += 1;
            }
        }
    }

    let blueprint = Blueprint {
        label: "test".to_string(),
        label_color: Some(Color {
            r: R32::new(0.0),
            g: R32::new(0.0),
            b: R32::new(0.0),
            a: R32::new(0.0),
        }),
        entities,
        tiles,
        icons: vec![],
        schedules: vec![],
        item: "blueprint".to_string(),
        version: 281479276658688,
    };

    Ok(blueprint)
}

fn new_entity(name: String, entity_number: NonZeroUsize, position: Position) -> Entity {
    Entity {
        position,
        name,
        entity_number,

        direction: None,
        orientation: None,
        connections: None,
        control_behavior: None,
        items: None,
        recipe: None,
        bar: None,
        inventory: None,
        infinity_settings: None,
        type_: None,
        input_priority: None,
        output_priority: None,
        filter: None,
        filters: None,
        filter_mode: None,
        override_stack_size: None,
        drop_position: None,
        pickup_position: None,
        request_filters: None,
        request_from_buffers: None,
        parameters: None,
        alert_parameters: None,
        auto_launch: None,
        variation: None,
        color: None,
        station: None,
    }
}

enum BlueprintPosition {
    Tile,
    Entity,
}

fn blueprint_image_from_blueprint(bp: &Blueprint) -> DynamicImage {
    let (min_x, min_y, max_x, max_y) = get_bounds(&bp);

    let width = (max_x - min_x).to_u32().unwrap() + 1;
    let height = (max_y - min_y).to_u32().unwrap() + 1;
    let mut image = ImageBuffer::new(width, height);

    let mut positions = std::collections::HashMap::new();

    // let mut tile_positions = std::collections::HashSet::new();
    // let mut entity_positions = std::collections::HashMap::new();

    for tile in bp.tiles.iter() {
        let x = tile.position.x;
        let y = tile.position.y;
        positions.insert((x, y), BlueprintPosition::Tile);
        // tile_positions.insert((x, y));
    }

    for entity in bp.entities.iter() {
        let x = entity.position.x;
        let y = entity.position.y;
        positions.insert((x, y), BlueprintPosition::Entity);
        // entity_positions.insert((x, y), &entity.name);
    }

    // let progress_bar = indicatif::ProgressBar::new((width * height) as u64);
    // progress_bar.set_message("Creating image");

    for y in 0..height {
        for x in 0..width {
            // let image_x = x * TILE_SIZE;
            // let image_y = y * TILE_SIZE;

            let position = positions.get(&(R64::new(x as f64), R64::new(y as f64)));
            if let Some(position) = position {
                match position {
                    BlueprintPosition::Tile => {
                        // image.put_pixel(x, y, Rgba([0, 0, 0, 255]));
                        image.put_pixel(x, y, Rgba([58, 53, 46, 255]));
                        // image.copy_from(&tile_image, image_x, image_y).unwrap();
                    }
                    BlueprintPosition::Entity => {
                        // image.put_pixel(x, y, Rgba([255, 0, 0, 255]));
                        image.put_pixel(x, y, Rgba([205, 203, 207, 255]));
                        // image.copy_from(&entity_image, image_x, image_y).unwrap();
                    }
                }
            }

            // if tile_positions.contains(&(R64::new(x as f64), R64::new(y as f64))) {
            //     image.copy_from(&tile_image, image_x, image_y).unwrap();
            // }

            // if entity_positions.contains_key(&(R64::new(x as f64), R64::new(y as f64))) {
            //     image.copy_from(&entity_image, image_x, image_y).unwrap();
            // }

            // progress_bar.inc(1);
        }
    }

    // progress_bar.finish();

    DynamicImage::ImageRgba8(image)

    // png
}

fn get_bounds(bp: &Blueprint) -> (R64, R64, R64, R64) {
    let mut min_x = R64::new(0.0);
    let mut min_y = R64::new(0.0);
    let mut max_x = R64::new(0.0);
    let mut max_y = R64::new(0.0);

    for tile in &bp.tiles {
        if tile.position.x < min_x {
            min_x = tile.position.x;
        }
        if tile.position.y < min_y {
            min_y = tile.position.y;
        }
        if tile.position.x > max_x {
            max_x = tile.position.x;
        }
        if tile.position.y > max_y {
            max_y = tile.position.y;
        }
    }

    for entity in &bp.entities {
        if entity.position.x < min_x {
            min_x = entity.position.x;
        }
        if entity.position.y < min_y {
            min_y = entity.position.y;
        }
        if entity.position.x > max_x {
            max_x = entity.position.x;
        }
        if entity.position.y > max_y {
            max_y = entity.position.y;
        }
    }

    (min_x, min_y, max_x, max_y)
}
