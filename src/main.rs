use bollard::Docker;
use redb::Database;

mod docker;
mod routes;

#[derive(Clone, Copy)]
pub struct AppState {
    pub docker: Docker,
    pub db: Database,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let docker = docker::init_docker();
    let db = Database::create("brontomc.redb").unwrap();

    let app_state = AppState {
        docker: docker.await.unwrap(),
        db,
    };

    // build our application with a route
    let app = routes::main(app_state);

    // run our app with hyper, listening globally on port 3000
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    tracing::debug!("listening on {}", addr);
}
