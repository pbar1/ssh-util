use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("SSH key error: {0}")]
    Key(#[from] ssh_key::Error),

    #[error("Encrypted private key requires passphrase to be used")]
    EncryptedPrivateKeyNoPasshrase,

    #[cfg(feature = "russh")]
    #[error("Russh library error: {0}")]
    Russh(#[from] ::russh::Error),

    #[error("Connect timed out")]
    ConnectTimeout,
}
