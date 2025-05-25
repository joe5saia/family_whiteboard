# Hello WASM Project

This is a Rust + WebAssembly hello world application.

## Setup

To build and run this project:

1. Build the WASM package:
```bash
wasm-pack build --target web
```

2. Serve the project locally:
```bash
python3 -m http.server 8000
```

3. Open http://localhost:8000 in your browser

## Project Structure

- `src/lib.rs` - Rust code that compiles to WASM
- `Cargo.toml` - Rust dependencies and WASM configuration
- `index.html` - HTML file that loads and uses the WASM module
- `pkg/` - Generated WASM bindings (created after build)

## Dependencies

- `wasm-bindgen` - Rust/WASM/JS bindings
- `web-sys` - Web API bindings for Rust