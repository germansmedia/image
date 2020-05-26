// image
// by Desmond Germans

use math::*;

trait Clamp1 {
    fn clamp1(self) -> Self;
}

macro_rules! impl_clamp1 (
    ($t:ty) => (
        impl Clamp1 for $t {
            fn clamp1(self) -> $t {
                if self < 0.0 { return 0.0; }
                if self > 1.0 { return 1.0; }
                self
            }            
        }
    )
);

impl_clamp1!(f32);
impl_clamp1!(f64);

pub trait Pixel: Zero + Clone + Copy {
    fn new_rgb(r: u8,g: u8,b: u8) -> Self;
    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> Self;
    fn set_r(&mut self,r: u8);
    fn set_g(&mut self,g: u8);
    fn set_b(&mut self,b: u8);
    fn set_a(&mut self,a: u8);
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;
}

macro_rules! impl_pixel (
    ($t:ty) => (
        impl Pixel for RGB<$t> {
            fn new_rgb(r: u8,g: u8,b: u8) -> RGB<$t> {
                let r = (r as $t) / 255.0;
                let g = (g as $t) / 255.0;
                let b = (b as $t) / 255.0;
                RGB { r: r,g: g,b: b, }
            }

            fn new_rgba(r: u8,g: u8,b: u8,_a: u8) -> RGB<$t> {
                let r = (r as $t) / 255.0;
                let g = (g as $t) / 255.0;
                let b = (b as $t) / 255.0;
                RGB { r: r,g: g,b: b, }
            }

            fn set_r(&mut self,r: u8) {
                self.r = (r as $t) / 255.0;
            }

            fn set_g(&mut self,g: u8) {
                self.g = (g as $t) / 255.0;
            }

            fn set_b(&mut self,b: u8) {
                self.b = (b as $t) / 255.0;
            }

            fn set_a(&mut self,_a: u8) {
            }

            fn r(&self) -> u8 {
                (self.r.clamp1() * 255.0) as u8
            }

            fn g(&self) -> u8 {
                (self.g.clamp1() * 255.0) as u8
            }

            fn b(&self) -> u8 {
                (self.b.clamp1() * 255.0) as u8
            }

            fn a(&self) -> u8 {
                255
            }
        }

        impl Pixel for RGBA<$t> {
            fn new_rgb(r: u8,g: u8,b: u8) -> RGBA<$t> {
                let r = (r as $t) / 255.0;
                let g = (g as $t) / 255.0;
                let b = (b as $t) / 255.0;
                RGBA { r: r,g: g,b: b,a: 1.0, }
            }

            fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> RGBA<$t> {
                let r = (r as $t) / 255.0;
                let g = (g as $t) / 255.0;
                let b = (b as $t) / 255.0;
                let a = (a as $t) / 255.0;
                RGBA { r: r,g: g,b: b,a: a, }
            }

            fn set_r(&mut self,r: u8) {
                self.r = (r as $t) / 255.0;
            }

            fn set_g(&mut self,g: u8) {
                self.g = (g as $t) / 255.0;
            }

            fn set_b(&mut self,b: u8) {
                self.b = (b as $t) / 255.0;
            }

            fn set_a(&mut self,a: u8) {
                self.a = (a as $t) / 255.0;
            }

            fn r(&self) -> u8 {
                (self.r.clamp1() * 255.0) as u8
            }

            fn g(&self) -> u8 {
                (self.g.clamp1() * 255.0) as u8
            }

            fn b(&self) -> u8 {
                (self.b.clamp1() * 255.0) as u8
            }

            fn a(&self) -> u8 {
                (self.a.clamp1() * 255.0) as u8
            }
        }
    )
);

impl_pixel!(f32);
impl_pixel!(f64);

#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct R3G3B2 {
    pub d: u8,
}

