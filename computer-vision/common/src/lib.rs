mod error;
mod matrix;
mod numerics;
pub use error::Error;
pub use matrix::Matrix;
pub use numerics::NumericType;

pub trait Image {
    type PixelFormat;

    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn get_matrix(&self) -> &matrix::Matrix<Self::PixelFormat> where <Self as Image>::PixelFormat : NumericType;
}
