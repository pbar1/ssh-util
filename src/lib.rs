#![warn(clippy::pedantic)]

use bon::Builder;

mod auth;
mod driver;
mod error;
mod transport;

pub use auth::Auth;
pub use driver::DriverKind;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

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
    driver: DriverKind,
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
    fn test_session_builder() {}
}
