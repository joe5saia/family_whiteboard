# Development Setup for Family Todo App

This document describes the optimized development environment for the Family Todo App with real-time hot reloading.

## Quick Start

```bash
# Start development environment
./dev.sh start

# View logs
./dev.sh logs

# Stop environment
./dev.sh stop
```

## Features

### 🔥 Hot Reloading
- **Frontend**: Changes to HTML, CSS, and JavaScript are reflected immediately
- **Backend**: Rust code automatically recompiles on save using `cargo-watch`
- **No manual restarts needed!**

### 📁 Volume Mounts
- Frontend source files are mounted directly into nginx container
- Backend source is mounted with Rust compilation cache preserved
- Database data persists between restarts

### 🛠 Development Tools

#### Using the dev.sh Script
```bash
# Start all services
./dev.sh start

# View logs for specific service
./dev.sh logs backend
./dev.sh logs frontend
./dev.sh logs postgres

# Open shell in container
./dev.sh shell backend
./dev.sh shell frontend

# Connect to database
./dev.sh db

# Run migrations
./dev.sh migrate

# Clean everything (including volumes)
./dev.sh clean
```

#### Manual Docker Commands
```bash
# Start with build
docker compose -f docker-compose.dev.yml up --build

# Start in background
docker compose -f docker-compose.dev.yml up -d

# Rebuild specific service
docker compose -f docker-compose.dev.yml build backend

# View real-time logs
docker compose -f docker-compose.dev.yml logs -f
```

## Architecture

### Service URLs
- **Frontend**: http://localhost:8080
- **Backend API**: http://localhost:3000
- **Database Admin**: http://localhost:8081
- **API Docs** (optional): http://localhost:8082

### File Structure
```
.
├── docker-compose.dev.yml    # Development compose file
├── Dockerfile.dev           # Frontend dev container
├── nginx.dev.conf          # Nginx dev configuration
├── backend/
│   └── Dockerfile.dev      # Backend dev container
├── frontend/               # Frontend source (hot reload)
│   ├── index.html
│   ├── styles.css
│   └── app.js
└── dev.sh                  # Development helper script
```

### Key Differences from Production

1. **No Build Step**: Source files are mounted directly
2. **Debug Logging**: Enhanced logging for development
3. **CORS Enabled**: Permissive CORS for local development
4. **No Caching**: All caching disabled for instant updates
5. **Source Maps**: Full debugging support

## Development Workflow

### Frontend Development
1. Edit files in `frontend/` directory
2. Changes appear immediately in browser
3. Browser DevTools work normally

### Backend Development
1. Edit Rust files in `backend/src/`
2. `cargo-watch` detects changes
3. Automatically recompiles and restarts
4. Watch logs with `./dev.sh logs backend`

### Database Changes
1. Add migration files to `backend/migrations/`
2. Run `./dev.sh migrate`
3. Use Adminer at http://localhost:8081 for GUI access

## Troubleshooting

### Container won't start
```bash
# Clean and rebuild
./dev.sh clean
./dev.sh build
./dev.sh start
```

### Port already in use
```bash
# Find and kill process using port
lsof -i :8080
kill -9 <PID>
```

### Cargo watch not detecting changes
```bash
# Restart backend service
docker compose -f docker-compose.dev.yml restart backend
```

### Database connection issues
```bash
# Check postgres is healthy
docker compose -f docker-compose.dev.yml ps
# Check logs
./dev.sh logs postgres
```

## Performance Optimization

The development setup uses several optimizations:

1. **Delegated mounts**: Better performance on macOS
2. **Cargo cache volumes**: Preserves compiled dependencies
3. **Separate target directory**: Avoids conflicts with host
4. **Minimal rebuilds**: Only changed files trigger recompilation

## Environment Variables

Edit `.env.development` to customize:
- Database credentials
- API ports
- Log levels
- Feature flags

## Next Steps

1. Start the development environment: `./dev.sh start`
2. Open http://localhost:8080 in your browser
3. Start coding! Changes will reflect automatically
4. Use `./dev.sh logs` to monitor all services

Happy coding! 🚀