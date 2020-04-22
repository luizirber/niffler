use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Feature disabled, enabled it during compilation")]
    FeatureDisabled,

    #[error("File is too short, less than five bytes")]
    FileTooShort,

    #[error("I/O error")]
    IOError(#[from] std::io::Error),
}
