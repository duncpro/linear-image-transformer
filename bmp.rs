use crate::error::AnyError;
use crate::raster::Raster;
use crate::raster::Pixel;

// routines for reading BMP files

pub fn read_bmp<R>(input: &mut R) -> Result<Raster, AnyError>
where R: std::io::Read + std::io::Seek
{
    // read the file header
    assert_eq!(read_text(input, 2)?, "BM");
    let img_size = read_u32_le(input)?;
    assert_eq!(read_u16_le(input)?, 0);
    assert_eq!(read_u16_le(input)?, 0);
    let offset = read_u32_le(input)?;  

    // read the image header
    let img_head_size = read_u32_le(input)?;
    let width = usize::try_from(read_u32_le(input)?)
        .expect("width too large to fit into usize on this platform");
    let (porder, height) = read_height_field(input)?;
    assert_eq!(read_u16_le(input)?, 1);
    let bpp /* bits per pixel */ = read_u16_le(input)?;
    assert_eq!(bpp, 24, "only 24 bits per pixel is supported");
    let compression_t = read_u32_le(input)?;
    assert_eq!(compression_t, 0, "only uncompressed images supported");

    input.seek(std::io::SeekFrom::Start(u64::from(offset)))?;

    let size = height * width;    
    let mut pixels: Vec<Pixel> = Vec::with_capacity(size);
    
    assert!(porder == PixelOrder::Normal, "only normal pixel order is supported");
    for _ in 0..size { pixels.push(read_color(input)?); }
    
    Ok(Raster { pixels, width })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PixelOrder { Normal, Strange }

fn read_height_field<R>(input: &mut R) -> std::io::Result<(PixelOrder, usize)>
where R: std::io::Read
{
    let value = read_i32_le(input)?;
    let order = if value > 0 { PixelOrder::Strange } else { PixelOrder::Normal };
    let magnitude = usize::try_from(value.unsigned_abs())
        .expect("height too large to fit into usize on this platform");
    return Ok((order, magnitude));
}


fn read_i32_le<R>(input: &mut R) -> std::io::Result<i32>
where R: std::io::Read
{
    let mut buf: [u8; 4] = [0; 4];
    input.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

fn read_u32_le<R>(input: &mut R) -> std::io::Result<u32>
where R: std::io::Read
{
    let mut buf: [u8; 4] = [0; 4];
    input.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u16_le<R>(input: &mut R) -> std::io::Result<u16>
where R: std::io::Read
{
    let mut buf: [u8; 2] = [0; 2];
    input.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))    
}


fn read_i16_le<R>(input: &mut R) -> std::io::Result<i16>
where R: std::io::Read
{
    let mut buf: [u8; 2] = [0; 2];
    input.read_exact(&mut buf)?;
    Ok(i16::from_le_bytes(buf))    
}

fn read_text<R>(input: &mut R, len: usize) -> Result<String, AnyError>
where R: std::io::Read
{
    let mut s = vec![0u8; len];   
    input.read_exact(&mut s[0..len])?;
    Ok(String::from_utf8(s)?)
}

fn read_color<R>(input: &mut R) -> std::io::Result<Pixel> 
where R: std::io::Read
{
    let mut buf: [u8; 3] = [0; 3];
    input.read_exact(&mut buf)?;
    Ok(Pixel { red: buf[0], green: buf[1], blue: buf[2] })
}

// Routines for writing BMP files
// Useful for verifying that the reading routines above
// work properly.

pub fn write_bmp<W>(output: &mut W, img: &Raster) -> std::io::Result<()>
where W: std::io::Write 
{
    // write file header
    output.write_all(&[
        std::ascii::Char::CapitalB.to_u8(),
        std::ascii::Char::CapitalM.to_u8()
    ])?;

    let pxbcount = u32::try_from(img.pixels.len() * 3)
        .expect("image size too large to be measured in u32");
    
    let fsize: u32 = 14 /* file header */ + 40 /* img header */ 
        + pxbcount;
    write_u32_le(output, fsize)?;

    write_u32_le(output, 0)?;
    write_u32_le(output, 54)?;

    // write image header
    write_u32_le(output, 40)?;
    write_u32_le(output, u32::try_from(img.width())
        .expect("width too large to be measured in u32"))?;
    write_i32_le(output, -1 * i32::try_from(img.height())
        .expect("image to tall to be measured in i32"))?;
    write_i16_le(output, 1)?;
    write_i16_le(output, 24)?;
    write_u32_le(output, 0)?; // compression type
    write_u32_le(output, 0)?; // compression size
    write_u32_le(output, 0)?; // prefer pixels per meter x
    write_u32_le(output, 0)?; // prefer pixels per meter y
    write_u32_le(output, 0)?; // color map size
    write_u32_le(output, 0)?; // significant colors

    for color in &img.pixels {
        write_color(output, *color)?;
    }
    
    Ok(())    
}

fn write_i32_le<W>(output: &mut W, value: i32) -> std::io::Result<()>
where W: std::io::Write
{
    let buf = value.to_le_bytes();
    output.write_all(&buf)
}

fn write_u32_le<W>(output: &mut W, value: u32) -> std::io::Result<()>
where W: std::io::Write
{
    let buf = value.to_le_bytes();
    output.write_all(&buf)
}

fn write_u16_le<W>(output: &mut W, value: u16) -> std::io::Result<()>
where W: std::io::Write
{
    let buf = value.to_le_bytes();
    output.write_all(&buf)
}


fn write_i16_le<W>(output: &mut W, value: i16) -> std::io::Result<()>
where W: std::io::Write
{
    let buf = value.to_le_bytes();
    output.write_all(&buf)
}

fn write_color<W>(output: &mut W, value: Pixel) -> std::io::Result<()> 
where W: std::io::Write
{
    let mut buf: [u8; 3] = [value.red, value.green, value.blue];
    output.write_all(&buf)
}
