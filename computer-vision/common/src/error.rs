#[derive(Debug)]
pub enum Error {
    UnsupportedExtension(String),
    IOError(std::io::Error),
}

impl Error {
    pub fn invalid_extension<P: AsRef<std::path::Path>>(path: P) -> Self {
        let ext = path
            .as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_owned())
            .unwrap_or_else(|| "<none>".to_owned());

        Error::UnsupportedExtension(ext)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedExtension(ext) => write!(f, "Unsupported Extension: {}", ext),
            Self::IOError(err) => writeln!(f, "IOError : {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(unit: std::io::Error) -> Self {
        Error::IOError(unit)
    }
}
