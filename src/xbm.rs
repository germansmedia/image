// image
// by Desmond Germans

use crate::*;

pub fn test(_src: &[u8]) -> Option<(u32,u32)> {
	None
}

pub fn decode<T: Pixel>(_src: &[u8]) -> Result<Image<T>,String> {
	Err("not implemented yet".to_string())
}

pub fn encode<T: Pixel>(_image: &Image<T>) -> Result<Vec<u8>,String> {
	Err("not implemented yet".to_string())
}
