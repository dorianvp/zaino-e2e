mod e2e;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use testcontainers::ImageExt;
    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        time::timeout,
    };

    #[tokio::test]
    async fn starts_zcashd() {
        use testcontainers::{GenericImage, runners::AsyncRunner};

        let zcashd_image = GenericImage::new("zcashd", "test")
            .with_env_var("ZCASHD_NETWORK", "regtest")
            .with_env_var("ENABLE_COOKIE_AUTH", "false")
            .with_env_var(
                "ZCASHD_ACK_FLAG",
                "-i-am-aware-zcashd-will-be-replaced-by-zebrad-and-zallet-in-2025=1",
            );

        let container = match zcashd_image.start().await {
            Ok(container) => {
                println!("Container id: {}", container.id());
                container
            }
            Err(err) => {
                panic!("Error: {}", err)
            }
        };

        let logs = container.stdout(true);

        let mut reader = BufReader::new(logs).lines();

        println!("Container logs (press Ctrl+C to interrupt):");

        let log_scan = async {
            while let Ok(Some(line)) = reader.next_line().await {
                println!("{}", line);
                if line.contains("Done loading") {
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
                panic!("Received Ctrl+C. Cleaning up...");
            }
        }
    }

    #[tokio::test]
    async fn starts_zebrad() {
        use testcontainers::{GenericImage, runners::AsyncRunner};
        use tokio::io::AsyncBufReadExt;
        use tokio::io::BufReader;

        let zebrad_image = GenericImage::new("zebrad", "test")
            .with_env_var("NETWORK", "Regtest")
            .with_env_var("ENABLE_COOKIE_AUTH", "false");

        let container = match zebrad_image.start().await {
            Ok(container) => {
                println!("Container id: {}", container.id());
                container
            }
            Err(err) => {
                panic!("Error: {}", err)
            }
        };

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
                panic!("Received Ctrl+C. Cleaning up...");
            }
        }
    }

    #[tokio::test]
    async fn starts_disconnected_zaino() {
        use testcontainers::{GenericImage, runners::AsyncRunner};
        use tokio::io::AsyncBufReadExt;
        use tokio::io::BufReader;

        let zainod_image = GenericImage::new("zainod", "test");

        let container = match zainod_image.start().await {
            Ok(container) => {
                println!("Container id: {}", container.id());
                container
            }
            Err(err) => {
                panic!("Error: {}", err)
            }
        };

        let logs = container.stdout(true);

        let mut reader = BufReader::new(logs).lines();

        println!("Container logs (press Ctrl+C to interrupt):");

        let log_scan = async {
            while let Ok(Some(line)) = reader.next_line().await {
                println!("{}", line);
                if line.contains("Error: Could not establish connection with node.") {
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
                panic!("Received Ctrl+C. Cleaning up...");
            }
        }
    }
}
