use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub assignee: String,
    pub due_date: Option<NaiveDate>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub text: String,
    pub assignee: String,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub text: Option<String>,
    pub assignee: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TodosGroupedByDate {
    pub date: String,
    pub todos: Vec<Todo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
}