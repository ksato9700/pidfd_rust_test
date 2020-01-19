use pidfd::PidFd;
use std::{io, process::Command};

fn main() {
    futures::executor::block_on(async move {
        futures::try_join!(
            spawn_sleeper("1", "5"),
            spawn_sleeper("2", "4"),
            spawn_sleeper("3", "3"),
            spawn_sleeper("4", "2"),
            spawn_sleeper("5", "1"),
        )
        .unwrap();
    })
}

async fn spawn_sleeper(id: &str, timeout: &str) -> io::Result<()> {
    println!("started job {}", id);

    let exit_status = Command::new("/bin/sleep")
        .arg(timeout)
        .spawn()
        .map(PidFd::from)
        .unwrap()
        .into_future()
        .await?;

    println!("finished job {}: {}", id, exit_status);
    Ok(())
}
