use std::ops::{Index, IndexMut};

pub struct CanvasBuffer<E> {
    buffer: Vec<E>,
    size_x: u16,
    size_y: u16,
}
impl<E> Index<(u16, u16)> for CanvasBuffer<E> {
    type Output = E;
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        let idx = index.1 as usize * self.size_x as usize + index.0 as usize;
        &self.buffer[idx]
    }
}

impl<E> IndexMut<(u16, u16)> for CanvasBuffer<E> {
    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        let idx = index.1 as usize * self.size_x as usize + index.0 as usize;
        &mut self.buffer[idx]
    }
}

impl<E> CanvasBuffer<E>
where
    E: Copy,
{
    fn new(size_x: u16, size_y: u16, blank: E) -> CanvasBuffer<E> {
        let size = size_x as usize * size_y as usize;
        CanvasBuffer {
            buffer: vec![blank; size],
            size_x,
            size_y,
        }
    }

    pub fn get_dimensions(&self) -> (u16, u16) {
        (self.size_x, self.size_y)
    }
}

pub struct Canvas<E> {
    blank: E,
    buffer: CanvasBuffer<E>,
}

impl<E> Canvas<E>
where
    E: Copy,
{
    pub fn new(size_x: u16, size_y: u16, blank: E) -> Canvas<E> {
        let buffer = CanvasBuffer::new(size_x, size_y, blank);
        Canvas { blank, buffer }
    }

    pub fn frame(&mut self, e: E) {
        let (size_x, size_y) = self.buffer.get_dimensions();
        for x in 0..size_x {
            self.buffer[(x, 0)] = e;
            self.buffer[(x, size_y - 1)] = e;
        }
        for y in 1..size_y - 1 {
            self.buffer[(0, y)] = e;
            self.buffer[(size_x - 1, y)] = e;
        }
    }

    pub fn clear(&mut self) {
        let (size_x, size_y) = self.buffer.get_dimensions();
        for y in 0..size_y {
            for x in 0..size_x {
                self.buffer[(x, y)] = self.blank;
            }
        }
    }

    pub fn draw(&mut self, x: u16, y: u16, snake: E) {
        self.buffer[(x, y)] = snake;
    }

    pub fn get_buffer(&self) -> &CanvasBuffer<E> {
        &self.buffer
    }
}
