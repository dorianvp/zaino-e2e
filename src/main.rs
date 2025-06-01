mod e2e;

fn main() {
    println!("Hello, world!");
}

mod tests {
    use std::time::Duration;

    use testcontainers::ImageExt;
    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        time,
    };

    #[tokio::test]
    async fn runs_zcashd() {
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

        tokio::select! {
            _ = async {
                while let Ok(Some(line)) = reader.next_line().await {
                    println!("{}", line);
                    if line.contains("Done loading") {
                        break;
                    }
                }
            } => {
                println!("Container started and log message found.");
            }

            _ = tokio::signal::ctrl_c() => {
                println!("Received Ctrl+C. Cleaning up and exiting...");
                container.stop().await.unwrap();

            }
            _ = time::sleep(Duration::from_secs(10)) => {
                println!("Reached timeout. Cleaning up and exiting...");
                container.stop().await.unwrap();
                panic!("Timed out waiting for zcashd to be ready.");
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

        tokio::select! {
            _ = async {
                while let Ok(Some(line)) = reader.next_line().await {
                    println!("{}", line);
                    if line.contains("Zebra is close to the tip tip_height=Height(0)") {
                        break;
                    }
                }
            } => {
                println!("Container started and log message found.");
            }

            _ = tokio::signal::ctrl_c() => {
                println!("Received Ctrl+C. Cleaning up and exiting...");
                container.stop().await.unwrap();

            }
            _ = time::sleep(Duration::from_secs(10)) => {
                println!("Reached timeout. Cleaning up and exiting...");
                container.stop().await.unwrap();
                panic!("Timed out waiting for zcashd to be ready.");
            }
        }
    }
}
