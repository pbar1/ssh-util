use std::net::SocketAddr;
use std::sync::Arc;

use bon::Builder;
use russh::client::Handle;
use secrecy::ExposeSecret;

use crate::Auth;
use crate::Result;
use crate::driver::Driver;
use crate::driver::Session;
use crate::transport::Transport;
use crate::transport::TransportFactory;

#[derive(Builder)]
pub struct RusshDriver<T: TransportFactory> {
    #[builder(field)]
    auth: Vec<Auth>,

    #[builder(into)]
    user: String,
    addr: SocketAddr,
    transport_factory: T,
}

impl<T: TransportFactory> Driver for RusshDriver<T> {
    type Session = RusshSession;

    async fn connect(self) -> Result<Self::Session> {
        let transport = self.transport_factory.connect(self.addr).await.unwrap();

        let config = Arc::new(russh::client::Config::default());

        let handle = match transport {
            Transport::None => panic!(),
            Transport::TokioTcp(tcp_stream) => {
                russh::client::connect_stream(config, tcp_stream, ClientHandler)
                    .await
                    .unwrap()
            }
        };

        Ok(RusshSession {
            handle,
            user: self.user,
            auth: self.auth,
        })
    }
}

pub struct RusshSession {
    handle: Handle<ClientHandler>,
    user: String,
    auth: Vec<Auth>,
}

impl Session for RusshSession {
    async fn authenticate(&mut self) -> Result<()> {
        for payload in &self.auth {
            let auth_result = match payload {
                Auth::Password(password) => self
                    .handle
                    .authenticate_password(&self.user, password.expose_secret())
                    .await
                    .unwrap(),
                _ => todo!(),
            };

            if auth_result.success() {
                return Ok(());
            }
        }

        todo!()
    }

    fn command(&self) -> crate::process::Command {
        todo!()
    }
}

struct ClientHandler;

impl russh::client::Handler for ClientHandler {
    type Error = russh::Error;

    // FIXME: Verify server key
    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::PublicKey,
    ) -> std::result::Result<bool, Self::Error> {
        Ok(true)
    }
}
