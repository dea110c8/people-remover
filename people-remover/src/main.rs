use std::{env, path};

fn main() {
    let inputs = env::args()
        .skip(1)
        .map(path::PathBuf::from)
        .filter(|path| path.exists())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();

    if inputs.is_empty() {
        eprintln!("Not enough valid inputs to continue.");
    }

    let inputs = inputs
        .iter()
        .map(|path| primitives::load_image(path))
        .collect::<Result<Vec<_>, _>>();

    if let Err(error) = inputs {
        eprintln!("Failed to load one of the input images: {}", error);
        return;
    }

    // transform the inputs so that we can ensure they are similar
    let thumbnails = inputs.unwrap().iter().map(|image| {
        let dim = image.width().min(image.height());
        dim
    });
}
