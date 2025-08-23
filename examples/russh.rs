use ssh_util::Auth;
use ssh_util::Session;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let session = Session::russh()
        .user("pierce")
        .host("localhost")
        .port(22)
        .auth(Auth::from_agent_env()?)
        .auth(Auth::from_password_file("passwordfile")?)
        .build();

    dbg!(session);

    Ok(())
}
