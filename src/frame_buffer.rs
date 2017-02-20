use color::Rgb;
use std::ops;
use std::marker::PhantomData;


/// Return an initialized heap allocated frame buffer.
pub fn frame_buffer(width: usize, height: usize) -> Box<FrameBuffer<TopLeft>> {
    let mut frame_buffer = Box::new(FrameBuffer::new(width, height));
    (&mut (*frame_buffer)).initialize();

    frame_buffer
}

///
/// Marker trait for defining where in the buffer the origin is.
/// We must demarcate these because if the frame buffer data is
/// written to the frame buffer starting from the bottom left corner
/// of the screen, but the output device assumes the data in the 
/// buffer starts from the top left corner of the screen, the image will be
/// drawn upside down. 
///
pub trait Origin {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TopLeft {}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BottomLeft {}

impl Origin for TopLeft {}
impl Origin for BottomLeft {}

#[derive(PartialEq, Eq, Debug)]
pub struct FrameBuffer<Or: Origin> {
    width: usize,
    height: usize,
    buf: Vec<Vec<Rgb>>,
    _phantom: PhantomData<Or>,
}

impl<Or: Origin> FrameBuffer<Or> {
    pub fn new(width: usize, height: usize) -> FrameBuffer<Or> {
        let mut frame_buffer = FrameBuffer {
            width: width,
            height: height,
            buf: Vec::with_capacity(height),
            _phantom: PhantomData,
        };

        for _ in 0..frame_buffer.height {
            frame_buffer.buf.push(Vec::with_capacity(width));
        }

        frame_buffer
    }

    pub fn initialize(&mut self) {
        self.initialize_with(Rgb::from_channels(0,0,0));
    }

    pub fn initialize_with(&mut self, rgb: Rgb) {
        for row in 0..self.height {
            for _ in 0..self.width {
                self.buf[row].push(rgb);
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl FrameBuffer<BottomLeft> {
    pub fn dump_frame(&self, other_buf: &mut [u8]) -> Option<usize> {
        if other_buf.len() >= 3 * self.height * self.width {
            for i in 0..self.height {
                for j in 0..self.width {
                    other_buf[3*(self.width * i + j)]   = self.buf[i][j][0]; 
                    other_buf[3*(self.width * i + j)+1] = self.buf[i][j][1];
                    other_buf[3*(self.width * i + j)+2] = self.buf[i][j][2];
                }
            }

            Some(3 * self.height * self.width)
        } else {
            None
        }
    }

    pub fn lines(&self) -> RowIter<BottomLeft> {
        RowIter {
            index: 0,
            rows: &self.buf,
            _phantom: PhantomData,
        }
    }
}

impl FrameBuffer<TopLeft> {
    pub fn dump_frame(&self, other_buf: &mut [u8]) -> Option<usize> {
        if other_buf.len() >= 3 * self.height * self.width {
            for i in 0..self.height {
                for j in 0..self.width {
                    other_buf[3*(self.width * i + j)]   = self.buf[i][j][0]; 
                    other_buf[3*(self.width * i + j)+1] = self.buf[i][j][1];
                    other_buf[3*(self.width * i + j)+2] = self.buf[i][j][2];
                }
            }

            Some(3 * self.height * self.width)
        } else {
            None
        }
    }

    pub fn lines(&self) -> RowIter<TopLeft> {
        RowIter {
            index: self.height,
            rows: &self.buf,
            _phantom: PhantomData,
        }
    }
}

pub struct RowIter<'a, Or: Origin> {
    index: usize,
    rows: &'a [Vec<Rgb>],
    _phantom: PhantomData<Or>,
}

// Iteration for frame buffers whose origins starts from 
// the bottom left.
impl<'a> Iterator for RowIter<'a, BottomLeft> {
    type Item = &'a [Rgb];

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index < self.rows.len() {
            Some(&self.rows[self.index])
        } else {
            None
        }
    }
}

impl ops::Index<usize> for FrameBuffer<BottomLeft> {
    type Output = [Rgb];

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    } 
}

impl<'a> ops::Index<usize> for &'a FrameBuffer<BottomLeft> {
    type Output = [Rgb];

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    } 
}

impl ops::IndexMut<usize> for FrameBuffer<BottomLeft> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}

// Iteration for Frame Buffers whose origins start from the top
// left corner of the screen.
impl<'a> Iterator for RowIter<'a, TopLeft> {
    type Item = &'a [Rgb];

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index < self.rows.len() {
            Some(&self.rows[self.rows.len() - self.index - 1])
        } else {
            None
        }
    }
}

impl ops::Index<usize> for FrameBuffer<TopLeft> {
    type Output = [Rgb];

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[self.height - index - 1]
    } 
}

impl<'a> ops::Index<usize> for &'a FrameBuffer<TopLeft> {
    type Output = [Rgb];

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[self.height - index - 1]
    } 
}

impl ops::IndexMut<usize> for FrameBuffer<TopLeft> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[self.height - index - 1]
    }
}

