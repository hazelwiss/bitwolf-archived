use super::Canvas;
use util::colour::BGRA;

pub struct TextBGRA<const WIDTH: usize, const HEIGHT: usize> {
    pub text: [[util::colour::BGRA; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Canvas for TextBGRA<WIDTH, HEIGHT> {
    const HEIGHT: usize = HEIGHT;
    const WIDTH: usize = WIDTH;

    fn empty() -> Self {
        Self {
            text: [[BGRA(0xFF, 0xFF, 0xFF, 0xFF); WIDTH]; HEIGHT],
        }
    }

    fn pitch(&self) -> usize {
        WIDTH * std::mem::size_of::<BGRA>()
    }
}
