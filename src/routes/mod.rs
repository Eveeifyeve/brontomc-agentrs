use axum::Router;

pub fn main(state: crate::AppState) -> Router<crate::AppState> {
    Router::new()
        .route("/", axum::routing::get(root))
        .with_state(state)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
