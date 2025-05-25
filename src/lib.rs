use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct App {
    state: AppState,
}

#[derive(Clone, PartialEq, Debug)]
enum AppState {
    Hello,
    World,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> App {
        App {
            state: AppState::Hello,
        }
    }

    #[wasm_bindgen]
    pub fn get_display_text(&self) -> String {
        match self.state {
            AppState::Hello => "Hello".to_string(),
            AppState::World => "World".to_string(),
        }
    }

    #[wasm_bindgen]
    pub fn get_button_text(&self) -> String {
        match self.state {
            AppState::Hello => "Continue".to_string(),
            AppState::World => "Reset".to_string(),
        }
    }

    #[wasm_bindgen]
    pub fn handle_button_click(&mut self) {
        self.state = match self.state {
            AppState::Hello => AppState::World,
            AppState::World => AppState::Hello,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    fn test_app_initial_state() {
        let app = App::new();
        assert_eq!(app.get_display_text(), "Hello");
        assert_eq!(app.get_button_text(), "Continue");
    }

    #[test]
    fn test_button_click_transitions() {
        let mut app = App::new();

        app.handle_button_click();
        assert_eq!(app.get_display_text(), "World");
        assert_eq!(app.get_button_text(), "Reset");

        app.handle_button_click();
        assert_eq!(app.get_display_text(), "Hello");
        assert_eq!(app.get_button_text(), "Continue");
    }

    #[wasm_bindgen_test]
    fn test_wasm_app_creation() {
        let app = App::new();
        assert_eq!(app.get_display_text(), "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_state_transition() {
        let mut app = App::new();
        app.handle_button_click();
        assert_eq!(app.get_display_text(), "World");
    }
}
