use ssh_util::Auth;
use ssh_util::Session;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _session = Session::russh()
        .user("pierce")
        .host("localhost")
        .port(22)
        .auth(Auth::from_agent_env())
        .build();

    Ok(())
}
