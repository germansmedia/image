// (C) Copyright 2020, by Germans Media Technology & Services
// Image

use math::*;

mod pixel;
pub use pixel::*;

mod image;
pub use image::*;

pub mod bmp;
pub mod png;
pub mod jpeg;
pub mod tga;
pub mod gif;
pub mod pbm;
pub mod tiff;
pub mod xbm;
pub mod webp;

#[allow(dead_code)]
pub fn test(src: &[u8]) -> Option<usizexy> {
    if let Some(size) = bmp::test(src) {
        Some(size)
    }
    else if let Some(size) = png::test(src) {
        Some(size)
    }
    else if let Some(size) = jpeg::test(src) {
        Some(size)
    }
    else if let Some(size) = gif::test(src) {
        Some(size)
    }
    else if let Some(size) = tga::test(src) {
        Some(size)
    }
    else if let Some(size) = tiff::test(src) {
        Some(size)
    }
    else if let Some(size) = pbm::test(src) {
        Some(size)
    }
    else if let Some(size) = xbm::test(src) {
        Some(size)
    }
    else if let Some(size) = webp::test(src) {
        Some(size)
    }
    else {
        None
    }
}

#[allow(dead_code)]
pub fn decode<T: Pixel>(src: &[u8]) -> Result<Image<T>,String> {
    if let Ok(image) = bmp::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = png::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = jpeg::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = gif::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = tga::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = tiff::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = pbm::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = xbm::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = webp::decode(src) {
        Ok(image)
    }
    else {
        Err("image format not supported".to_string())
    }
}
