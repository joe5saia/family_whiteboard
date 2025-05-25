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

#[derive(Clone)]
enum AppState {
    Hello,
    World,
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