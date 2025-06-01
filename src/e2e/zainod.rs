use std::{net::IpAddr, time::Duration};

use testcontainers::{GenericImage, ImageExt, core::Mount, runners::AsyncRunner};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::timeout,
};

use super::DynContainer;

pub async fn start_zaino<'a>(validator_ip: &IpAddr) -> Result<DynContainer<'a>, String> {
    let project_dir = std::fs::canonicalize(".").unwrap(); // resolves to repo root
    let config_dir = project_dir.join("config");
    let data_dir = project_dir.join("zaino-data");

    std::fs::create_dir_all(&data_dir).unwrap();

    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&data_dir, std::fs::Permissions::from_mode(0o777)).unwrap();

    let image = GenericImage::new("zainod", "test")
        .with_env_var("ZCASH_RPC_URL", format!("{}:8237", validator_ip))
        .with_cmd(["zainod", "--config", "/config/zindexer.toml"])
        .with_network("zingo-net")
        .with_mount(Mount::bind_mount(config_dir.to_str().unwrap(), "/config"))
        .with_mount(Mount::bind_mount(
            config_dir.join(".cookie").to_str().unwrap(),
            "/home/zaino/.cookie",
        ))
        .with_mount(Mount::bind_mount(
            data_dir.to_str().unwrap(),
            "/var/lib/zaino",
        ));

    println!("Starting Zaino...");
    let container = image.start().await.unwrap();
    println!("Zaino container id: {}", container.id());

    let logs = container.stdout(true);

    let mut reader = BufReader::new(logs).lines();

    println!("Container logs (press Ctrl+C to interrupt):");

    let log_scan = async {
        while let Ok(Some(line)) = reader.next_line().await {
            println!("{}", line);
            if line.contains("Zaino Indexer started successfully.") {
                return Ok(());
            }
        }
        Err("Log stream ended before expected message")
    };

    tokio::select! {
        result = timeout(Duration::from_secs(10), log_scan) => {
            match result {
                Ok(Ok(())) => println!("Log message found."),
                Ok(Err(e)) => panic!("Error: {e}"),
                Err(_) => panic!("Timeout while waiting for log message."),
            }
        }

        _ = tokio::signal::ctrl_c() => {
            container.stop().await.unwrap();
            return Err("Received Ctrl+C. Cleaning up...".to_string());
        }
    }

    Ok(container)
}
