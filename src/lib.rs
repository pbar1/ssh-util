#![warn(clippy::pedantic)]

pub struct Session {}

pub mod process {
    pub struct Command {}

    pub struct Child {}

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
