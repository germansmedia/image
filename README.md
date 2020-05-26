# image

Image manipulation and format implementations, version 0.1.0

Desmond Germans, 2019

## Image Manipulation

### Pixel Formats

From math we have `RGB<T>` and `RGBA<T>` which are prettymuch only valid for `f32` and `f64` (`f32rgb`, `f32rgba`, `f64rgb` and `f64rgba`). These are float formats that also implement mathematical operations, much like vectors.

`RGB<T>` and `RGBA<T>` can be used as floating-point pixel formats because the trait `Pixel` is implemented for them. This means, an image can be loaded directly into a floating-point array. Next to `RGB<T>` and `RGBA<T>` there are a few common pixel formats defined as well:

format      | bits
------------|-----
`R3G3B2`    | RRRGGGBB
`ARGB2`     | AARRGGBB
`ARGB4`     | AAAARRRRGGGGBBBB
`A1RGB5`    | ARRRRRGGGGGBBBBB
`R5G6B5`    | RRRRRGGGGGGBBBBB
`RGB8`      | RRRRRRRRGGGGGGGGBBBBBBBB
`ARGB8`     | AAAAAAAARRRRRRRRGGGGGGGGBBBBBBBB
`A2RGB10`   | AARRRRRRRRRRGGGGGGGGGGBBBBBBBBBB
`RGB<f32>`  | 3 32-bit floats
`RGB<f64>`  | 3 64-bit floats
`RGBA<f32>` | 4 32-bit floats
`RGBA<f64>` | 4 64-bit floats 

All formats can be converted to/from `RGB<T>`, `RGBA<T>` and `u32` via the `From` trait. Use `RGB<T>` and `RGBA<T>` to do calculations, and the other formats for storage.

`RGB8` and `RGBA8` allow direct access to the u8 red, green, blue and alpha components, much like `RGB<T>` and `RGBA<T>`.

### Image Formats

format   | decode         | encode        | tested        | optimization
---------|----------------|---------------|---------------|-------------
`BMP`    | **yes**        | *in progress* | **yes**       |
`PNG`    | **yes**        | *soon*        | **yes**       | improved huffman for LZ77
`JPEG`   | *in progress*  | *soon*        | *in progress* |
`GIF`    | *later*        | *later*       |               |
`TGA`    | *later*        | *later*       |               |
`PBM`    | *later*        | *later*       |               |
`TIFF`   | *later*        | *later*       |               |
`XBM`    | *later*        | *later*       |               |
`WEBP`   | *later*        | *later*       |               |

### How to Use

Each format has a `test`, `decode` and `encode` function, in their own namespace (`bmp`, `png`, `jpeg`, etc.).

#### `fn test(bytes: &[u8]) -> Option<usizexy>`

This tests if `bytes` are a valid image of that format. Returns `Some(usizexy)` if valid. Returns `None` otherwise.

#### `fn decode(bytes: &[u8]) -> Result<Image,String>`

Decodes `bytes` in that format. If succesful, returns `Ok(image)`, otherwise it returns `Err(message)`.

#### `fn encode(image: &Image) -> Result<Vec<u8>,String>`

Encodes `image` in that format. If succesful, returns the encoded bytes as `Ok(Vec<u8>)`, otherwise it returns `Err(message)`.

## But, but, my image doesn't work?!

email it to me at desmond@germansmedia.nl
