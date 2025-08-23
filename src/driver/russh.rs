use std::sync::Arc;

use russh::client::Handle;

use crate::Result;

pub(crate) struct SessionImpl {
    handle: Handle<ClientHandler>,
}

pub async fn connect(host: &str) -> Result<()> {
    let config = Arc::new(russh::client::Config::default());

    let session = russh::client::connect(config, host, ClientHandler).await?;

    Ok(())
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