impl Pixel for R3G3B2 {
    fn new_rgb(r: u8,g: u8,b: u8) -> R3G3B2 {
        let r = r >> 5;
        let g = g >> 5;
        let b = b >> 6;
        R3G3B2 { d: (r << 5) | (g << 2) | b, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,_a: u8) -> R3G3B2 {
        let r = r >> 5;
        let g = g >> 5;
        let b = b >> 6;
        R3G3B2 { d: (r << 5) | (g << 2) | b, }
    }

    fn set_r(&mut self,r: u8) {
        let r = r >> 5;
        self.d = (self.d & 0x1F) | (r << 5);
    }

    fn set_g(&mut self,g: u8) {
        let g = g >> 5;
        self.d = (self.d & 0xE3) | (g << 2);
    }

    fn set_b(&mut self,b: u8) {
        let b = b >> 5;
        self.d = (self.d & 0xFC) | b;
    }

    fn set_a(&mut self,_a: u8) {
    }

    fn r(&self) -> u8 {
        let r = self.d >> 5;
        (r << 5) | (r << 2) | (r >> 1)
    }

    fn g(&self) -> u8 {
        let g = (self.d >> 2) & 0x07;
        (g << 5) | (g << 2) | (g >> 1)
    }

    fn b(&self) -> u8 {
        let b = self.d & 0x03;
        (b << 6) | (b << 4) | (b << 2) | b
    }

    fn a(&self) -> u8 {
        255
    }
}

impl PartialEq<R3G3B2> for R3G3B2 {
    fn eq(&self,other: &R3G3B2) -> bool {
        self.d == other.d
    }
}

impl Zero for R3G3B2 {
    fn zero() -> R3G3B2 {
        R3G3B2 { d: 0x00, }
    }
}

macro_rules! impl_r3g3b2 (
    ($t:ty) => (
        impl From<RGB<$t>> for R3G3B2 {
            fn from(c: RGB<$t>) -> R3G3B2 {
                let r: u8 = (c.r.clamp1() * 7.0) as u8;
                let g: u8 = (c.g.clamp1() * 7.0) as u8;
                let b: u8 = (c.b.clamp1() * 3.0) as u8;
                R3G3B2 { d: (r << 5) | (g << 2) | b, }
            }
        }
        
        impl From<R3G3B2> for RGB<$t> {
            fn from(c: R3G3B2) -> RGB<$t> {
                let r = ((c.d >> 5) as $t) / 7.0;
                let g = (((c.d >> 2) & 0x07) as $t) / 7.0;
                let b = ((c.d & 0x03) as $t) / 3.0;
                RGB { r: r,g: g,b: b, }
            }
        }

        impl From<RGBA<$t>> for R3G3B2 {
            fn from(c: RGBA<$t>) -> R3G3B2 {
                let r: u8 = (c.r.clamp1() * 7.0) as u8;
                let g: u8 = (c.g.clamp1() * 7.0) as u8;
                let b: u8 = (c.b.clamp1() * 3.0) as u8;
                R3G3B2 { d: (r << 5) | (g << 2) | b, }
            }
        }
        
        impl From<R3G3B2> for RGBA<$t> {
            fn from(c: R3G3B2) -> RGBA<$t> {
                let r = ((c.d >> 5) as $t) / 7.0;
                let g = (((c.d >> 2) & 0x07) as $t) / 7.0;
                let b = ((c.d & 0x03) as $t) / 3.0;
                RGBA { r: r,g: g,b: b,a: 1.0, }
            }
        }        
    )
);

impl_r3g3b2!(f32);
impl_r3g3b2!(f64);

impl From<R3G3B2> for u32 {
    fn from(c: R3G3B2) -> u32 {
        let mut r = (c.d >> 5) as u32;
        let mut g = ((c.d >> 2) & 0x07) as u32;
        let mut b = (c.d & 0x03) as u32;
        r = (r << 5) | (r << 2) | (r >> 1);
        g = (g << 5) | (g << 2) | (g >> 1);
        b = (b << 6) | (b << 4) | (b << 2) | b;
        0xFF000000 | (r << 16) | (g << 8) | b
    }
}

impl From<R3G3B2> for RGB8 {
    fn from(c: R3G3B2) -> RGB8 {
        let mut r = (c.d >> 5) as u8;
        let mut g = ((c.d >> 2) & 0x07) as u8;
        let mut b = (c.d & 0x03) as u8;
        r = (r << 5) | (r << 2) | (r >> 1);
        g = (g << 5) | (g << 2) | (g >> 1);
        b = (b << 6) | (b << 4) | (b << 2) | b;
        RGB8 { r: r,g: g,b: b, }
    }
}

#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct ARGB2 {
    pub d: u8,
}

