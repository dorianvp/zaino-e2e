use testcontainers::{ContainerAsync, GenericImage};

pub mod zainod;
pub mod zcashd;
pub mod zebrad;

pub type DynContainer<'a> = ContainerAsync<GenericImage>;

#[cfg(test)]
mod e2e_tests {
    use testcontainers::bollard::{Docker, network::CreateNetworkOptions};

    use crate::e2e::{zainod, zcashd::start_zcashd, zebrad::start_zebrad};

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

        let zebrad_container = match zebrad {
            Ok(zebrad) => zebrad,
            Err(err) => panic!("Error: {}", err),
        };

        let ip = zebrad_container.get_bridge_ip_address().await.unwrap();

        let zainod = zainod::start_zaino(&ip).await;

        match zainod {
            Ok(zainod) => println!("Zaino container id: {}", zainod.id()),
            Err(err) => panic!("Error: {}", err),
        }
    }
}
