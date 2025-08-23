#![warn(clippy::pedantic)]

use bon::Builder;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use secrecy::SecretSlice;
use secrecy::SecretString;

#[cfg(feature = "libssh2")]
mod libssh2;
#[cfg(test)]
mod mock;
#[cfg(feature = "openssh")]
mod openssh;
#[cfg(feature = "russh")]
mod russh;

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
    pub fn from_password_file(password_file: impl AsRef<Utf8Path>) -> Auth {
        let password = std::fs::read_to_string(password_file.as_ref()).expect("todo");
        let password = SecretString::from(password);
        Auth::Password(password)
    }

    /// Sources SSH private key from file.
    pub fn from_key_file(
        private_key_file: impl AsRef<Utf8Path>,
        passphrase: Option<impl AsRef<str>>,
    ) -> Auth {
        let private_key = std::fs::read(private_key_file.as_ref()).expect("todo");
        let private_key = SecretSlice::from(private_key);
        let passphrase = passphrase.map(|s| SecretString::from(s.as_ref()));
        Auth::Key {
            private_key,
            passphrase,
        }
    }

    /// Sources SSH certificate and private key from files.
    pub fn from_cert_file(
        private_key_file: impl AsRef<Utf8Path>,
        passphrase: Option<impl AsRef<str>>,
        certificate_file: impl AsRef<Utf8Path>,
    ) -> Auth {
        let private_key = std::fs::read(private_key_file.as_ref()).expect("todo");
        let private_key = SecretSlice::from(private_key);
        let passphrase = passphrase.map(|s| SecretString::from(s.as_ref()));
        let certificate = std::fs::read(certificate_file.as_ref()).expect("todo");
        Auth::Certificate {
            private_key,
            passphrase,
            certificate,
        }
    }

    /// Sources SSH agent to connect to from `SSH_AUTH_SOCK` environment
    /// variable.
    pub fn from_agent_env() -> Auth {
        let path = std::env::var("SSH_AUTH_SOCK").expect("todo");
        let path = Utf8PathBuf::from(path);
        Auth::Agent { path }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_builder() {
        let session = Session::builder()
            .user("root")
            .host("localhost")
            .port(22)
            .auth(Auth::Password("password".into()))
            .driver(Driver::Mock)
            .build();
        dbg!(session);
    }
}
