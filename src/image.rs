// image
// by Desmond Germans

use math::*;
use crate::*;

pub struct Image<T> {
    pub size: usizexy,
    pub data: Box<[T]>,
}

impl<T: Pixel + Clone + Zero> Image<T> {
    pub fn new(size: usizexy) -> Image<T> {
        Image {
            size: size,
            data: vec![T::zero(); size.x * size.y].into_boxed_slice(),
        }
    }

    pub fn pixel(&self,p: usizexy) -> &T {
        &self.data[p.y * self.size.x + p.x]
    }

    pub fn pixel_mut(&mut self,p: usizexy) -> &mut T {
        &mut self.data[p.y * self.size.x + p.x]
    }
}
