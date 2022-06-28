pub mod access;
pub mod buffers;
pub mod textures;

use textures::TextureInfo;

trait Buffer<T: TextureInfo> {
    fn read(&mut self) -> &T;

    fn reader_drop(&mut self);

    fn write(&mut self) -> &mut T;

    fn writer_drop(&mut self);
}
