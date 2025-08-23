#![warn(clippy::pedantic)]

#[cfg(feature = "libssh2")]
mod libssh2;
#[cfg(feature = "openssh")]
mod openssh;
#[cfg(feature = "russh")]
mod russh;

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
