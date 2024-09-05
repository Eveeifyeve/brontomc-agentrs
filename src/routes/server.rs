use axum::{extract::State, response::IntoResponse, Json};
use bollard::{container::Config, image::CreateImageOptions};
use futures_util::TryStreamExt;
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

    pub fn image(platform: &str) -> Result<String, String> {
        match platform {
            "vanilla" => Ok(String::from("itzg/minecraft-server:java17")),
            _ => Err("Invalid server platform".to_string()),
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
    let DOCKER_IMAGE: String =
        ServerPlatforms::image(&input.server_platfrom).expect("Expected image");
    match server_platform {
        Ok(server_platform) => match server_platform {
            ServerPlatforms::Vanilla => {
                let docker = state.docker.clone();
                let mc_version = format!("VERSION={}", input.mc_version);

                let minecraft_server = Config {
                    image: Some(DOCKER_IMAGE.as_str()),
                    cmd: Some(vec!["/etc/confluent/docker/run"]),
                    env: Some(vec!["EULA=TRUE", &mc_version]),
                    ..Default::default()
                };

                docker
                    .create_image(
                        Some(CreateImageOptions {
                            from_image: DOCKER_IMAGE.clone(),
                            ..Default::default()
                        }),
                        None,
                        None,
                    )
                    .try_collect::<Vec<_>>()
                    .await;

                let id = docker
                    .create_container::<&str, &str>(None, minecraft_server)
                    .await
                    .unwrap()
                    .id;
                docker.start_container::<String>(&id, None).await;

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
