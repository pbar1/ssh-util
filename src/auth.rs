use camino::Utf8Path;
use camino::Utf8PathBuf;
use secrecy::SecretSlice;
use secrecy::SecretString;

use crate::Error;
use crate::Result;

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
