mod audio;
mod input;
mod video;

pub use audio::Audio;
pub use input::Input;
pub use video::Video;

pub(crate) type VideoInterface = Box<dyn Video + Send>;
pub(crate) type AudioInterface = Box<dyn Audio + Send>;
pub(crate) type InputInterface = Box<dyn Input + Send>;
