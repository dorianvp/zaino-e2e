use std::time::Duration;

use testcontainers::core::IntoContainerPort;
use testcontainers::{GenericImage, ImageExt, runners::AsyncRunner};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::timeout,
};

use super::DynContainer;

pub async fn start_zebrad<'a>() -> Result<DynContainer<'a>, String> {
    let image = GenericImage::new("zebrad", "test")
        .with_env_var("NETWORK", "Regtest")
        .with_env_var("ENABLE_COOKIE_AUTH", "false")
        .with_env_var("ZEBRA_RPC_PORT", "8237")
        .with_mapped_port(8237, 8237.tcp())
        .with_network("zingo-net")
        .with_container_name("zebrad");
    // .with_env_var("ZEBRAD_CONNECT", &format!("http://{}:8232", zcashd_host))

    let container = image.start().await.unwrap();

    let logs = container.stdout(true);

    let mut reader = BufReader::new(logs).lines();

    println!("Container logs (press Ctrl+C to interrupt):");

    let log_scan = async {
        while let Ok(Some(line)) = reader.next_line().await {
            println!("{}", line);
            if line.contains("Zebra is close to the tip tip_height=Height(0)") {
                return Ok(());
            }
        }
        Err("Log stream ended before expected message")
    };

    tokio::select! {
        result = timeout(Duration::from_secs(10), log_scan) => {
            match result {
                Ok(Ok(())) => println!("✅ Log message found."),
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
