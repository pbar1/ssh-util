use ssh_util::Driver;
use ssh_util::Session;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _session = Session::builder()
        .driver(Driver::Russh)
        .user("pierce")
        .host("localhost")
        .port(22)
        .build();

    Ok(())
}
