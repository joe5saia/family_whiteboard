use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const NO_DUE_DATE_GROUP: &str = "No Due Date";

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodoItem {
    id: u32,
    text: String,
    assignee: String,
    date: String,
    completed: bool,
}

impl TodoItem {
    fn new(id: u32, text: &str, assignee: &str, date: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            assignee: assignee.to_string(),
            date: date.to_string(),
            completed: false,
        }
    }

    fn toggle_completion(&mut self) {
        self.completed = !self.completed;
    }

    fn update(&mut self, text: &str, assignee: &str, date: &str) {
        self.text = text.to_string();
        self.assignee = assignee.to_string();
        self.date = date.to_string();
    }

    fn date_group_key(&self) -> String {
        if self.date.is_empty() {
            NO_DUE_DATE_GROUP.to_string()
        } else {
            self.date.clone()
        }
    }
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
        let todo = TodoItem::new(self.next_id, text, assignee, date);
        self.todos.push(todo);
        self.next_id += 1;
        self.sort_todos();
    }

    #[wasm_bindgen]
    pub fn toggle_todo(&mut self, id: u32) {
        if let Some(todo) = self.find_todo_by_id_mut(id) {
            todo.toggle_completion();
            self.sort_todos();
        }
    }

    #[wasm_bindgen]
    pub fn get_todos_json(&self) -> String {
        serde_json::to_string(&self.todos).unwrap_or_else(|_| "[]".to_string())
    }

    #[wasm_bindgen]
    pub fn get_todos_grouped_by_date_json(&self) -> String {
        let grouped = self.group_todos_by_date();
        let sorted_groups = self.sort_date_groups(grouped);
        serde_json::to_string(&sorted_groups).unwrap_or_else(|_| "[]".to_string())
    }

    #[wasm_bindgen]
    pub fn get_todo_count(&self) -> usize {
        self.todos.len()
    }

    #[wasm_bindgen]
    pub fn edit_todo(&mut self, id: u32, text: &str, assignee: &str, date: &str) -> bool {
        if let Some(todo) = self.find_todo_by_id_mut(id) {
            todo.update(text, assignee, date);
            self.sort_todos();
            true
        } else {
            false
        }
    }

    fn find_todo_by_id_mut(&mut self, id: u32) -> Option<&mut TodoItem> {
        self.todos.iter_mut().find(|todo| todo.id == id)
    }

    fn group_todos_by_date(&self) -> HashMap<String, Vec<&TodoItem>> {
        let mut grouped: HashMap<String, Vec<&TodoItem>> = HashMap::new();

        for todo in &self.todos {
            let date_key = todo.date_group_key();
            grouped.entry(date_key).or_default().push(todo);
        }

        // Sort todos within each group
        for todos in grouped.values_mut() {
            todos.sort_by(|a, b| Self::compare_todos_for_sorting(a, b));
        }

        grouped
    }

    fn sort_date_groups(
        &self,
        grouped: HashMap<String, Vec<&TodoItem>>,
    ) -> Vec<(String, Vec<TodoItem>)> {
        let mut date_groups: Vec<(String, Vec<TodoItem>)> = grouped
            .into_iter()
            .map(|(date, todos)| (date, todos.into_iter().cloned().collect()))
            .collect();

        date_groups.sort_by(Self::compare_date_groups);
        date_groups
    }

    fn compare_todos_for_sorting(a: &TodoItem, b: &TodoItem) -> std::cmp::Ordering {
        match (a.completed, b.completed) {
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            _ => a.id.cmp(&b.id),
        }
    }

    fn compare_date_groups(
        a: &(String, Vec<TodoItem>),
        b: &(String, Vec<TodoItem>),
    ) -> std::cmp::Ordering {
        match (a.0.as_str(), b.0.as_str()) {
            (NO_DUE_DATE_GROUP, NO_DUE_DATE_GROUP) => std::cmp::Ordering::Equal,
            (NO_DUE_DATE_GROUP, _) => std::cmp::Ordering::Less,
            (_, NO_DUE_DATE_GROUP) => std::cmp::Ordering::Greater,
            (date_a, date_b) => date_a.cmp(date_b),
        }
    }

    fn sort_todos(&mut self) {
        self.todos.sort_by(Self::compare_todos_for_sorting);
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