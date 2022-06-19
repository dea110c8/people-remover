use std::{ffi, io, mem, path};
use common::Matrix;
pub struct JpegImage {
    width: u32,
    height: u32,
    data: Matrix<u32>,
}

impl JpegImage {
    pub fn read<P: AsRef<path::Path>>(path: P) -> Result<Box<Self>, common::Error> {
        unsafe {
            let path = path
                .as_ref()
                .to_str()
                .map(|s| s.to_owned())
                .expect("Failed to convert input");
            let path = ffi::CString::new(path).expect("Failed to convert to native C String");
            let mode = ffi::CString::new("rb").unwrap(); // really shouldn't fail

            let stream = jpeg_sys::fopen(path.as_ptr(), mode.as_ptr());
            if stream.is_null() {
                // hopefully that covers us for failing to load...
                return Err(io::Error::last_os_error().into());
            }

            let mut scratch: jpeg_sys::jpeg_decompress_struct = mem::zeroed();

            jpeg_sys::jpeg_CreateDecompress(
                &mut scratch,
                jpeg_sys::JPEG_LIB_VERSION as i32,
                mem::size_of::<jpeg_sys::jpeg_decompress_struct>() as u64,
            );

            jpeg_sys::jpeg_stdio_src(&mut scratch, stream);
            jpeg_sys::jpeg_read_header(&mut scratch, jpeg_sys::TRUE as i32);
            jpeg_sys::jpeg_start_decompress(&mut scratch);

            let w = scratch.image_width;
            let h = scratch.image_height;

            let data = Matrix::new(w, h);

            todo!("finish reading file!");

            jpeg_sys::jpeg_finish_decompress(&mut scratch);
            jpeg_sys::jpeg_destroy_decompress(&mut scratch);
            jpeg_sys::fclose(stream);

            Ok(Box::new(Self {
                width: w,
                height: h,
                data: data,
            }))
        }
    }
}

impl common::Image for JpegImage {

    type PixelFormat = u32;

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn get_matrix(&self) -> &common::Matrix<Self::PixelFormat> where <Self as common::Image>::PixelFormat : common::NumericType {
        &self.data
    }
}
