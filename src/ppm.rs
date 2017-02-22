#![allow(dead_code)]
use std::io;


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NetPBM {
    BitMapAscii,
    GrayMapAscii,
    PixMapAscii,
    BitMapBinary,
    GrayMapBinary,
    PixMapBinary,
}

impl NetPBM {
    #[inline(always)]
    fn magic_number(&self) -> &str {
        match *self {
            NetPBM::BitMapAscii   => "P1",
            NetPBM::GrayMapAscii  => "P2",
            NetPBM::PixMapAscii   => "P3",
            NetPBM::BitMapBinary  => "P4",
            NetPBM::GrayMapBinary => "P5",
            NetPBM::PixMapBinary  => "P6",
        }
    }

    fn file_extension(&self) -> &str {
        match *self {
            NetPBM::BitMapAscii  | NetPBM::BitMapBinary  => "pbm",
            NetPBM::GrayMapAscii | NetPBM::GrayMapBinary => "pgm",
            NetPBM::PixMapAscii  | NetPBM::PixMapBinary  => "ppm",
        }
    }
}

// We only support an 8-bit color depth here.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum ColorType {
    BitMap,
    Gray,
    Rgb,
}

impl ColorType {
    fn max_pixel_value(&self) -> usize {
        match *self {
            ColorType::BitMap => 1,
            ColorType::Gray | ColorType::Rgb => 255,
        }
    }
}

pub struct NetPBMEncoder<'a, W: 'a> {
    enc_type: NetPBM,
    writer: &'a mut W,
}

impl<'a, W> NetPBMEncoder<'a, W> where W: 'a + io::Write {
    pub fn new(enc_type: NetPBM, writer: &mut W) -> NetPBMEncoder<W> {
        NetPBMEncoder {
            enc_type: enc_type,
            writer: writer,
        }
    }

    pub fn encode(&mut self,
                  image: &[u8],
                  width: u32,
                  height: u32) -> io::Result<()>
    {
        // Calculate pixel color type.
        let pixel_type = match self.enc_type {
            NetPBM::BitMapAscii | NetPBM::BitMapBinary   => ColorType::BitMap,
            NetPBM::GrayMapAscii | NetPBM::GrayMapBinary => ColorType::Gray,
            NetPBM::PixMapAscii | NetPBM::PixMapBinary   => ColorType::Rgb,
        };
        self.__encode(image, width, height, pixel_type)
    }

    fn __encode(&mut self,
                image: &[u8],
                width: u32,
                height: u32,
                pixel_type: ColorType) -> io::Result<()>
    {
        try!(self.write_magic_number());
        try!(self.write_header(width, height, pixel_type));

        self.write_image(image, width, height, pixel_type)
    }

    fn write_magic_number(&mut self) -> io::Result<()> {
        write!(self.writer, "{}\n", self.enc_type.magic_number())
    }

    fn write_header(&mut self, width: u32, height: u32, pixel_type: ColorType) -> io::Result<()> {
        let max_val = pixel_type.max_pixel_value();
        write!(self.writer, "{} {}\n{}\n", width, height, max_val)
    }

    fn write_image(&mut self,
                   image: &[u8],
                   width: u32,
                   height: u32,
                   pixel_type: ColorType) -> io::Result<()> 
    {
        assert!(image.len() > 0);
        match pixel_type {
            ColorType::BitMap => {
                for line in image.chunks(width as usize) {
                    for pixel in line {
                        if *pixel == 0 {
                            try!(write!(self.writer, "0 "));
                        } else {
                            try!(write!(self.writer, "1 "));
                        }
                    }
                    try!(write!(self.writer, "\n"));
                }
            }
            ColorType::Gray => {
                for i in 0..height as usize {
                    for j in 0..width as usize {
                        try!(write!(self.writer, "{} ", image[(width as usize) * i + j]));
                    }
                    try!(self.writer.write_all("\n".as_bytes()));
                }
            }
            ColorType::Rgb => {
                for line in image.chunks(3 * width as usize) {
                    for pixel in line.chunks(3) {
                        try!(write!(self.writer, "{} {} {} ", pixel[0], pixel[1], pixel[2]));
                    }
                    try!(write!(self.writer, "\n"));
                }
            }
        }

        Ok(())
    }
}
