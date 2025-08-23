use std::net::SocketAddr;
use std::time::Duration;

use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;

use crate::Error;
use crate::Result;

/// Factory for building reliable async bytestream that will be used for the
/// SSH protocol.
pub trait Transport {
    /// Connects the transport, returning a duplex bytestream ready for use.
    async fn connect(&self, addr: SocketAddr) -> Result<impl AsyncRead + AsyncWrite>;
}

#[derive(Debug)]
pub struct TokioTcp {
    /// Connect timeout
    timeout: Duration,
}

impl Transport for TokioTcp {
    async fn connect(&self, addr: SocketAddr) -> Result<impl AsyncRead + AsyncWrite> {
        let result = tokio::time::timeout(self.timeout, tokio::net::TcpStream::connect(addr))
            .await
            .map_err(|_| Error::ConnectTimeout)?;

        let stream = result?;

        Ok(stream)
    }
}
