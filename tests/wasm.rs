extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use image::Luma;
use imageproc::{assert_dimensions_match, assert_pixels_eq, gray_image};
use imageproc::region_labelling::{connected_components, Connectivity};

#[wasm_bindgen_test]
fn connected_components_connectivity_four() {
    let background_color = Luma([0u8]);

    let image = gray_image!(
        1, 0, 1, 1;
        0, 1, 1, 0;
        0, 0, 0, 0;
        0, 0, 0, 1);

    let components_four = gray_image!(type: u32,
        1, 0, 2, 2;
        0, 2, 2, 0;
        0, 0, 0, 0;
        0, 0, 0, 3);

    assert_pixels_eq!(
        connected_components(&image, Connectivity::Four, background_color),
        components_four
    );
}

// Following does not fail in the right way as you can't construct
// a image::GrayImage where width * height > usize

// #[wasm_bindgen_test]
// fn panics_when_image_too_large() {
//     use std::panic;

//     let background_color = Luma([0u8]);

//     let image = image::GrayImage::new(2, usize::max_value() as u32);

//     let result = panic::catch_unwind(|| {
//         connected_components(&image, Connectivity::Four, background_color);
//     });
//     assert!(result.is_err());
// }
