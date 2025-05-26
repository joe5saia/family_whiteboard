#!/bin/bash

# Development helper script for the Family Todo App

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}→ $1${NC}"
}

# Main functions
start() {
    print_info "Starting development environment..."
    docker compose -f docker-compose.dev.yml up -d
    print_success "Development environment started!"
    print_info "Frontend: http://localhost:8080"
    print_info "Backend API: http://localhost:3000"
    print_info "Database Admin: http://localhost:8081"
}

stop() {
    print_info "Stopping development environment..."
    docker compose -f docker-compose.dev.yml down
    print_success "Development environment stopped!"
}

restart() {
    print_info "Restarting development environment..."
    stop
    start
}

logs() {
    service=${1:-}
    if [ -z "$service" ]; then
        docker compose -f docker-compose.dev.yml logs -f
    else
        docker compose -f docker-compose.dev.yml logs -f "$service"
    fi
}

build() {
    print_info "Building development containers..."
    docker compose -f docker-compose.dev.yml build
    print_success "Build complete!"
}

shell() {
    service=${1:-backend}
    print_info "Opening shell in $service container..."
    docker compose -f docker-compose.dev.yml exec "$service" /bin/bash
}

db() {
    print_info "Connecting to PostgreSQL..."
    docker compose -f docker-compose.dev.yml exec postgres psql -U todo_user -d family_todos
}

migrate() {
    print_info "Running database migrations..."
    docker compose -f docker-compose.dev.yml exec backend sqlx migrate run
    print_success "Migrations complete!"
}

clean() {
    print_info "Cleaning development environment..."
    docker compose -f docker-compose.dev.yml down -v
    print_success "Clean complete!"
}

status() {
    docker compose -f docker-compose.dev.yml ps
}

# Parse command
case "$1" in
    start)
        start
        ;;
    stop)
        stop
        ;;
    restart)
        restart
        ;;
    logs)
        logs "$2"
        ;;
    build)
        build
        ;;
    shell)
        shell "$2"
        ;;
    db)
        db
        ;;
    migrate)
        migrate
        ;;
    clean)
        clean
        ;;
    status)
        status
        ;;
    *)
        echo "Family Todo App Development Tool"
        echo ""
        echo "Usage: ./dev.sh [command]"
        echo ""
        echo "Commands:"
        echo "  start     - Start development environment"
        echo "  stop      - Stop development environment"
        echo "  restart   - Restart development environment"
        echo "  logs      - Show logs (optional: service name)"
        echo "  build     - Build development containers"
        echo "  shell     - Open shell in container (default: backend)"
        echo "  db        - Connect to PostgreSQL"
        echo "  migrate   - Run database migrations"
        echo "  clean     - Clean up (removes volumes)"
        echo "  status    - Show container status"
        echo ""
        echo "Examples:"
        echo "  ./dev.sh start"
        echo "  ./dev.sh logs backend"
        echo "  ./dev.sh shell frontend"
        ;;
esac