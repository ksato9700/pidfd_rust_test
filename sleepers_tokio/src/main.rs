use tokio::process::Command;
use futures::try_join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    try_join! (
        spawn_sleeper("1", "5"),
        spawn_sleeper("2", "4"),
        spawn_sleeper("3", "3"),
        spawn_sleeper("4", "2"),
        spawn_sleeper("5", "1"),
    ).unwrap();

    Ok(())
}

async fn spawn_sleeper(id: &str, timeout: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("started job {}", id);

    let exit_status = Command::new("/bin/sleep")
        .arg(timeout)
        .spawn()
        .unwrap()
        .await?;

    println!("finished job {}: {}", id, exit_status);

    Ok(())
}
