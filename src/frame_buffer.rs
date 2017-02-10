use color::Rgb;
use std::ops;


/// Return an initialized heap allocated frame buffer.
pub fn frame_buffer(width: usize, height: usize) -> Box<FrameBuffer> {
    let mut frame_buffer = Box::new(FrameBuffer::new(width, height));
    (&mut (*frame_buffer)).initialize();

    frame_buffer
}

pub struct FrameBuffer {
    width: usize,
    height: usize,
    buf: Vec<Vec<Rgb>>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        let mut frame_buffer = FrameBuffer {
            width: width,
            height: height,
            buf: Vec::with_capacity(height)
        };

        for _ in 0..frame_buffer.height {
            frame_buffer.buf.push(Vec::with_capacity(width));
        }

        frame_buffer
    }

    pub fn initialize(&mut self) {
        for i in 0..self.height {
            for _ in 0..self.width {
                self.buf[i].push(Rgb::from_channels(0,0,0));
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn dump_frame(&self, other_buf: &mut [u8]) -> Option<usize> {
        if other_buf.len() >= 3 * self.height * self.width {
            for i in 0..self.height {
                for j in 0..self.width {
                    other_buf[self.width * i + j]   = self.buf[i][j][0]; 
                    other_buf[self.width * i + j+1] = self.buf[i][j][1];
                    other_buf[self.width * i + j+2] = self.buf[i][j][2];
                }
            }

            Some(3 * self.height * self.width)
        } else {
            None
        }
    }

    fn scanlines(&self) -> ScanlineIter {
        ScanlineIter {
            index: 0,
            lines: &self.buf
        }
    }
}

struct ScanlineIter<'a> {
    index: usize,
    lines: &'a [Vec<Rgb>],
}

impl<'a> Iterator for ScanlineIter<'a> {
    type Item = &'a [Rgb];

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index < self.lines.len() {
            Some(&self.lines[self.index])
        } else {
            None
        }
    }
}

impl ops::Index<usize> for FrameBuffer {
    type Output = Vec<Rgb>;

    fn index(&self, index: usize) -> &Vec<Rgb> {
        &self.buf[index]
    } 
}

impl<'a> ops::Index<usize> for &'a FrameBuffer {
    type Output = Vec<Rgb>;

    fn index(&self, index: usize) -> &Vec<Rgb> {
        &self.buf[index]
    } 
}

impl ops::IndexMut<usize> for FrameBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}

#[cfg(test)]
mod tests {
    use color::Rgb;


    #[test]
    fn test_frame_buffer_should_correctly_report_dimensions() {
        let width  = 128;
        let height = 128;
        let buf = super::frame_buffer(width, height);

        assert_eq!(buf.width(), width);
        assert_eq!(buf.height(), height);

        for line in buf.scanlines() {
            assert_eq!(line.len(), buf.width());
        }
    }

    #[test]
    fn test_frame_buffer_should_be_zero_after_initialization() {
        let width  = 512;
        let height = 512;
        let buf  = super::frame_buffer(width, height);
        let zero = Rgb::from_channels(0,0,0);

        for line in buf.scanlines() {
            for pixel in line {
                assert_eq!(pixel, &zero);
            }
        }
    }

    #[test]
    fn test_dump_frame() {
        let width  = 128;
        let height = 128;
        let mut buf  = super::frame_buffer(width, height);
        let red = Rgb::from_channels(255,0,0);
        
        for i in 0..buf.height() {
            for j in 0..buf.width() {
                buf[i][j] = red;
            }
        }

        let mut dump_buf = Vec::with_capacity(3 * width * height);

        buf.dump_frame(&mut dump_buf);

        for chunk in dump_buf.chunks(3) {
            let rgb = Rgb::from_channels(chunk[0], chunk[1], chunk[2]);

            assert_eq!(red, rgb);
        }
    }
}
