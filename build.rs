use std::{io::Error, process::Command};

fn main() {
    // println!("cargo:rerun-if-changed=build.rs");

    match get_docker_version() {
        Ok(v) => {
            println!("cargo:warning=Docker version: {}", v);
        }
        Err(e) => {
            println!("Docker Error: {}", e);
        }
    }

    match get_docker_compose_version() {
        Ok(v) => {
            println!("cargo:warning=Docker Compose version: {}", v);
        }
        Err(e) => {
            println!("cargo:error=Docker Compose Error: {}", e);
        }
    }
}

fn get_docker_version() -> Result<String, String> {
    let output = Command::new("docker")
        .arg("--version")
        .output()
        .expect("failed to execute process");

    let version = String::from_utf8_lossy(&output.stdout);
    Ok(version.to_string())
}

fn get_docker_compose_version() -> Result<String, Error> {
    let output = Command::new("docker-compose").arg("--version").output()?;

    let version = String::from_utf8_lossy(&output.stdout);
    Ok(version.to_string())
}
