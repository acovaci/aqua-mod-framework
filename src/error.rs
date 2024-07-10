#[derive(thiserror::Error, Debug)]
pub enum AquaError {
    #[error("Unknown error")]
    Unknown,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Template error: {0}")]
    Template(#[from] minijinja::Error),
}

pub type Res<T> = Result<T, AquaError>;
