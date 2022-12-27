use axum::{response::IntoResponse, Json};
use serde::Serialize;
use utoipa::ToSchema;

#[utoipa::path(
        get,
        path = "/health",
        responses((
            status = 200,
            description = "Health response (ok)",
            body = Health,
            content_type = "application/json",
        )),
    )]
pub async fn get_health() -> impl IntoResponse {
    Json(Health {
        status: "ok".to_string(),
    })
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"status": "ok"}))]
pub struct Health {
    pub status: String,
}

#[utoipa::path(get,
    path = "/random",
    responses((
        status = 200,
        description = "Random response",
        body = Random,
        content_type = "application/json",
    )),
    )]
pub async fn get_random() -> impl IntoResponse {
    Json(Random {
        random: rand::random::<i32>(),
    })
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"random": 283727386}))]
pub struct Random {
    pub random: i32,
}
