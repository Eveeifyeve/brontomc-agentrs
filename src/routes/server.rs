use axum::{
    http::{response, StatusCode},
    Json,
};
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

pub struct ServerCreateResponse {
    pub status: String,
    pub message: String,
    pub data: ServerCreateResponseData,
}

pub enum ServerCreateResponseData {
    Normal(String),
    Data(ServerCreateResponseDataInner),
}

pub struct ServerCreateResponseDataInner {
    pub server_platform: String,
    pub mc_version: String,
}

pub async fn create_server(Json(input): Json<ServerInput>) -> Json<ServerCreateResponse> {
    let server_platform = ServerPlatforms::from_str(&input.server_platfrom);
    let mc_version = input.mc_version;
    match server_platform {
        Ok(server_platform) => match server_platform {
            ServerPlatforms::Vanilla => {
                let response = ServerCreateResponse {
                    status: "success".to_string(),
                    message: "Server created successfully".to_string(),
                    data: ServerCreateResponseData::Data(ServerCreateResponseDataInner {
                        server_platform: "vanilla".to_string(),
                        mc_version,
                    }),
                };
                Json(response)
            }
            _ => todo!(),
        },
        Err(err) => {
            tracing::error!("Error parsing server platform: {}", err);
            let response = ServerCreateResponse {
                status: "error".to_string(),
                message: "Error parsing server platform".to_string(),
                data: ServerCreateResponseData::Normal(err.to_string()),
            };

            Json(response)
        }
    }
}
