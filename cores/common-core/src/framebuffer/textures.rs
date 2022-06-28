use util::colour::BGRA;

pub struct Texture<Col, const WIDTH: usize, const HEIGHT: usize> {
    pub text: [[Col; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Texture<BGRA, WIDTH, HEIGHT> {
    pub fn pitch(&self) -> usize {
        WIDTH * std::mem::size_of::<BGRA>()
    }

    pub const fn width(&self) -> usize {
        WIDTH
    }

    pub const fn height(&self) -> usize {
        HEIGHT
    }
}

pub trait TextureInfo: Default {
    const HEIGHT: usize;
    const WIDTH: usize;
}

impl<Col: Default + Copy, const WIDTH: usize, const HEIGHT: usize> Default
    for Texture<Col, WIDTH, HEIGHT>
{
    fn default() -> Self {
        Self {
            text: [[Default::default(); WIDTH]; HEIGHT],
        }
    }
}

impl<Col: Default + Copy, const WIDTH: usize, const HEIGHT: usize> TextureInfo
    for Texture<Col, WIDTH, HEIGHT>
{
    const HEIGHT: usize = WIDTH;
    const WIDTH: usize = HEIGHT;
}
