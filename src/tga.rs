// image_formats::tga
// by Desmond Germans, 2019

use crate::*;

pub fn test(_src: &[u8]) -> Option<usizexy> {
    None
}

pub fn decode<T: Pixel>(_src: &[u8]) -> Result<Image<T>,String> {
    Err("not implemented yet".to_string())
}

pub fn encode<T: Pixel>(_image: &Image<T>) -> Result<Vec<u8>,String> {
    Err("not implemented yet".to_string())
}
