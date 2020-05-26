# image

MIT NIHS image manipulation and format implementations, version 0.1.0

Desmond Germans, 2019

If you use this, mention my name somewhere, and also, do one good deed for your friends.

*WARNING: NIHS stands for Not Invented Here Syndrome. If this bothers you, please use the official image format crates (there are a few, for instance `image-0.23.0`) and associated dependencies. If, however, that makes your compiling sessions slow and bloated, feel free to use `image` anyway.*

## Image Manipulation

### Pixel Formats

From math we have `RGB<T>` and `RGBA<T>` which are prettymuch only valid for `f32` and `f64` (`f32rgb`, `f32rgba`, `f64rgb` and `f64rgba`). These are float formats that also implement mathematical operations, much like vectors.

`image` also defines the following bit-precise pixelformats that can be used to define images, etc.:

format  | bits                             | notes
--------+----------------------------------+------
R3G3B2  | RRRGGGBB                         |
ARGB2   | AARRGGBB                         |
A1RGB5  | ARRRRRGGGGGBBBBB                 |
R5G6B5  | RRRRRGGGGGGBBBBB                 |
RGB8    | RRRRRRRRGGGGGGGGBBBBBBBB         | go-to 8-bit RGB access format (`RGB<u8>` defines the wrong mathematical operators)
ARGB8   | AAAAAAAARRRRRRRRGGGGGGGGBBBBBBBB | go-to 8-bit RGBA access format (`RGBA<u8>` defines the wrong mathematical operators)
A2RGB10 | AARRRRRRRRRRGGGGGGGGGGBBBBBBBBBB |

All formats can be converted to/from `RGB<T>`, `RGBA<T>` and `u32` via the `From` trait. Use `RGB<T>` and `RGBA<T>` to do calculations, and the other formats for storage.

### Image Formats

format | decode         | encode        | tested        | optimization
-------|----------------|---------------|---------------|-------------
BMP    | **yes**        | *in progress* | **yes**       |
PNG    | **yes**        | *soon*        | **yes**       | improved huffman for LZ77
JPEG   | *in progress*  | *soon*        | *in progress* |
GIF    | *later*        | *later*       |               |
TGA    | *later*        | *later*       |               |
PBM    | *later*        | *later*       |               |
TIFF   | *later*        | *later*       |               |
XBM    | *later*        | *later*       |               |
WEBP   | *later*        | *later*       |               |

### How to Use

Each format has a `test`, `decode` and `encode` function, in their own namespace (`bmp`, `png`, `jpeg`, etc.).

#### `fn test(bytes: &[u8]) -> Option<usizexy>`

This tests if `bytes` are a valid image of that format. Returns `Some((width,height))` if valid. Returns `None` otherwise.

#### `fn decode(bytes: &[u8]) -> Result<Image,String>`

Decodes `bytes` in that format. If succesful, returns `Ok(image)`, otherwise it returns `Err(message)`.

#### `fn encode(image: &Image) -> Result<Vec<u8>,String>`

Encodes `image` in that format. If succesful, returns the encoded bytes as `Ok(Vec<u8>)`, otherwise it returns `Err(message)`.

## But, but, my image doesn't work?!

email it to me at desmond@germansmedia.nl
