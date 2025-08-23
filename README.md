# `ssh-util`

## Run processes

```rs
// Create an SSH connection to a remote host
let mut session = Session::russh()
    .user("root")
    .host("localhost")
    .port(22)
    .password("password")
    .key_file()
    .connect()
    .await?;

// Run a command similar to `tokio::process::Command`
let mut child = session.command("echo")
    .arg("hello")
    .arg("world")
    .spawn()?;

// Await completion of the command
let status = child.wait().await?;
println!("the command exited with: {}", status);
```

## Transfer files

```rs
```