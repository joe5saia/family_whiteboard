use anyhow::Result;
use chrono::NaiveDate;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

use crate::models::{Todo, CreateTodoRequest, UpdateTodoRequest, TodosGroupedByDate};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost/family_todo".to_string());
        
        let pool = PgPool::connect(&database_url).await?;
        
        Ok(Self { pool })
    }
    
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
    
    pub async fn create_todo(&self, request: CreateTodoRequest) -> Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todos (text, assignee, due_date)
            VALUES ($1, $2, $3)
            RETURNING id, text, assignee, due_date, completed, created_at, updated_at
            "#
        )
        .bind(&request.text)
        .bind(&request.assignee)
        .bind(request.due_date)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(todo)
    }
    
    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, text, assignee, due_date, completed, created_at, updated_at FROM todos ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(todos)
    }
    
    pub async fn get_todos_grouped_by_date(&self) -> Result<Vec<TodosGroupedByDate>> {
        let todos = self.get_todos().await?;
        let mut grouped: HashMap<String, Vec<Todo>> = HashMap::new();
        
        for todo in todos {
            let date_key = match todo.due_date {
                Some(date) => date.to_string(),
                None => "No Due Date".to_string(),
            };
            
            grouped.entry(date_key).or_default().push(todo);
        }
        
        // Sort each group's todos
        for todos in grouped.values_mut() {
            todos.sort_by(|a, b| {
                match (a.completed, b.completed) {
                    (true, false) => std::cmp::Ordering::Greater,
                    (false, true) => std::cmp::Ordering::Less,
                    _ => a.id.cmp(&b.id),
                }
            });
        }
        
        // Convert to sorted vector
        let mut result: Vec<TodosGroupedByDate> = grouped
            .into_iter()
            .map(|(date, todos)| TodosGroupedByDate { date, todos })
            .collect();
        
        // Sort date groups
        result.sort_by(|a, b| {
            match (a.date.as_str(), b.date.as_str()) {
                ("No Due Date", "No Due Date") => std::cmp::Ordering::Equal,
                ("No Due Date", _) => std::cmp::Ordering::Less,
                (_, "No Due Date") => std::cmp::Ordering::Greater,
                (date_a, date_b) => date_a.cmp(date_b),
            }
        });
        
        Ok(result)
    }
    
    pub async fn get_todo_by_id(&self, id: i32) -> Result<Option<Todo>> {
        let todo = sqlx::query_as::<_, Todo>(
            "SELECT id, text, assignee, due_date, completed, created_at, updated_at FROM todos WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(todo)
    }
    
    pub async fn update_todo(&self, id: i32, request: UpdateTodoRequest) -> Result<Option<Todo>> {
        // First get the current todo
        let current_todo = match self.get_todo_by_id(id).await? {
            Some(todo) => todo,
            None => return Ok(None),
        };
        
        // Use current values as defaults
        let text = request.text.unwrap_or(current_todo.text);
        let assignee = request.assignee.unwrap_or(current_todo.assignee);
        let due_date = request.due_date.or(current_todo.due_date);
        let completed = request.completed.unwrap_or(current_todo.completed);
        
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos 
            SET text = $1, assignee = $2, due_date = $3, completed = $4, updated_at = NOW()
            WHERE id = $5
            RETURNING id, text, assignee, due_date, completed, created_at, updated_at
            "#
        )
        .bind(&text)
        .bind(&assignee)
        .bind(due_date)
        .bind(completed)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Some(todo))
    }
    
    pub async fn toggle_todo(&self, id: i32) -> Result<Option<Todo>> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos 
            SET completed = NOT completed, updated_at = NOW()
            WHERE id = $1
            RETURNING id, text, assignee, due_date, completed, created_at, updated_at
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(todo)
    }
    
    pub async fn delete_todo(&self, id: i32) -> Result<bool> {
        let result = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }
}