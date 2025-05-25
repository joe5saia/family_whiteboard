#!/bin/bash

# Family Todo Backend Startup Script

set -e  # Exit on any error

echo "🚀 Starting Family Todo Backend..."

# Check if PostgreSQL is running
if ! pgrep -x "postgres" > /dev/null; then
    echo "⚠️  PostgreSQL is not running. Please start PostgreSQL first."
    echo "   macOS: brew services start postgresql"
    echo "   Linux: sudo systemctl start postgresql"
    exit 1
fi

# Navigate to backend directory
cd "$(dirname "$0")/backend"

# Check if .env exists
if [ ! -f .env ]; then
    echo "📝 Creating .env file from example..."
    cp .env.example .env
    echo "✅ Please edit backend/.env with your database credentials"
fi

# Check if database exists
DB_NAME="family_todo"
if ! psql -lqt | cut -d \| -f 1 | grep -qw $DB_NAME; then
    echo "🗃️  Creating database '$DB_NAME'..."
    createdb $DB_NAME || {
        echo "❌ Failed to create database. Please create it manually:"
        echo "   psql -c 'CREATE DATABASE family_todo;'"
        exit 1
    }
    echo "✅ Database created successfully"
fi

# Build and run the server
echo "🔨 Building and starting server..."
cargo run

echo "🎉 Backend server should now be running on http://localhost:3000"