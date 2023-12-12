use std::sync::Arc;

use axum::{extract::State, Json, http::StatusCode};
use axum::extract::Path;
use serde_json::{json, Value};
use tokio::sync::Mutex;
use validator::Validate;

use crate::{config::app_state::AppState, dto::user_request_dto::UserRequestDto};

pub async fn find_all(
    State(app_state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    tracing::info!("Requesting to get all users.");
    let list_user = app_state.lock().await.user_service.find_all();

    (
        StatusCode::OK,
        Json(json!({
            "status": 200,
            "success": true,
            "data": list_user,
        }))
    )
}

pub async fn find_by_id(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    tracing::info!("Requesting to find user by id: {}", id);
    app_state
        .lock()
        .await
        .user_service
        .find_by_id(id.clone())
        .map_or_else(
            || {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "status": 404,
                        "success": false,
                        "message": format!("User with given id [{}] is not exist", id)
                    }))
                )
            },
            |user| {
                (StatusCode::OK, Json(json!({
                    "status": 200,
                    "success": false,
                    "data": user,
                })))
            }
        )
}

pub async fn create(
    app_state: State<Arc<Mutex<AppState>>>,
    Json(dto): Json<UserRequestDto>,
) -> (StatusCode, Json<Value>) {
    tracing::info!("request receivd: {:?}", dto);
    match dto.validate() {
        Ok(_) => {
            let created_id = app_state.lock().await.user_service.save(dto);
            (
                StatusCode::CREATED,
                Json(json!({
                    "success": true,
                    "status": 201,
                    "message": "User successfully created.",
                    "data": created_id.to_string(),
                })),
            )
        },
        Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "status": 400,
                    "message": "Validation fails",
                    "errors": e,
                }))
            )
        }
    }
}