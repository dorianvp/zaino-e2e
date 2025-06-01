use std::sync::Arc;

use testcontainers::core::IntoContainerPort;
use testcontainers::{GenericImage, ImageExt, runners::AsyncRunner};
use tokio::io::{AsyncBufReadExt, BufReader};
use zaino_e2e::test_helpers::DynContainer;

pub async fn start_zebrad<'a>() -> Result<Arc<DynContainer<'a>>, (String, String)> {
    let image = GenericImage::new("zebrad", "test")
        .with_env_var("NETWORK", "Regtest")
        .with_env_var("ENABLE_COOKIE_AUTH", "false")
        .with_env_var("ZEBRA_RPC_PORT", "8237")
        .with_mapped_port(8237, 8237.tcp())
        .with_network("zingo-net")
        .with_container_name("zebrad");

    let container = image.start().await.unwrap();
    let container = Arc::new(container);

    let cloned = Arc::clone(&container);

    let logs = container.stdout(true);

    let reader = BufReader::new(logs);

    let mut lines = reader.lines();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        cloned.stop().await.unwrap();
        println!("Received Ctrl+C. Cleaning up...");
    });

    tokio::spawn(async move {
        while let Ok(Some(line)) = lines.next_line().await {
            println!("{}", line);
        }
    });

    Ok(container)
}
