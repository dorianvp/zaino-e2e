use std::{net::IpAddr, sync::Arc};

use testcontainers::{GenericImage, ImageExt, core::Mount, runners::AsyncRunner};
use tokio::io::{AsyncBufReadExt, BufReader};
use zaino_e2e::test_helpers::DynContainer;

pub async fn start_zaino<'a>(
    validator_ip: &IpAddr,
) -> Result<Arc<DynContainer<'a>>, (String, String)> {
    let project_dir = std::fs::canonicalize(".").unwrap(); // resolves to repo root
    let config_dir = project_dir.join("config");
    let data_dir = project_dir.join("zaino-data");

    let generated_config = generate_zindexer_config(validator_ip.to_string().as_str());

    let tempdir = tempfile::TempDir::new().unwrap();
    let config_path = tempdir.path().join("zindexer.toml");

    std::fs::write(&config_path, generated_config).unwrap();

    std::fs::create_dir_all(&data_dir).unwrap();

    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&data_dir, std::fs::Permissions::from_mode(0o777)).unwrap();

    let image = GenericImage::new("zainod", "test")
        // TODO: Zaino does not support configuring the validator URL through env vars
        // .with_env_var("ZCASH_RPC_URL", format!("{}:8237", validator_ip))
        .with_cmd(["zainod", "--config", "/config/zindexer.toml"])
        .with_network("zingo-net")
        .with_mount(Mount::bind_mount(
            tempdir.path().to_str().unwrap(),
            "/config",
        ))
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

// TODO: Remove. Zaino should support configuring the validator URL through env vars
fn generate_zindexer_config(validator_ip: &str) -> String {
    ZAINO_CONFIG_TEMPLATE.replace(
        "${VALIDATOR_LISTEN_ADDRESS}",
        &format!("{validator_ip}:8237"),
    )
}

const ZAINO_CONFIG_TEMPLATE: &str = r#"
# Configuration for Zaino

# Backend:
backend = "fetch"

# JsonRPC server config:
enable_json_server = false
json_rpc_listen_address = "localhost:8232"
enable_cookie_auth = false
cookie_dir = "None"

# gRPC server config:
grpc_listen_address = "localhost:8137"
grpc_tls = false
tls_cert_path = "None"
tls_key_path = "None"

# JsonRPC client config:
validator_listen_address = "${VALIDATOR_LISTEN_ADDRESS}"
validator_cookie_auth = true
validator_cookie_path = "/home/zaino/.cookie"
validator_user = "xxxxxx"
validator_password = "xxxxxx"

# Mempool, Non-Finalised State and Finalised State config:
map_capacity = "None"
map_shard_amount = "None"
zaino_db_path = "/home/zaino/.cache/zaino/"
zebra_db_path = "/home/zaino/.cache/zebra/"
db_size = "None"

# Network:
network = "Regtest"

# Options:
no_sync = false
no_db = false
slow_sync = false
"#;
