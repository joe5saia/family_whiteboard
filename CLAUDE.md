# Family Todo App

A modern web application built with Rust + WebAssembly for the backend logic and vanilla JavaScript for the frontend.

## Features

- ✅ Add, edit, and complete todos
- 👥 Assign tasks to family members (Joe, Shannon, or Unassigned)
- 📅 Set due dates and organize by date groups
- 🔍 Filter todos by assignee, status, and date range
- 💾 Persistent collapsible date groups (localStorage)
- 🎨 Responsive design with modern UI

## Architecture

### Backend (Rust + WASM)
- **`src/lib.rs`** - Core todo logic compiled to WebAssembly
  - `TodoItem` struct with serialization support
  - `TodoApp` for managing todo state and operations
  - Comprehensive test suite with both unit and WASM tests

### Frontend
- **`index.html`** - Clean HTML structure with semantic markup
- **`styles.css`** - Organized CSS with component-based styling
- **`app.js`** - Modular JavaScript controller with class-based architecture

### Generated Files
- **`pkg/`** - WASM bindings and JavaScript glue code (auto-generated)

## Setup and Development

### Initial Build
```bash
wasm-pack build --target web
```

### Local Development Server
```bash
python3 -m http.server 8000
```
Then open http://localhost:8000

### Development Workflow

After making code changes, run these commands in order:

1. **Format code:**
```bash
cargo fmt
```

2. **Run linter:**
```bash
cargo clippy
```

3. **Run tests:**
```bash
cargo test
```

4. **Run WASM tests:**
```bash
wasm-pack test --node
```

5. **Rebuild WASM:**
```bash
wasm-pack build --target web
```

## Code Structure

### Rust Architecture

The Rust code follows clean architecture principles:

- **Constants**: Shared constants for date grouping
- **TodoItem**: Immutable data structure with helper methods
- **TodoApp**: Stateful controller with separated concerns:
  - Public WASM-bindgen methods for JavaScript interface
  - Private helper methods for internal logic
  - Comparison functions for consistent sorting

### JavaScript Architecture

The frontend uses a class-based controller pattern:

- **TodoAppController**: Main application controller
  - Initialization and event setup
  - Todo CRUD operations
  - UI rendering and state management
  - Filter system with persistent state

### CSS Organization

Styles are organized by component:

- Base styles (body, containers)
- Form components
- Button variants
- Todo list and items
- Date grouping and collapsible sections
- Filter modal and controls
- Responsive utilities

## Dependencies

### Rust Dependencies
- `wasm-bindgen` - Rust/WASM/JavaScript bindings
- `serde` + `serde_json` - Serialization for data exchange
- `web-sys` - Web API bindings
- `wasm-bindgen-test` - WASM-specific testing

### Browser Requirements
- Modern browser with ES6 module support
- WebAssembly support
- localStorage for persistence

## Testing

The project includes comprehensive testing:

- **Unit tests**: Test core Rust logic
- **WASM tests**: Test WebAssembly integration
- **Integration tests**: Test JavaScript-WASM interface

Run all tests:
```bash
cargo test && wasm-pack test --node
```