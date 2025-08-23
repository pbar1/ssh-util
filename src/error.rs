use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File does not exist: {0}")]
    FileNotFound(camino::Utf8PathBuf),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed reading environment variable: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[cfg(feature = "russh")]
    #[error("Russh error: {0}")]
    Russh(#[from] ::russh::Error),
    #[error("Connect timed out")]
    ConnectTimeout,
}
