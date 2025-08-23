use std::net::SocketAddr;
use std::time::Duration;

use bon::Builder;
use tokio::net::TcpSocket;

use super::Transport;
use super::TransportFactory;
use crate::Result;

pub type SocketModifier = dyn Fn(&TcpSocket) -> std::io::Result<()>;

#[derive(Builder)]
pub struct TokioTcp {
    timeout: Duration,
    modifier: Option<Box<SocketModifier>>,
}

impl TransportFactory for TokioTcp {
    async fn connect(&self, addr: SocketAddr) -> Result<Transport> {
        let socket = match addr {
            SocketAddr::V4(_) => TcpSocket::new_v4(),
            SocketAddr::V6(_) => TcpSocket::new_v6(),
        }
        .unwrap();

        if let Some(modifier) = &self.modifier {
            modifier(&socket).unwrap();
        }

        let stream = tokio::time::timeout(self.timeout, socket.connect(addr))
            .await
            .unwrap()
            .unwrap();

        Ok(Transport::TokioTcp(stream))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[tokio::test]
    async fn tokio_tcp_works() {
        let factory = TokioTcp::builder()
            .timeout(Duration::from_secs(1))
            .modifier(Box::new(|socket| socket.set_reuseport(true)))
            .build();

        let addr = SocketAddr::from_str("127.0.0.1:22").unwrap();

        let _stream = factory.connect(addr).await.unwrap();
    }
}
