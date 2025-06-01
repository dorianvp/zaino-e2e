use std::time::Duration;

use testcontainers::{GenericImage, ImageExt, runners::AsyncRunner};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::timeout,
};
use zaino_e2e::test_helpers::DynContainer;

pub async fn start_zcashd<'a>() -> DynContainer<'a> {
    let image = GenericImage::new("zcashd", "test")
        .with_env_var("ZCASHD_NETWORK", "regtest")
        .with_env_var(
            "ZCASHD_ACK_FLAG",
            "-i-am-aware-zcashd-will-be-replaced-by-zebrad-and-zallet-in-2025=1",
        )
        .with_env_var("ENABLE_COOKIE_AUTH", "false");

    let container = image.start().await.unwrap();
    let logs = container.stdout(true);
    let mut reader = BufReader::new(logs).lines();

    timeout(Duration::from_secs(30), async {
        while let Ok(Some(line)) = reader.next_line().await {
            println!("[zcashd] {}", line);
            if line.contains("Done loading") {
                break;
            }
        }
    })
    .await
    .expect("zcashd startup timed out");

    container
}
