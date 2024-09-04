use bollard::Docker;
use std::process::exit;

pub async fn init_docker() -> Result<Docker, bool> {
    let bollard = Docker::connect_with_local_defaults().unwrap();

    if bollard.ping().await.is_err() {
        tracing::error!("Bollard docker daemon failed!");
        exit(1)
    }

    Ok(bollard)
}
