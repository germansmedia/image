// (C) Copyright 2020, by Germans Media Technology & Services
// Image
// WebP Image Format

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
