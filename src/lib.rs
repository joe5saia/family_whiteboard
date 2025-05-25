use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodoItem {
    id: u32,
    text: String,
    assignee: String,
    date: String,
    completed: bool,
}

#[wasm_bindgen]
impl TodoItem {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn assignee(&self) -> String {
        self.assignee.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn date(&self) -> String {
        self.date.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn completed(&self) -> bool {
        self.completed
    }
}

#[wasm_bindgen]
pub struct TodoApp {
    todos: Vec<TodoItem>,
    next_id: u32,
}

impl Default for TodoApp {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl TodoApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TodoApp {
        TodoApp {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    #[wasm_bindgen]
    pub fn add_todo(&mut self, text: &str, assignee: &str, date: &str) {
        let todo = TodoItem {
            id: self.next_id,
            text: text.to_string(),
            assignee: assignee.to_string(),
            date: date.to_string(),
            completed: false,
        };
        self.todos.push(todo);
        self.next_id += 1;
        self.sort_todos();
    }

    #[wasm_bindgen]
    pub fn toggle_todo(&mut self, id: u32) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
        self.sort_todos();
    }

    #[wasm_bindgen]
    pub fn get_todos_json(&self) -> String {
        serde_json::to_string(&self.todos).unwrap_or_else(|_| "[]".to_string())
    }

    #[wasm_bindgen]
    pub fn get_todos_grouped_by_date_json(&self) -> String {
        let mut grouped: HashMap<String, Vec<&TodoItem>> = HashMap::new();

        for todo in &self.todos {
            let date_key = if todo.date.is_empty() {
                "No Due Date".to_string()
            } else {
                todo.date.clone()
            };
            grouped.entry(date_key).or_default().push(todo);
        }

        // Sort each group by completion status and ID
        for todos in grouped.values_mut() {
            todos.sort_by(|a, b| match (a.completed, b.completed) {
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                _ => a.id.cmp(&b.id),
            });
        }

        // Convert to sorted date groups
        let mut date_groups: Vec<(String, Vec<TodoItem>)> = grouped
            .into_iter()
            .map(|(date, todos)| (date, todos.into_iter().cloned().collect()))
            .collect();

        // Sort dates: "No Due Date" first, then chronologically
        date_groups.sort_by(|a, b| match (a.0.as_str(), b.0.as_str()) {
            ("No Due Date", "No Due Date") => std::cmp::Ordering::Equal,
            ("No Due Date", _) => std::cmp::Ordering::Less,
            (_, "No Due Date") => std::cmp::Ordering::Greater,
            (date_a, date_b) => date_a.cmp(date_b),
        });

        serde_json::to_string(&date_groups).unwrap_or_else(|_| "[]".to_string())
    }

    #[wasm_bindgen]
    pub fn get_todo_count(&self) -> usize {
        self.todos.len()
    }

    #[wasm_bindgen]
    pub fn edit_todo(&mut self, id: u32, text: &str, assignee: &str, date: &str) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.text = text.to_string();
            todo.assignee = assignee.to_string();
            todo.date = date.to_string();
            self.sort_todos();
            true
        } else {
            false
        }
    }

    fn sort_todos(&mut self) {
        self.todos.sort_by(|a, b| match (a.completed, b.completed) {
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            _ => a.id.cmp(&b.id),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    fn test_todo_app_creation() {
        let app = TodoApp::new();
        assert_eq!(app.get_todo_count(), 0);
    }

    #[test]
    fn test_add_todo() {
        let mut app = TodoApp::new();
        app.add_todo("Test task", "Joe", "2024-01-01");
        assert_eq!(app.get_todo_count(), 1);
    }

    #[test]
    fn test_add_unassigned_todo() {
        let mut app = TodoApp::new();
        app.add_todo("Unassigned task", "Unassigned", "2024-01-01");
        assert_eq!(app.get_todo_count(), 1);

        let todos_json = app.get_todos_json();
        assert!(todos_json.contains("\"assignee\":\"Unassigned\""));
    }

    #[test]
    fn test_toggle_todo() {
        let mut app = TodoApp::new();
        app.add_todo("Test task", "Joe", "2024-01-01");
        app.toggle_todo(1);

        let todos_json = app.get_todos_json();
        assert!(todos_json.contains("\"completed\":true"));
    }

    #[test]
    fn test_todo_sorting() {
        let mut app = TodoApp::new();
        app.add_todo("Task 1", "Joe", "2024-01-01");
        app.add_todo("Task 2", "Shannon", "2024-01-02");

        app.toggle_todo(1);

        let todos_json = app.get_todos_json();
        let todos: Vec<TodoItem> = serde_json::from_str(&todos_json).unwrap();

        assert!(!todos[0].completed);
        assert!(todos[1].completed);
    }

    #[wasm_bindgen_test]
    fn test_wasm_todo_creation() {
        let app = TodoApp::new();
        assert_eq!(app.get_todo_count(), 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_add_todo() {
        let mut app = TodoApp::new();
        app.add_todo("WASM task", "Shannon", "2024-01-01");
        assert_eq!(app.get_todo_count(), 1);
    }

    #[test]
    fn test_edit_todo() {
        let mut app = TodoApp::new();
        app.add_todo("Original task", "Joe", "2024-01-01");

        let success = app.edit_todo(1, "Updated task", "Shannon", "2024-01-02");
        assert!(success);

        let todos_json = app.get_todos_json();
        assert!(todos_json.contains("\"text\":\"Updated task\""));
        assert!(todos_json.contains("\"assignee\":\"Shannon\""));
        assert!(todos_json.contains("\"date\":\"2024-01-02\""));
    }

    #[test]
    fn test_edit_nonexistent_todo() {
        let mut app = TodoApp::new();
        let success = app.edit_todo(999, "New text", "Joe", "2024-01-01");
        assert!(!success);
    }

    #[wasm_bindgen_test]
    fn test_wasm_edit_todo() {
        let mut app = TodoApp::new();
        app.add_todo("WASM task", "Joe", "2024-01-01");

        let success = app.edit_todo(1, "Edited WASM task", "Shannon", "2024-01-02");
        assert!(success);
    }
}
