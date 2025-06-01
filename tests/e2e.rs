mod zainod;
mod zcashd;
mod zebrad;

#[cfg(test)]
pub mod test_utils {
    use std::{sync::Arc, time::Duration};

    use testcontainers::{ContainerAsync, GenericImage};
    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        time::timeout,
    };

    pub async fn wait_for_log(
        container: Arc<ContainerAsync<GenericImage>>,
        matching_line: &str,
        timeout_in_seconds: u64,
    ) -> Result<String, String> {
        let reader = BufReader::new(container.stdout(true));
        let mut lines = reader.lines();
        let log_scan = async {
            while let Ok(Some(line)) = lines.next_line().await {
                if line.contains(matching_line) {
                    return Ok(());
                }
            }
            Err("Log stream ended before expected message")
        };

        let select_result = tokio::select! {
            result = timeout(Duration::from_secs(timeout_in_seconds), log_scan) => {
                match result {
                    Ok(Ok(())) => Ok("Log message found."),
                    Ok(Err(e)) => Err(e),
                    Err(_) => Err("Timeout while waiting for log message."),
                }
            }

            _ = tokio::signal::ctrl_c() => {
                Err("Ctrl-C detected.")
            }
        };

        if let Err(e) = select_result {
            Err(e.to_string())
        } else {
            Ok("Log message found.".to_string())
        }
    }
}

#[cfg(test)]
mod e2e_tests {

    use testcontainers::bollard::{Docker, network::CreateNetworkOptions};

    use crate::{test_utils::wait_for_log, zainod::start_zaino, zebrad::start_zebrad};

    #[tokio::test]
    async fn integration_test_with_zcashd_and_zebrad() {
        let docker = Docker::connect_with_local_defaults().unwrap();

        let _ = docker
            .create_network(CreateNetworkOptions {
                name: "zingo-net",
                check_duplicate: true,
                driver: "bridge",
                ..Default::default()
            })
            .await;

        // let zcashd = start_zcashd().await;
        // let zcashd_host = zcashd.get_hostname(); // or just "zcashd" if using DNS names

        let zebrad = start_zebrad().await;
        assert!(zebrad.is_ok(), "Zebra did not start");

        match wait_for_log(
            zebrad.clone().unwrap(),
            "Zebra is close to the tip tip_height=Height(0)",
            20,
        )
        .await
        {
            Err(e) => panic!("Error: {}", e),
            Ok(_) => {}
        }

        let ip = zebrad.unwrap().get_bridge_ip_address().await.unwrap();

        let zainod = start_zaino(&ip).await;
        assert!(zainod.is_ok());

        match wait_for_log(
            zainod.clone().unwrap(),
            "Zaino Indexer started successfully.",
            20,
        )
        .await
        {
            Err(e) => panic!("Error: {}", e),
            Ok(_) => {}
        }
    }
}
