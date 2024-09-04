use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

enum ServerPlatforms {
    Vanilla,
    Paper,
    Bukkit,
    Minestom,
    BungeeCord,
    Velocity,
}

impl ServerPlatforms {
    pub fn from_str(platform: &str) -> Result<Self, String> {
        match platform {
            "vanilla" => Ok(ServerPlatforms::Vanilla),
            "paper" => Ok(ServerPlatforms::Paper),
            "bukkit" => Ok(ServerPlatforms::Bukkit),
            "minestom" => Ok(ServerPlatforms::Minestom),
            "bungeecord" => Ok(ServerPlatforms::BungeeCord),
            "velocity" => Ok(ServerPlatforms::Velocity),
            _ => Err(format!("Invalid server platform: {}", platform)),
        }
    }
}

pub struct ServerInput {
    pub server_platfrom: String,
    pub mc_version: String,
}

pub async fn create_server(
    Json(input): Json<ServerInput>,
    State(state): State<crate::AppState>,
) -> impl IntoResponse {
    let server_platform = ServerPlatforms::from_str(&input.server_platfrom);
    let mc_version = input.mc_version;
    match server_platform {
        Ok(server_platform) => match server_platform {
            ServerPlatforms::Vanilla => {
                let response = json!({
                    "status": "success",
                    "message": "Server created successfully",
                    "data": {
                        "server_platform": "vanilla",
                        "mc_version": mc_version,
                    }
                });
                Json(response)
            }
            _ => todo!(),
        },
        Err(err) => {
            tracing::error!("Error parsing server platform: {}", err);
            let response = json!({
                "status": "error",
                "message": "Error parsing server platform",
                "data": err
            });

            Json(response)
        }
    }
}
