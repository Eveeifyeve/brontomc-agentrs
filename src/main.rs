use std::{process::exit, sync::Arc};

use bollard::Docker;
use redb::Database;
use tokio::net::TcpListener;

mod routes;

#[derive(Clone)]
pub struct AppState {
    pub docker: Docker,
    pub db: Arc<Database>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let docker = Docker::connect_with_local_defaults().unwrap();
    let db = Database::open("brontomc-agentrs.db").unwrap();

    if docker.ping().await.is_err() {
        tracing::error!("Bollard docker daemon failed!");
        exit(1)
    }

    let app_state = AppState {
        docker: docker,
        db: Arc::new(db),
    };

    // build our application with a route
    let app = routes::main(app_state);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
