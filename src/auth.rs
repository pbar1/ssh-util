use std::env;
use std::fs;
use std::io;

use camino::Utf8Path;
use camino::Utf8PathBuf;
use secrecy::SecretSlice;
use secrecy::SecretString;
use ssh_key::Certificate;
use ssh_key::PrivateKey;

use crate::Error;
use crate::Result;

/// SSH authentication payloads.
#[derive(Debug)]
pub enum Auth {
    Password(SecretString),
    Key {
        private_key: PrivateKey,
    },
    Cert {
        certificate: Certificate,
        private_key: PrivateKey,
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
        let password = read_secret_string(password_file)?;

        Ok(Auth::Password(password))
    }

    /// Sources SSH private key from file.
    ///
    /// # Errors
    ///
    /// - If `private_key_file` cannot be read.
    pub fn from_key_file(
        private_key_file: impl AsRef<Utf8Path>,
        passphrase: Option<impl AsRef<[u8]>>,
    ) -> Result<Auth> {
        let private_key = read_openssh_private_key(private_key_file, passphrase)?;

        Ok(Auth::Key { private_key })
    }

    /// Sources SSH certificate and private key from files.
    ///
    /// # Errors
    ///
    /// - If `certificate_file` cannot be read.
    /// - If `private_key_file` cannot be read.
    pub fn from_cert_file(
        certificate_file: impl AsRef<Utf8Path>,
        private_key_file: impl AsRef<Utf8Path>,
        passphrase: Option<impl AsRef<[u8]>>,
    ) -> Result<Auth> {
        let certificate_file = certificate_file.as_ref().as_std_path();
        let certificate = Certificate::read_file(certificate_file)?;

        let private_key = read_openssh_private_key(private_key_file, passphrase)?;

        Ok(Auth::Cert {
            certificate,
            private_key,
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
        let path = env::var("SSH_AUTH_SOCK").map(Utf8PathBuf::from)?;

        if !path.try_exists()? {
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }

        Ok(Self::Agent { path })
    }
}

fn _read_secret_bytes(path: impl AsRef<Utf8Path>) -> Result<SecretSlice<u8>> {
    let secret = fs::read(path.as_ref()).map(SecretSlice::from)?;

    Ok(secret)
}

fn read_secret_string(path: impl AsRef<Utf8Path>) -> Result<SecretString> {
    let secret = fs::read_to_string(path.as_ref()).map(SecretString::from)?;

    Ok(secret)
}

fn read_openssh_private_key(
    private_key_file: impl AsRef<Utf8Path>,
    passphrase: Option<impl AsRef<[u8]>>,
) -> Result<PrivateKey> {
    let private_key_file = private_key_file.as_ref().as_std_path();
    let private_key = PrivateKey::read_openssh_file(private_key_file)?;

    let private_key = match (private_key.is_encrypted(), passphrase) {
        (true, Some(passphrase)) => private_key.decrypt(passphrase)?,
        (true, None) => {
            return Err(Error::EncryptedPrivateKeyNoPasshrase);
        }
        (false, _) => private_key,
    };

    Ok(private_key)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use secrecy::ExposeSecret;
    use ssh_key::HashAlg;

    use super::*;

    #[rstest]
    #[case("test/password", "test_password")]
    fn from_password_file_works(#[case] file: &str, #[case] password_should: &str) {
        let auth = Auth::from_password_file(file).unwrap();
        match auth {
            Auth::Password(got) => assert_eq!(got.expose_secret(), password_should),
            other => panic!("Got wrong Auth type: {other:?}"),
        };
    }

    #[rstest]
    #[case(
        "test/id_rsa",
        None,
        "SHA256:GJ0nE5DC04QqMlXKqNmbUqxpOWCSUZmbTMck+TlwGVM"
    )]
    #[case(
        "test/id_ecdsa",
        None,
        "SHA256:38PO2EwrjSu2AL8EymRDow4cbQveqNZIkvob8hbvYh8"
    )]
    #[case(
        "test/id_ed25519",
        None,
        "SHA256:qqVUhwuqHFgBv4R85QmdFIsKWkacxZ/MeB9oSXDbC7k"
    )]
    #[case(
        "test/enc_ed25519",
        Some("test_passphrase"),
        "SHA256:B/3vyYVgh7+kd7RuXKEC7zXvgegxUsVkHtNH+HC8XOM"
    )]
    fn from_key_file_works(
        #[case] private_key_file: &str,
        #[case] passphrase: Option<&str>,
        #[case] fingerprint_should: &str,
    ) {
        let auth = Auth::from_key_file(private_key_file, passphrase).unwrap();
        match auth {
            Auth::Key { private_key } => {
                assert_eq!(
                    private_key.fingerprint(HashAlg::Sha256).to_string(),
                    fingerprint_should
                );
            }
            other => panic!("Got wrong Auth type: {other:?}"),
        };
    }

    #[rstest]
    #[case(
        "test/id_ed25519",
        None,
        "SHA256:qqVUhwuqHFgBv4R85QmdFIsKWkacxZ/MeB9oSXDbC7k"
    )]
    #[case(
        "test/enc_ed25519",
        Some("test_passphrase"),
        "SHA256:B/3vyYVgh7+kd7RuXKEC7zXvgegxUsVkHtNH+HC8XOM"
    )]
    fn form_cert_file_works(
        #[case] private_key_file: &str,
        #[case] passphrase: Option<&str>,
        #[case] fingerprint_should: &str,
    ) {
        let certificate_file = format!("{private_key_file}-cert.pub");
        let auth = Auth::from_cert_file(certificate_file, private_key_file, passphrase).unwrap();
        match auth {
            Auth::Cert {
                certificate,
                private_key,
            } => {
                assert_eq!(certificate.serial(), 0);
                assert_eq!(certificate.key_id(), "test_identity");
                assert_eq!(certificate.valid_principals()[0], "test_user");
                assert_eq!(
                    private_key.fingerprint(HashAlg::Sha256).to_string(),
                    fingerprint_should
                );
            }
            other => panic!("Got wrong Auth type: {other:?}"),
        };
    }
}