impl PartialEq<FrameBuffer<TopLeft>> for FrameBuffer<BottomLeft> {
    fn eq(&self, other: &FrameBuffer<TopLeft>) -> bool {
        if self.shape() != other.shape() {
            return false; 
        }

        for (top_left_row, bottom_left_row) in self.lines().zip(other.lines()) {
            if top_left_row != bottom_left_row {
                return false;
            }
        }

        true
    }
}

impl PartialEq<FrameBuffer<BottomLeft>> for FrameBuffer<TopLeft> {
    fn eq(&self, other: &FrameBuffer<BottomLeft>) -> bool {
        if self.shape() != other.shape() {
            return false;
        }

        for (top_left_row, bottom_left_row) in self.lines().zip(other.lines()) {
            if top_left_row != bottom_left_row {
                return false;
            }
        }

        true
    }
}

impl<'a> PartialEq<&'a FrameBuffer<TopLeft>> for FrameBuffer<BottomLeft> {
    fn eq(&self, other: &&'a FrameBuffer<TopLeft>) -> bool {
        if self.shape() != other.shape() {
            return false;
        }

        for (top_left_row, bottom_left_row) in self.lines().zip(other.lines()) {
            if top_left_row != bottom_left_row {
                return false;
            }
        }

        true
    }
}

impl<'a> PartialEq<&'a FrameBuffer<BottomLeft>> for FrameBuffer<TopLeft> {
    fn eq(&self, other: &&'a FrameBuffer<BottomLeft>) -> bool {
        if self.shape() != other.shape() {
            return false;
        }

        for (top_left_row, bottom_left_row) in self.lines().zip(other.lines()) {
            if top_left_row != bottom_left_row {
                return false;
            }
        }

        true
    }
}

impl<'a> PartialEq<FrameBuffer<TopLeft>> for &'a FrameBuffer<BottomLeft> {
    fn eq(&self, other: &FrameBuffer<TopLeft>) -> bool {
        if self.shape() != other.shape() {
            return false;
        }

        for (top_left_row, bottom_left_row) in self.lines().zip(other.lines()) {
            if top_left_row != bottom_left_row {
                return false;
            }
        }

        true
    }
}

impl<'a> PartialEq<FrameBuffer<BottomLeft>> for &'a FrameBuffer<TopLeft> {
    fn eq(&self, other: &FrameBuffer<BottomLeft>) -> bool {
        if self.shape() != other.shape() {
            return false;
        }

        for (top_left_row, bottom_left_row) in self.lines().zip(other.lines()) {
            if top_left_row != bottom_left_row {
                return false;
            }
        }

        true
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

        for line in buf.lines() {
            assert_eq!(line.len(), buf.width());
        }
    }

    #[test]
    fn test_frame_buffer_should_be_zero_after_initialization() {
        let width  = 128;
        let height = 128;
        let buf  = super::frame_buffer(width, height);
        let zero = Rgb::from_channels(0,0,0);

        for line in buf.lines() {
            for pixel in line {
                assert_eq!(pixel, &zero);
            }
        }
    }

    #[test]
    fn test_dump_frame() {
        let width  = 512;
        let height = 512;
        let mut buf  = super::frame_buffer(width, height);
        let color = Rgb::from_channels(80,90,100);
        
        for i in 0..buf.height() {
            for j in 0..buf.width() {
                buf[i][j] = color;
            }
        }

        let mut dump_buf = Vec::with_capacity(3 * width * height);

        buf.dump_frame(&mut dump_buf);

        for chunk in dump_buf.chunks(3) {
            let rgb = Rgb::from_channels(chunk[0], chunk[1], chunk[2]);

            assert_eq!(rgb, color);
        }
    }

    #[test]
    fn test_topleft_frame_buffer_should_not_panic_during_iteration() {
        let width  = 128;
        let height = 128;
        let mut buf  = super::frame_buffer(width, height);
        let color = Rgb::from_channels(80,90,100);
        
        for i in 0..buf.height() {
            for j in 0..buf.width() {
                buf[i][j] = color;
            }
        }

        for line in buf.lines() {
            // No panics should occur.
            assert!(true);
        }
    }

    #[test]
    fn test_topleft_frame_buffer_and_bottomright_frame_buffer_should_be_identical_up_to_ordering_of_rows() {
        let width = 128;
        let height = 128;

        // Write a test pattern into the frame buffer that's just the 
        // row number repeated across the line.
        let mut top_left = super::frame_buffer(width, height);
        for (i, line) in top_left.lines().enumerate() {
            let new_rgb = Rgb::from_channels(i as u8, i as u8, i as u8);
            for ref mut old_rgb in line {
                *old_rgb = &new_rgb
            }
        }

        let mut bottom_left = super::FrameBuffer::<super::BottomLeft>::new(width, height);
        bottom_left.initialize();
        for (i, line) in bottom_left.lines().enumerate() {
            let new_rgb = Rgb::from_channels(i as u8, i as u8, i as u8);
            for ref mut old_rgb in line {
                *old_rgb = &new_rgb;
            }
        }

        // TopLeft and BottomLeft type should be equal with the rows flipped.
        assert_eq!(&*top_left, bottom_left);
    }
}
