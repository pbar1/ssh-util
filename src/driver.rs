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
