use nalgebra::BaseFloat;


/// Return an initialized heap-allocated z-buffer.
pub fn z_buffer<N: Copy + BaseFloat>(width: usize, height: usize) -> Box<ZBuffer<N>> {
    let mut z_buffer = Box::new(ZBuffer::new(width, height));
    z_buffer.initialize();

    z_buffer
}

/// Use a floating point ZBuffer for right now.
/// TODO: Convert to an integer Z-Buffer.
pub struct ZBuffer<N> {
    width: usize,
    height: usize,
    buf: Vec<Vec<N>>,
}

impl<N> ZBuffer<N> where N: BaseFloat {
    pub fn new(width: usize, height: usize) -> ZBuffer<N> {
        let mut z_buffer = ZBuffer {
            width: width,
            height: height,
            buf: Vec::with_capacity(height)
        };

        for _ in 0..z_buffer.height {
            z_buffer.buf.push(Vec::with_capacity(width));
        }

        let zero = N::zero();

        for i in 0..z_buffer.height {
            for _ in 0..z_buffer.width {
                z_buffer.buf[i].push(zero);
            }
        }

        z_buffer
    }

    pub fn initialize(&mut self) {
        let inf = N::infinity();

        for i in 0..self.height {
            for j in 0..self.width {
                self.buf[i][j] = inf;
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn lines(&self) -> ZBufferLineIter<N> {
        ZBufferLineIter {
            index: 0,
            lines: &self.buf
        }
    }
}

struct ZBufferLineIter<'a, N: 'a> {
    index: usize,
    lines: &'a [Vec<N>],
}

impl<'a, N: 'a> Iterator for ZBufferLineIter<'a, N> {
    type Item = &'a [N];

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index < self.lines.len() {
            Some(&self.lines[self.index])
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_z_buffer_should_correctly_report_dimensions() {
        let width  = 512;
        let height = 512;
        let buf = super::z_buffer::<f32>(width, height);

        assert_eq!(buf.width(), width);
        assert_eq!(buf.height(), height);

        for line in buf.lines() {
            assert_eq!(line.len(), buf.width());
        }
    }

    #[test]
    fn test_z_buffer_should_have_only_infinite_values_after_initialization() {
        use num_traits::Float;

        let width  = 512;
        let height = 512;
        let buf = super::z_buffer::<f32>(width, height);
        let inf: f32 = Float::infinity();

        for line in buf.lines() {
            for pixel in line {
                assert_eq!(pixel, &inf);
            }
        }
    }
}
