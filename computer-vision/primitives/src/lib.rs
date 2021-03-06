use std::path;
pub fn load_image<P: AsRef<path::Path>>(path: P) -> Result<Box<dyn common::Image<PixelFormat = u32>>, common::Error> {
    if let Some(ext) = path.as_ref().extension() {
        if ext == "jpg" || ext == "jpeg" {
            let image = jpeg::JpegImage::read(path)?;
            return Ok(image);
        }
    }

    Err(common::Error::invalid_extension(&path))
}