impl Pixel for ARGB2 {
    fn new_rgb(r: u8,g: u8,b: u8) -> ARGB2 {
        let r = r >> 6;
        let g = g >> 6;
        let b = b >> 6;
        ARGB2 { d: 0xC0 | (r << 4) | (g << 2) | b, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> ARGB2 {
        let r = r >> 6;
        let g = g >> 6;
        let b = b >> 6;
        let a = a >> 6;
        ARGB2 { d: (a << 6) | (r << 4) | (g << 2) | b, }
    }

    fn set_r(&mut self,r: u8) {
        let r = r >> 6;
        self.d = (self.d & 0xCF) | (r << 4);
    }

    fn set_g(&mut self,g: u8) {
        let g = g >> 6;
        self.d = (self.d & 0xF3) | (g << 2);
    }

    fn set_b(&mut self,b: u8) {
        let b = b >> 6;
        self.d = (self.d & 0xFC) | b;
    }

    fn set_a(&mut self,a: u8) {
        let a = a >> 6;
        self.d = (self.d & 0x3F) | (a << 6);
    }

    fn r(&self) -> u8 {
        let r = (self.d >> 4) & 0x03;
        (r << 6) | (r << 4) | (r << 2) | r
    }

    fn g(&self) -> u8 {
        let g = (self.d >> 2) & 0x03;
        (g << 6) | (g << 4) | (g << 2) | g
    }

    fn b(&self) -> u8 {
        let b = self.d & 0x03;
        (b << 6) | (b << 4) | (b << 2) | b
    }

    fn a(&self) -> u8 {
        let a = (self.d >> 6) & 0x03;
        (a << 6) | (a << 4) | (a << 2) | a
    }
}

impl PartialEq<ARGB2> for ARGB2 {
    fn eq(&self,other: &ARGB2) -> bool {
        self.d == other.d
    }
}

impl Zero for ARGB2 {
    fn zero() -> ARGB2 {
        ARGB2 { d: 0x00, }
    }
}

macro_rules! impl_argb2 (
    ($t:ty) => (
        impl From<RGB<$t>> for ARGB2 {
            fn from(c: RGB<$t>) -> ARGB2 {
                let r = (c.r.clamp1() * 3.0) as u8;
                let g = (c.g.clamp1() * 3.0) as u8;
                let b = (c.b.clamp1() * 3.0) as u8;
                ARGB2 { d: 0xC0 | (r << 4) | (g << 2) | b, }
            }
        }

        impl From<ARGB2> for RGB<$t> {
            fn from(c: ARGB2) -> RGB<$t> {
                let r = (((c.d >> 4) & 0x03) as $t) / 3.0;
                let g = (((c.d >> 2) & 0x03) as $t) / 3.0;
                let b = ((c.d & 0x03) as $t) / 3.0;
                RGB { r: r,g: g,b: b, }
            }
        }

        impl From<RGBA<$t>> for ARGB2 {
            fn from(c: RGBA<$t>) -> ARGB2 {
                let r = (c.r.clamp1() * 3.0) as u8;
                let g = (c.g.clamp1() * 3.0) as u8;
                let b = (c.b.clamp1() * 3.0) as u8;
                let a = (c.a.clamp1() * 3.0) as u8;
                ARGB2 { d: (a << 6) | (r << 4) | (g << 2) | b, }
            }
        }

        impl From<ARGB2> for RGBA<$t> {
            fn from(c: ARGB2) -> RGBA<$t> {
                let r = (((c.d >> 4) & 0x03) as $t) / 3.0;
                let g = (((c.d >> 2) & 0x03) as $t) / 3.0;
                let b = ((c.d & 0x03) as $t) / 3.0;
                let a = ((c.d >> 6) as $t) / 3.0;
                RGBA { r: r,g: g,b: b,a: a, }
            }
        }
    )
);

impl_argb2!(f32);
impl_argb2!(f64);

impl From<u32> for ARGB2 {
    fn from(c: u32) -> ARGB2 {
        let r = ((c >> 22) & 0x03) as u8;
        let g = ((c >> 14) & 0x03) as u8;
        let b = (c & 0x03) as u8;
        let a = ((c >> 30) & 0x03) as u8;
        ARGB2 { d: (a << 6) | (r << 4) | (g << 2) | b, }
    }
}

impl From<ARGB2> for u32 {
    fn from(c: ARGB2) -> u32 {
        let mut r = ((c.d >> 4) & 0x03) as u32;
        let mut g = ((c.d >> 2) & 0x03) as u32;
        let mut b = (c.d & 0x03) as u32;
        let mut a = (c.d >> 6) as u32;
        r = (r << 6) | (r << 4) | (r << 2) | r;
        g = (g << 6) | (g << 4) | (g << 2) | g;
        b = (b << 6) | (b << 4) | (b << 2) | b;
        a = (a << 6) | (a << 4) | (a << 2) | a;
        (a << 24) | (r << 16) | (g << 8) | b
    }
}
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct R5G6B5 {
    pub d: u16,
}

impl Pixel for R5G6B5 {
    fn new_rgb(r: u8,g: u8,b: u8) -> R5G6B5 {
        let r = (r >> 3) as u16;
        let g = (g >> 2) as u16;
        let b = (b >> 3) as u16;
        R5G6B5 { d: (r << 11) | (g << 5) | b, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,_a: u8) -> R5G6B5 {
        let r = (r >> 3) as u16;
        let g = (g >> 2) as u16;
        let b = (b >> 3) as u16;
        R5G6B5 { d: (r << 11) | (g << 5) | b, }
    }

    fn set_r(&mut self,r: u8) {
        let r = (r >> 3) as u16;
        self.d = (self.d & 0x07FF) | (r << 11);
    }

    fn set_g(&mut self,g: u8) {
        let g = (g >> 2) as u16;
        self.d = (self.d & 0xF81F) | (g << 5);
    }

    fn set_b(&mut self,b: u8) {
        let b = (b >> 3) as u16;
        self.d = (self.d & 0xFFE0) | b;
    }

    fn set_a(&mut self,_a: u8) {
    }

    fn r(&self) -> u8 {
        let r = ((self.d >> 11) & 0x1F) as u8;
        (r << 3) | (r >> 2)
    }

    fn g(&self) -> u8 {
        let g = ((self.d >> 5) & 0x3F) as u8;
        (g << 2) | (g >> 4)
    }

    fn b(&self) -> u8 {
        let b = (self.d & 0x1F) as u8;
        (b << 3) | (b >> 2)
    }

    fn a(&self) -> u8 {
        255
    }
}

impl PartialEq<R5G6B5> for R5G6B5 {
    fn eq(&self,other: &R5G6B5) -> bool {
        self.d == other.d
    }
}

impl Zero for R5G6B5 {
    fn zero() -> R5G6B5 {
        R5G6B5 { d: 0x0000, }
    }
}

macro_rules! impl_r5g6b5 (
    ($t:ty) => (
        impl From<RGB<$t>> for R5G6B5 {
            fn from(c: RGB<$t>) -> R5G6B5 {
                let r = (c.r.clamp1() * 31.0) as u16;
                let g = (c.g.clamp1() * 63.0) as u16;
                let b = (c.b.clamp1() * 31.0) as u16;
                R5G6B5 { d: (r << 11) | (g << 5) | b, }
            }
        }
        
        impl From<R5G6B5> for RGB<$t> {
            fn from(c: R5G6B5) -> RGB<$t> {
                let r = (((c.d >> 11) & 0x001F) as $t) / 31.0;
                let g = (((c.d >> 5) & 0x003F) as $t) / 63.0;
                let b = ((c.d & 0x001F) as $t) / 31.0;
                RGB { r: r,g: g,b: b, }
            }
        }
        
        impl From<RGBA<$t>> for R5G6B5 {
            fn from(c: RGBA<$t>) -> R5G6B5 {
                let r = (c.r.clamp1() * 31.0) as u16;
                let g = (c.g.clamp1() * 63.0) as u16;
                let b = (c.b.clamp1() * 31.0) as u16;
                R5G6B5 { d: (r << 11) | (g << 5) | b, }
            }
        }

        impl From<R5G6B5> for RGBA<$t> {
            fn from(c: R5G6B5) -> RGBA<$t> {
                let r = (((c.d >> 11) & 0x001F) as $t) / 31.0;
                let g = (((c.d >> 5) & 0x003F) as $t) / 63.0;
                let b = ((c.d & 0x001F) as $t) / 31.0;
                RGBA { r: r,g: g,b: b,a: 1.0, }
            }
        }
    )
);

impl_r5g6b5!(f32);
impl_r5g6b5!(f64);

impl From<u32> for R5G6B5 {
    fn from(c: u32) -> R5G6B5 {
        let r = ((c >> 19) & 0x001F) as u16;
        let g = ((c >> 10) & 0x003F) as u16;
        let b = ((c >> 3) & 0x001F) as u16;
        R5G6B5 { d: (r << 11) | (g << 5) | b, }
    }
}

impl From<R5G6B5> for u32 {
    fn from(c: R5G6B5) -> u32 {
        let mut r = ((c.d >> 11) & 0x001F) as u32;
        let mut g = ((c.d >> 5) & 0x003F) as u32;
        let mut b = (c.d & 0x001F) as u32;
        r = (r << 3) | (r >> 2);
        g = (g << 3) | (g >> 2);
        b = (b << 3) | (b >> 2);
        0xFF000000 | (r << 16) | (g << 8) | b
    }
}

#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct ARGB4 {
    pub d: u16,
}

impl Pixel for ARGB4 {
    fn new_rgb(r: u8,g: u8,b: u8) -> ARGB4 {
        let r = (r >> 4) as u16;
        let g = (g >> 4) as u16;
        let b = (b >> 4) as u16;
        ARGB4 { d: 0xF000 | (r << 8) | (g << 4) | b, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> ARGB4 {
        let r = (r >> 4) as u16;
        let g = (g >> 4) as u16;
        let b = (b >> 4) as u16;
        let a = (a >> 4) as u16;
        ARGB4 { d: (a << 12) | (r << 8) | (g << 4) | b, }
    }   

    fn set_r(&mut self,r: u8) {
        let r = (r >> 4) as u16;
        self.d = (self.d & 0xF0FF) | (r << 8);
    }

    fn set_g(&mut self,g: u8) {
        let g = (g >> 4) as u16;
        self.d = (self.d & 0xFF0F) | (g << 4);
    }

    fn set_b(&mut self,b: u8) {
        let b = (b >> 4) as u16;
        self.d = (self.d & 0xFFF0) | b;
    }

    fn set_a(&mut self,a: u8) {
        let a = (a >> 4) as u16;
        self.d = (self.d & 0x0FFF) | (a << 12);
    }

    fn r(&self) -> u8 {
        let r = ((self.d >> 8) & 0x0F) as u8;
        (r << 4) | 4
    }

    fn g(&self) -> u8 {
        let g = ((self.d >> 8) & 0x0F) as u8;
        (g << 4) | 4
    }

    fn b(&self) -> u8 {
        let b = (self.d & 0x0F) as u8;
        (b << 4) | 4
    }

    fn a(&self) -> u8 {
        let a = (self.d >> 12) as u8;
        (a << 4) | 4
    }
}

impl PartialEq<ARGB4> for ARGB4 {
    fn eq(&self,other: &ARGB4) -> bool {
        self.d == other.d
    }
}

impl Zero for ARGB4 {
    fn zero() -> ARGB4 {
        ARGB4 { d: 0x0000, }
    }
}

macro_rules! impl_argb4 (
    ($t:ty) => (
        impl From<RGB<$t>> for ARGB4 {
            fn from(c: RGB<$t>) -> ARGB4 {
                let r = (c.r.clamp1() * 15.0) as u16;
                let g = (c.g.clamp1() * 15.0) as u16;
                let b = (c.b.clamp1() * 15.0) as u16;
                ARGB4 { d: 0xF000 | (r << 8) | (g << 4) | b, }
            }
        }
        
        impl From<ARGB4> for RGB<$t> {
            fn from(c: ARGB4) -> RGB<$t> {
                let r = (((c.d >> 8) & 0x000F) as $t) / 15.0;
                let g = (((c.d >> 4) & 0x000F) as $t) / 15.0;
                let b = ((c.d & 0x000F) as $t) / 15.0;
                RGB { r: r,g: g,b: b, }
            }
        }
        
        impl From<RGBA<$t>> for ARGB4 {
            fn from(c: RGBA<$t>) -> ARGB4 {
                let r = (c.r.clamp1() * 15.0) as u16;
                let g = (c.g.clamp1() * 15.0) as u16;
                let b = (c.b.clamp1() * 15.0) as u16;
                let a = (c.a.clamp1() * 15.0) as u16;
                ARGB4 { d: (a << 12) | (r << 8) | (g << 4) | b, }
            }
        }
        
        impl From<ARGB4> for RGBA<$t> {
            fn from(c: ARGB4) -> RGBA<$t> {
                let r = (((c.d >> 8) & 0x000F) as $t) / 15.0;
                let g = (((c.d >> 4) & 0x000F) as $t) / 15.0;
                let b = ((c.d & 0x000F) as $t) / 15.0;
                let a = (((c.d >> 12) & 0x000F) as $t) / 15.0;
                RGBA { r: r,g: g,b: b,a: a, }
            }
        }
    )
);

impl_argb4!(f32);
impl_argb4!(f64);

impl From<u32> for ARGB4 {
    fn from(c: u32) -> ARGB4 {
        let r = ((c >> 20) & 0x000F) as u16;
        let g = ((c >> 12) & 0x000F) as u16;
        let b = ((c >> 4) & 0x000F) as u16;
        let a = ((c >> 28) & 0x000F) as u16;
        ARGB4 { d: (a << 12) | (r << 8) | (g << 4) | b, }
    }
}

impl From<ARGB4> for u32 {
    fn from(c: ARGB4) -> u32 {
        let mut r = ((c.d >> 8) & 0x000F) as u32;
        let mut g = ((c.d >> 4) & 0x000F) as u32;
        let mut b = (c.d & 0x000F) as u32;
        let mut a = ((c.d >> 12) & 0x000F) as u32;
        a = (a << 4) | a;
        r = (r << 4) | r;
        g = (g << 4) | g;
        b = (b << 4) | b;
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct A1RGB5 {
    pub d: u16,
}

impl Pixel for A1RGB5 {
    fn new_rgb(r: u8,g: u8,b: u8) -> A1RGB5 {
        let r = (r >> 3) as u16;
        let g = (g >> 3) as u16;
        let b = (b >> 3) as u16;
        A1RGB5 { d: 0x8000 | (r << 10) | (g << 5) | b, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> A1RGB5 {
        let r = (r >> 3) as u16;
        let g = (g >> 3) as u16;
        let b = (b >> 3) as u16;
        let a = (a >> 7) as u16;
        A1RGB5 { d: (a << 15) | (r << 10) | (g << 5) | b, }
    }

    fn set_r(&mut self,r: u8) {
        let r = (r >> 3) as u16;
        self.d = (self.d & 0x83FF) | (r << 10);
    }

    fn set_g(&mut self,g: u8) {
        let g = (g >> 3) as u16;
        self.d = (self.d & 0xFC1F) | (g << 5);
    }

    fn set_b(&mut self,b: u8) {
        let b = (b >> 3) as u16;
        self.d = (self.d & 0xFFE0) | b;
    }

    fn set_a(&mut self,a: u8) {
        let a = (a >> 7) as u16;
        self.d = (self.d & 0x7FFF) | (a << 15);
    }

    fn r(&self) -> u8 {
        let r = ((self.d >> 10) & 0x1F) as u8;
        (r << 3) | (r >> 2)
    }

    fn g(&self) -> u8 {
        let g = ((self.d >> 5) & 0x1F) as u8;
        (g << 3) | (g >> 2)
    }

    fn b(&self) -> u8 {
        let b = (self.d & 0x1F) as u8;
        (b << 3) | (b >> 2)
    }

    fn a(&self) -> u8 {
        if (self.d & 0x8000) != 0 {
            255
        }
        else {
            0
        }
    }
}

impl PartialEq<A1RGB5> for A1RGB5 {
    fn eq(&self,other: &A1RGB5) -> bool {
        self.d == other.d
    }
}

impl Zero for A1RGB5 {
    fn zero() -> A1RGB5 {
        A1RGB5 { d: 0x0000, }
    }
}

macro_rules! impl_a1rgb5 (
    ($t:ty) => (
        impl From<RGB<$t>> for A1RGB5 {
            fn from(c: RGB<$t>) -> A1RGB5 {
                let r = (c.r.clamp1() * 31.0) as u16;
                let g = (c.g.clamp1() * 31.0) as u16;
                let b = (c.b.clamp1() * 31.0) as u16;
                A1RGB5 { d: 0x8000 | (r << 10) | (g << 5) | b, }
            }
        }
        
        impl From<A1RGB5> for RGB<$t> {
            fn from(c: A1RGB5) -> RGB<$t> {
                let r = (((c.d >> 10) & 0x001F) as $t) / 31.0;
                let g = (((c.d >> 5) & 0x001F) as $t) / 31.0;
                let b = ((c.d & 0x001F) as $t) / 31.0;
                RGB { r: r,g: g,b: b, }
            }
        }
        
        impl From<RGBA<$t>> for A1RGB5 {
            fn from(c: RGBA<$t>) -> A1RGB5 {
                let r = (c.r.clamp1() * 31.0) as u16;
                let g = (c.g.clamp1() * 31.0) as u16;
                let b = (c.b.clamp1() * 31.0) as u16;
                let a = c.a.clamp1() as u16;
                A1RGB5 { d: (a << 15) | (r << 10) | (g << 5) | b, }
            }
        }
        
        impl From<A1RGB5> for RGBA<$t> {
            fn from(c: A1RGB5) -> RGBA<$t> {
                let r = (((c.d >> 10) & 0x001F) as $t) / 31.0;
                let g = (((c.d >> 5) & 0x001F) as $t) / 31.0;
                let b = ((c.d & 0x001F) as $t) / 31.0;
                let a = ((c.d >> 15) & 0x0001) as $t;
                RGBA { r: r,g: g,b: b,a: a, }
            }
        }
    )
);

impl_a1rgb5!(f32);
impl_a1rgb5!(f64);

impl From<u32> for A1RGB5 {
    fn from(c: u32) -> A1RGB5 {
        let r = ((c >> 19) & 0x001F) as u16;
        let g = ((c >> 11) & 0x001F) as u16;
        let b = ((c >> 3) & 0x001F) as u16;
        let a = ((c >> 31) & 0x0001) as u16;
        A1RGB5 { d: (a << 15) | (r << 10) | (g << 5) | b, }
    }
}

impl From<A1RGB5> for u32 {
    fn from(c: A1RGB5) -> u32 {
        let mut r = ((c.d >> 10) & 0x001F) as u32;
        let mut g = ((c.d >> 5) & 0x001F) as u32;
        let mut b = (c.d & 0x001F) as u32;
        let mut a = ((c.d >> 15) & 0x0001) as u32;
        r = (r << 3) | (r >> 2);
        g = (g << 3) | (g >> 2);
        b = (b << 3) | (b >> 2);
        a = if a != 0 {
            0xFF
        }
        else {
            0x00
        };
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel for RGB8 {
    fn new_rgb(r: u8,g: u8,b: u8) -> RGB8 {
        RGB8 { r: r,g: g,b: b, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,_a: u8) -> RGB8 {
        RGB8 { r: r,g: g,b: b, }
    }

    fn set_r(&mut self,r: u8) {
        self.r = r
    }

    fn set_g(&mut self,g: u8) {
        self.g = g
    }

    fn set_b(&mut self,b: u8) {
        self.b = b
    }

    fn set_a(&mut self,_a: u8) {
    }

    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }

    fn a(&self) -> u8 {
        255
    }
}

impl PartialEq<RGB8> for RGB8 {
    fn eq(&self,other: &RGB8) -> bool {
        (self.r == other.r) &&
        (self.g == other.g) &&
        (self.b == other.b)
    }
}

impl Zero for RGB8 {
    fn zero() -> RGB8 {
        RGB8 { r: 0x00,g: 0x00,b: 0x00, }
    }
}

macro_rules! impl_rgb8 (
    ($t:ty) => (
        impl From<RGB<$t>> for RGB8 {
            fn from(c: RGB<$t>) -> RGB8 {
                let r = (c.r.clamp1() * 255.0) as u8;
                let g = (c.g.clamp1() * 255.0) as u8;
                let b = (c.b.clamp1() * 255.0) as u8;
                RGB8 { r: r,g: g,b: b, }
            }
        }
        
        impl From<RGB8> for RGB<$t> {
            fn from(c: RGB8) -> RGB<$t> {
                let r = (c.r as $t) / 255.0;
                let g = (c.g as $t) / 255.0;
                let b = (c.b as $t) / 255.0;
                RGB { r: r,g: g,b: b, }
            }
        }
        
        impl From<RGBA<$t>> for RGB8 {
            fn from(c: RGBA<$t>) -> RGB8 {
                let r = (c.r.clamp1() * 255.0) as u8;
                let g = (c.g.clamp1() * 255.0) as u8;
                let b = (c.b.clamp1() * 255.0) as u8;
                RGB8 { r: r,g: g,b: b, }
            }
        }
        
        impl From<RGB8> for RGBA<$t> {
            fn from(c: RGB8) -> RGBA<$t> {
                let r = (c.r as $t) / 255.0;
                let g = (c.g as $t) / 255.0;
                let b = (c.b as $t) / 255.0;
                RGBA { r: r,g: g,b: b,a: 1.0, }
            }
        }
    )
);

impl_rgb8!(f32);
impl_rgb8!(f64);

impl From<u32> for RGB8 {
    fn from(c: u32) -> RGB8 {
        let r = ((c >> 16) & 0xFF) as u8;
        let g = ((c >> 8) & 0xFF) as u8;
        let b = (c & 0xFF) as u8;
        RGB8 { r: r,g: g,b: b, }
    }
}

impl From<RGB8> for u32 {
    fn from(c: RGB8) -> u32 {
        let r = c.r as u32;
        let g = c.g as u32;
        let b = c.b as u32;
        0xFF000000 | (r << 16) | (g << 8) | b
    }
}

#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct ARGB8 {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl Pixel for ARGB8 {
    fn new_rgb(r: u8,g: u8,b: u8) -> ARGB8 {
        ARGB8 { r: r,g: g,b: b,a: 255, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> ARGB8 {
        ARGB8 { r: r,g: g,b: b,a: a, }
    }

    fn set_r(&mut self,r: u8) {
        self.r = r
    }

    fn set_g(&mut self,g: u8) {
        self.g = g
    }

    fn set_b(&mut self,b: u8) {
        self.b = b
    }

    fn set_a(&mut self,a: u8) {
        self.a = a
    }

    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }

    fn a(&self) -> u8 {
        self.a
    }
}

impl PartialEq<ARGB8> for ARGB8 {
    fn eq(&self,other: &ARGB8) -> bool {
        (self.r == other.r) &&
        (self.g == other.g) &&
        (self.b == other.b) &&
        (self.a == other.a)
    }
}

impl Zero for ARGB8 {
    fn zero() -> ARGB8 {
        ARGB8 { r: 0x00,g: 0x00,b: 0x00,a: 0x00, }
    }
}

macro_rules! impl_argb8 (
    ($t:ty) => (
        impl From<RGB<$t>> for ARGB8 {
            fn from(c: RGB<$t>) -> ARGB8 {
                let r = (c.r.clamp1() * 255.0) as u8;
                let g = (c.g.clamp1() * 255.0) as u8;
                let b = (c.b.clamp1() * 255.0) as u8;
                ARGB8 { r: r,g: g,b: b,a: 255, }
            }
        }
        
        impl From<ARGB8> for RGB<$t> {
            fn from(c: ARGB8) -> RGB<$t> {
                let r = (c.r as $t) / 255.0;
                let g = (c.g as $t) / 255.0;
                let b = (c.b as $t) / 255.0;
                RGB { r: r,g: g,b: b, }
            }
        }
        
        impl From<RGBA<$t>> for ARGB8 {
            fn from(c: RGBA<$t>) -> ARGB8 {
                let r = (c.r.clamp1() * 255.0) as u8;
                let g = (c.g.clamp1() * 255.0) as u8;
                let b = (c.b.clamp1() * 255.0) as u8;
                let a = (c.a.clamp1() * 255.0) as u8;
                ARGB8 { r: r,g: g,b: b,a: a, }
            }
        }
        
        impl From<ARGB8> for RGBA<$t> {
            fn from(c: ARGB8) -> RGBA<$t> {
                let r = (c.r as $t) / 255.0;
                let g = (c.g as $t) / 255.0;
                let b = (c.b as $t) / 255.0;
                let a = (c.a as $t) / 255.0;
                RGBA { r: r,g: g,b: b,a: a, }
            }
        }
    )
);

impl_argb8!(f32);
impl_argb8!(f64);

impl From<u32> for ARGB8 {
    fn from(c: u32) -> ARGB8 {
        let r = ((c >> 16) & 0xFF) as u8;
        let g = ((c >> 8) & 0xFF) as u8;
        let b = (c & 0xFF) as u8;
        let a = ((c >> 24) & 0xFF) as u8;
        ARGB8 { r: r,g: g,b: b,a: a, }
    }
}

impl From<ARGB8> for u32 {
    fn from(c: ARGB8) -> u32 {
        let r = c.r as u32;
        let g = c.g as u32;
        let b = c.b as u32;
        let a = c.a as u32;
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct A2RGB10 {
    pub d: u32,
}

impl Pixel for A2RGB10 {
    fn new_rgb(r: u8,g: u8,b: u8) -> A2RGB10 {
        let mut r = r as u32;
        let mut g = g as u32;
        let mut b = b as u32;
        r = (r << 2) | (r >> 6);
        g = (g << 2) | (g >> 6);
        b = (b << 2) | (b >> 6);
        A2RGB10 { d: 0xC0000000 | (r << 20) | (g << 10) | b, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> A2RGB10 {
        let mut r = r as u32;
        let mut g = g as u32;
        let mut b = b as u32;
        let mut a = a as u32;
        r = (r << 2) | (r >> 6);
        g = (g << 2) | (g >> 6);
        b = (b << 2) | (b >> 6);
        a = a >> 6;
        A2RGB10 { d: (a << 30) | (r << 20) | (g << 10) | b, }
    }

    fn set_r(&mut self,r: u8) {
        let r = ((r as u32) << 2) | ((r as u32) >> 6);
        self.d = (self.d & 0xC00FFFFF) | (r << 20);
    }

    fn set_g(&mut self,g: u8) {
        let g = ((g as u32) << 2) | ((g as u32) >> 6);
        self.d = (self.d & 0xFFF003FF) | (g << 10);
    }

    fn set_b(&mut self,b: u8) {
        let b = ((b as u32) << 2) | ((b as u32) >> 6);
        self.d = (self.d & 0xFFFFFC00) | b;
    }

    fn set_a(&mut self,a: u8) {
        let a = (a as u32) >> 6;
        self.d = (self.d & 0x3FFFFFFF) | (a << 30);
    }

    fn r(&self) -> u8 {
        ((self.d >> 22) & 0xFF) as u8
    }

    fn g(&self) -> u8 {
        ((self.d >> 12) & 0xFF) as u8
    }

    fn b(&self) -> u8 {
        ((self.d >> 2) & 0xFF) as u8
    }

    fn a(&self) -> u8 {
        let a = (self.d >> 30) as u8;
        (a << 6) | (a << 4) | (a << 2) | a
    }
}

impl Zero for A2RGB10 {
    fn zero() -> A2RGB10 {
        A2RGB10 { d: 0x00000000, }
    }
}

impl PartialEq<A2RGB10> for A2RGB10 {
    fn eq(&self,other: &A2RGB10) -> bool {
        self.d == other.d
    }
}

macro_rules! impl_a2rgb10 (
    ($t:ty) => (
        impl From<RGB<$t>> for A2RGB10 {
            fn from(c: RGB<$t>) -> A2RGB10 {
                let r = (c.r.clamp1() * 1023.0) as u32;
                let g = (c.g.clamp1() * 1023.0) as u32;
                let b = (c.b.clamp1() * 1023.0) as u32;
                A2RGB10 { d: 0xC0000000 | (r << 20) | (g << 10) | b, }
            }
        }
        
        impl From<A2RGB10> for RGB<$t> {
            fn from(c: A2RGB10) -> RGB<$t> {
                let r = (((c.d >> 20) & 0x000003FF) as $t) / 1023.0;
                let g = (((c.d >> 10) & 0x000003FF) as $t) / 1023.0;
                let b = ((c.d & 0x000003FF) as $t) / 1023.0;
                RGB { r: r,g: g,b: b, }
            }
        }
        
        impl From<RGBA<$t>> for A2RGB10 {
            fn from(c: RGBA<$t>) -> A2RGB10 {
                let r = (c.r.clamp1() * 1023.0) as u32;
                let g = (c.g.clamp1() * 1023.0) as u32;
                let b = (c.b.clamp1() * 1023.0) as u32;
                let a = (c.a.clamp1() * 3.0) as u32;
                A2RGB10 { d: (a << 30) | (r << 20) | (g << 10) | b, }
            }
        }
        
        impl From<A2RGB10> for RGBA<$t> {
            fn from(c: A2RGB10) -> RGBA<$t> {
                let r = (((c.d >> 20) & 0x000003FF) as $t) / 1023.0;
                let g = (((c.d >> 10) & 0x000003FF) as $t) / 1023.0;
                let b = ((c.d & 0x000003FF) as $t) / 1023.0;
                let a = (((c.d >> 30) & 0x00000003) as $t) / 3.0;
                RGBA { r: r,g: g,b: b,a: a, }
            }
        }
    )
);

impl_a2rgb10!(f32);
impl_a2rgb10!(f64);

impl From<u32> for A2RGB10 {
    fn from(c: u32) -> A2RGB10 {
        let mut r = (c >> 16) & 0x000000FF;
        let mut g = (c >> 8) & 0x000000FF;
        let mut b = c & 0x000000FF;
        let mut a = (c >> 24) & 0x00000003;
        r = (r << 2) | (r >> 6);
        g = (g << 2) | (g >> 6);
        b = (b << 2) | (b >> 6);
        a = a >> 6;
        A2RGB10 { d: (a << 30) | (r << 20) | (g << 10) | b, }
    }
}

impl From<A2RGB10> for u32 {
    fn from(c: A2RGB10) -> u32 {
        let r = ((c.d >> 22) & 0x000000FF) as u32;
        let g = ((c.d >> 12) & 0x000000FF) as u32;
        let b = ((c.d >> 2) & 0x000000FF) as u32;
        let mut a = ((c.d >> 30) & 0x00000003) as u32;
        a = (a << 6) | (a << 4) | (a << 2) | a;
        (a << 24) | (r << 16) | (g << 8) | b
    }
}
