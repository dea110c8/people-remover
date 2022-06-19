mod error;
pub use error::Error;

pub trait Image {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
