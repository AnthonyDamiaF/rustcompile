#!/bin/bash

# Startup script for test-backend
# This script starts the server in release mode

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVER_NAME="test-backend"
PORT=8080

echo "🚀 Starting $SERVER_NAME server..."

# Check if server is already running
EXISTING_PID=$(pgrep -f "$SERVER_NAME" || true)
if [ -n "$EXISTING_PID" ]; then
    echo "⚠️  Server is already running (PID: $EXISTING_PID)"
    echo "📋 Run ./stopserver.sh first if you want to restart"
    exit 1
fi

# Check if port is in use
PORT_PID=$(lsof -ti:$PORT 2>/dev/null || true)
if [ -n "$PORT_PID" ]; then
    echo "❌ Port $PORT is already in use by PID: $PORT_PID"
    echo "📋 Run ./stopserver.sh first to free the port"
    exit 1
fi

# Check if .env file exists
if [ ! -f "$SCRIPT_DIR/.env" ]; then
    echo "⚠️  Warning: .env file not found in $SCRIPT_DIR"
    echo "📋 Using system environment variables"
fi

# Change to script directory
cd "$SCRIPT_DIR"

# Check if Cargo.toml exists
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found in $SCRIPT_DIR"
    exit 1
fi

# Build if needed (check if binary exists and is newer than source)
if [ ! -f "target/release/$SERVER_NAME" ] || \
   [ "src/main.rs" -nt "target/release/$SERVER_NAME" ] || \
   [ "Cargo.toml" -nt "target/release/$SERVER_NAME" ]; then
    echo "🔨 Building release version..."
    cargo build --release
    echo "✅ Build complete"
fi

# Start the server
echo "🌐 Starting server on port $PORT..."
echo "📋 Server will be available at: http://localhost:$PORT"
echo "📋 Health check: http://localhost:$PORT/health"
echo ""

# Wait a moment for server to start, then open browser
(sleep 3 && echo "🌐 Opening browser..." && open "http://localhost:$PORT/health" 2>/dev/null || xdg-open "http://localhost:$PORT/health" 2>/dev/null || echo "⚠️  Could not auto-open browser, please open manually") &

echo "Press Ctrl+C to stop the server"
echo ""

# Run the server in foreground so we can see logs
exec ./target/release/$SERVER_NAME

