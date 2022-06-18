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

    let inputs = inputs.iter().map(|path| primitives::load_image(path));
}
