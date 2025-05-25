# Family Todo Backend

A Rust backend server for the Family Todo application with PostgreSQL persistence and real-time WebSocket updates.

## Features

- REST API for todo CRUD operations
- Real-time WebSocket updates for multi-user synchronization
- PostgreSQL database with automatic migrations
- CORS support for frontend integration

## Prerequisites

- Rust (latest stable)
- PostgreSQL database
- Environment variables configured

## Setup

1. **Install dependencies**:
```bash
cd backend
```

2. **Set up environment**:
```bash
cp .env.example .env
# Edit .env with your database credentials
```

3. **Create PostgreSQL database**:
```sql
CREATE DATABASE family_todo;
```

4. **Run the server**:
```bash
cargo run
```

The server will:
- Run database migrations automatically
- Start on port 3000 (or PORT environment variable)
- Serve the frontend files from the parent directory

## API Endpoints

### REST API

- `GET /api/todos` - Get all todos grouped by date
- `POST /api/todos` - Create a new todo
- `PUT /api/todos/:id` - Update a todo
- `PUT /api/todos/:id/toggle` - Toggle todo completion
- `DELETE /api/todos/:id` - Delete a todo

### WebSocket

- `GET /ws` - WebSocket connection for real-time updates

## Request/Response Examples

### Create Todo
```bash
curl -X POST http://localhost:3000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"text": "Buy groceries", "assignee": "Joe", "due_date": "2024-01-15"}'
```

### Update Todo
```bash
curl -X PUT http://localhost:3000/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"text": "Buy organic groceries", "completed": false}'
```

### Toggle Todo
```bash
curl -X PUT http://localhost:3000/api/todos/1/toggle
```

## WebSocket Messages

The server broadcasts these message types:
- `todo_created` - When a new todo is created
- `todo_updated` - When a todo is updated
- `todo_toggled` - When a todo is toggled
- `todo_deleted` - When a todo is deleted

## Development

Run with auto-reload:
```bash
cargo install cargo-watch
cargo watch -x run
```

Run tests:
```bash
cargo test
```