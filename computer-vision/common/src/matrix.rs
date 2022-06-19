use crate::numerics::{NumericType, ToUsize};

#[derive(PartialEq, Debug)]
pub struct Matrix<T: NumericType> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: NumericType> Matrix<T> {

    pub fn new<I : ToUsize>(width: I, height: I) -> Self {
        let width : usize = width.to_usize();
        let height : usize = height.to_usize();

        Self { data: vec![T::zero(); width * height], width: width, height: height }
    }

    /// Generates a square matrix of `size` by `size` that is an identity matrix.
    pub fn identity(size: usize) -> Self {
        let mut values = vec![T::zero(); size * size];

        for index in (0..size * size).step_by(size + 1) {
            values[index] = T::one();
        }

        Self {
            data: values,
            width: size,
            height: size,
        }
    }

    /// Returns the matrix width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the matrix height
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn crop(&self, width: usize, height: usize) -> Self {
        assert!(width <= self.width);
        assert!(height <= self.height);

        // do this slow, but correctly.
        // Maybe the compiler will optimise this for us?

        let mut result = Vec::with_capacity(width * height);
        let mut iter = self.data.iter();

        for row in 0..height {
            if row == 0 {
                let r = iter.by_ref().take(width);
                result.extend(r);
            } else {
                let r = iter.by_ref().skip(self.width - width).take(width);
                result.extend(r);
            };
        }

        Self {
            data: result,
            width: width,
            height: height,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn test_ones() {
        let floats = Matrix::<f32>::identity(3);

        assert_eq!(
            floats.data,
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn test_crop() {
        let src = Matrix::<u8> {
            width: 10,
            height: 8,
            data: vec![
                00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62,
                63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
            ],
        };

        let cropped = src.crop(4, 4);

        let result = Matrix::<u8> {
            width: 4,
            height: 4,
            data: vec![0, 1, 2, 3, 10, 11, 12, 13, 20, 21, 22, 23, 30, 31, 32, 33],
        };

        assert_eq!(cropped, result);
    }
}
