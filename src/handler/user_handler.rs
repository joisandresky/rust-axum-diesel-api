use std::sync::Arc;

use axum::{extract::State, Json, http::StatusCode, Router};
use axum::extract::Path;
use axum::routing::{get, patch};
use serde_json::{json, Value};
use tokio::sync::Mutex;
use validator::Validate;

use crate::{config::app_state::AppState, dto::user_request_dto::UserRequestDto};

pub fn routes() -> Router<Arc<Mutex<AppState>>> {
    Router::new()
        .route("/api/v1/users/set-verified/:id", patch(set_verified_by_id))
        .route("/api/v1/users/:id", get(find_by_id).delete(delete_by_id))
        .route("/api/v1/users", get(find_all).post(create))
}

async fn find_all(
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

async fn find_by_id(
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
                    "success": true,
                    "data": user,
                })))
            }
        )
}

async fn create(
    app_state: State<Arc<Mutex<AppState>>>,
    Json(dto): Json<UserRequestDto>,
) -> (StatusCode, Json<Value>) {
    tracing::info!("request received: {:?}", dto);

    match dto.validate() {
        Ok(_) => {
            app_state.lock().await.user_service.save(dto)
                .map_or_else(|e| {
                    (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        Json(json!(e)),
                    )
                },
                |created_id| {
                    (
                        StatusCode::CREATED,
                        Json(json!({
                            "success": true,
                            "status": 201,
                            "message": "User successfully created.",
                            "data": created_id.to_string(),
                        })),
                    )
                })
        }
        Err(e) => {
            // Convert ServiceError to HTTP response
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "status": 400,
                    "message": e.to_string(),
                })),
            )
        }
    }
}

async fn set_verified_by_id(
    State(app_astate): State<Arc<Mutex<AppState>>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    tracing::info!("Requesting to set verified with id {}", id);

    app_astate
        .lock()
        .await
        .user_service
        .set_verified(id)
        .map_or_else(|e| {
            (
                StatusCode::NOT_FOUND,
                Json(json!(e))
            )
        }, |_result| {
            (
                StatusCode::OK,
                Json(json!({
                    "status": 200,
                    "success": true,
                }))
            )
        })
}

async fn delete_by_id(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    tracing::info!("Requesting to delete user by id: {}", id);

    app_state
        .lock()
        .await
        .user_service
        .delete_by_id(id.clone())
        .map_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": 404,
                    "success": false,
                    "message": format!("User with given id [{}] is not exist", id)
                }))
            )
        }, |_dto| {
            (
                StatusCode::OK,
                Json(json!({
                    "status": 200,
                    "success": true,
                }))
            )
        })
}