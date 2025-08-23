#![warn(clippy::pedantic)]

use bon::Builder;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use secrecy::SecretSlice;
use secrecy::SecretString;
use thiserror::Error;

#[cfg(feature = "libssh2")]
mod libssh2;
#[cfg(test)]
mod mock;
#[cfg(feature = "openssh")]
mod openssh;
#[cfg(feature = "russh")]
mod russh;

/// Alias for [`std::result::Result`] with this crate's [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can be encountered when using this crate.
#[derive(Debug, Error)]
pub enum Error {
    #[error("File does not exist: {0}")]
    FileNotFound(Utf8PathBuf),
    #[error("Unable to read file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("Failed reading environment variable: {0}")]
    EnvVar(#[from] std::env::VarError),
}

/// Underlying SSH implementation to use.
#[derive(Debug)]
pub enum Driver {
    /// Dummy driver used for testing.
    #[cfg(test)]
    Mock,
    /// Rust bindings to the [libssh2](https://libssh2.org/) C library.
    #[cfg(feature = "libssh2")]
    Libssh2,
    /// Shell out to the [OpenSSH](https://www.openssh.com/) binary.
    #[cfg(feature = "openssh")]
    OpenSsh,
    /// Pure Rust [russh](https://github.com/Eugeny/russh) library.
    #[cfg(feature = "russh")]
    Russh,
}

/// SSH authentication payloads.
#[derive(Debug)]
pub enum Auth {
    Password(SecretString),
    Key {
        private_key: SecretSlice<u8>,
        passphrase: Option<SecretString>,
    },
    Certificate {
        private_key: SecretSlice<u8>,
        passphrase: Option<SecretString>,
        certificate: Vec<u8>,
    },
    Agent {
        path: Utf8PathBuf,
    },
}

impl Auth {
    /// Sources password from file.
    ///
    /// # Errors
    ///
    /// - If `password_file` cannot be read.
    pub fn from_password_file(password_file: impl AsRef<Utf8Path>) -> Result<Auth> {
        let password = read_secret_string(password_file.as_ref())?;

        Ok(Auth::Password(password))
    }

    /// Sources SSH private key from file.
    ///
    /// # Errors
    ///
    /// - If `private_key_file` cannot be read.
    pub fn from_key_file(
        private_key_file: impl AsRef<Utf8Path>,
        passphrase: Option<impl AsRef<str>>,
    ) -> Result<Auth> {
        let private_key = read_secret_bytes(private_key_file.as_ref())?;

        let passphrase = passphrase.map(|s| SecretString::from(s.as_ref()));

        Ok(Auth::Key {
            private_key,
            passphrase,
        })
    }

    /// Sources SSH certificate and private key from files.
    ///
    /// # Errors
    ///
    /// - If `private_key` cannot be read.
    /// - If `certificate_file` cannot be read.
    pub fn from_cert_file(
        private_key_file: impl AsRef<Utf8Path>,
        passphrase: Option<impl AsRef<str>>,
        certificate_file: impl AsRef<Utf8Path>,
    ) -> Result<Auth> {
        let private_key = read_secret_bytes(private_key_file.as_ref())?;

        let passphrase = passphrase.map(|s| SecretString::from(s.as_ref()));

        let certificate = read_bytes(certificate_file.as_ref())?;

        Ok(Auth::Certificate {
            private_key,
            passphrase,
            certificate,
        })
    }

    /// Sources SSH agent to connect to from `SSH_AUTH_SOCK` environment
    /// variable.
    ///
    /// # Errors
    ///
    /// - If `SSH_AUTH_SOCK` environment variable is nonexistent or unreadable.
    /// - If the path value sourced from `SSH_AUTH_SOCK` does not exist.
    pub fn from_agent_env() -> Result<Auth> {
        let path = std::env::var("SSH_AUTH_SOCK")
            .map(Utf8PathBuf::from)
            .map_err(Error::from)?;

        if !path.try_exists().map_err(Error::from)? {
            return Err(Error::FileNotFound(path));
        }

        Ok(Self::Agent { path })
    }
}

/// SSH session.
#[derive(Debug, Builder)]
pub struct Session {
    #[builder(field)]
    auth: Vec<Auth>,
    /// Remote user to login as.
    #[builder(into)]
    user: String,
    /// Remote host to connect to.
    #[builder(into)]
    host: String,
    /// Port to connect to on the remote host.
    #[builder(default = 22)]
    port: u16,
    /// Underlying SSH implementation.
    driver: Driver,
}

impl<S: session_builder::State> SessionBuilder<S> {
    /// Payload that will be used for authentication attempts. Will be called
    /// in order until authentication succeeds; any remaining payloads will not
    /// be used.
    pub fn auth(mut self, value: Auth) -> Self {
        self.auth.push(value);
        self
    }
}

impl Session {
    #[cfg(test)]
    pub fn mock() -> SessionBuilder<session_builder::SetDriver> {
        Session::builder().driver(Driver::Mock)
    }

    #[cfg(feature = "libssh2")]
    pub fn libssh2() -> SessionBuilder<session_builder::SetDriver> {
        Session::builder().driver(Driver::Libssh2)
    }

    #[cfg(feature = "openssh")]
    pub fn openssh() -> SessionBuilder<session_builder::SetDriver> {
        Session::builder().driver(Driver::OpenSsh)
    }

    #[cfg(feature = "russh")]
    pub fn russh() -> SessionBuilder<session_builder::SetDriver> {
        Session::builder().driver(Driver::Russh)
    }
}

pub mod process {
    pub struct Command {}

    pub struct Child {
        pub stdin: Option<ChildStdin>,
        pub stdout: Option<ChildStdout>,
        pub stderr: Option<ChildStderr>,
    }

    pub struct ChildStdin {}

    pub struct ChildStdout {}

    pub struct ChildStderr {}
}

pub mod fs {
    pub struct DirBuilder {}

    pub struct DirEntry {}

    pub struct File {}

    pub struct OpenOptions {}

    pub struct ReadDir {}
}

fn read_bytes(path: &Utf8Path) -> Result<Vec<u8>> {
    if !path.try_exists().map_err(Error::from)? {
        return Err(Error::FileNotFound(path.to_owned()));
    }

    let value = std::fs::read(path).map_err(Error::from)?;

    Ok(value)
}

fn read_secret_string(path: &Utf8Path) -> Result<SecretString> {
    if !path.try_exists().map_err(Error::from)? {
        return Err(Error::FileNotFound(path.to_owned()));
    }

    let secret = std::fs::read_to_string(path)
        .map(SecretString::from)
        .map_err(Error::from)?;

    Ok(secret)
}

fn read_secret_bytes(path: &Utf8Path) -> Result<SecretSlice<u8>> {
    if !path.try_exists().map_err(Error::from)? {
        return Err(Error::FileNotFound(path.to_owned()));
    }

    let secret = std::fs::read(path)
        .map(SecretSlice::from)
        .map_err(Error::from)?;

    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_builder() {
        let session = Session::mock()
            .user("root")
            .host("localhost")
            .port(22)
            .auth(Auth::Password("password".into()))
            .build();
        dbg!(session);
    }
}
