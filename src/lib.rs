#![warn(clippy::pedantic)]

use bon::Builder;
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
    fn auth(mut self, value: Auth) -> Self {
        self.auth.push(value);
        self
    }
}

impl Session {}

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
