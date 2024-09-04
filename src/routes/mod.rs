use axum::{routing::post, Router};
use server::create_server;
mod server;

pub fn main(state: crate::AppState) -> Router {
    Router::new()
        .with_state(state.clone())
        .nest("/server", server(state.clone()))
}

fn server(state: crate::AppState) -> Router {
    Router::new()
        .route("/create", post(create_server))
        .with_state(state)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
