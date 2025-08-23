use std::net::SocketAddr;

use crate::Result;

pub mod tokio_tcp;

pub trait TransportFactory {
    async fn connect(&self, addr: SocketAddr) -> Result<Transport>;
}

#[derive(Debug)]
pub enum Transport {
    None,
    TokioTcp(tokio::net::TcpStream),
}
