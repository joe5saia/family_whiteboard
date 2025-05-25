use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;
use serde_json::json;

use crate::database::Database;
use crate::models::{Todo, CreateTodoRequest, UpdateTodoRequest, TodosGroupedByDate};
use crate::websocket::broadcast_todo_update;

pub async fn get_todos(
    Extension(db): Extension<Arc<Database>>,
) -> Result<Json<Vec<TodosGroupedByDate>>, StatusCode> {
    match db.get_todos_grouped_by_date().await {
        Ok(todos) => Ok(Json(todos)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_todo(
    Extension(db): Extension<Arc<Database>>,
    Json(request): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    // Basic validation
    if request.text.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    match db.create_todo(request).await {
        Ok(todo) => {
            // Broadcast the new todo to all connected clients
            broadcast_todo_update("todo_created", json!(todo));
            Ok(Json(todo))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_todo(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    // Validate text if provided
    if let Some(ref text) = request.text {
        if text.trim().is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
    
    match db.update_todo(id, request).await {
        Ok(Some(todo)) => {
            broadcast_todo_update("todo_updated", json!(todo));
            Ok(Json(todo))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn toggle_todo(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, StatusCode> {
    match db.toggle_todo(id).await {
        Ok(Some(todo)) => {
            broadcast_todo_update("todo_toggled", json!(todo));
            Ok(Json(todo))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_todo(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match db.delete_todo(id).await {
        Ok(true) => {
            broadcast_todo_update("todo_deleted", json!({"id": id}));
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}