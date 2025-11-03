#!/bin/bash

# Graceful shutdown script for test-backend
# This script finds and gracefully stops the running server

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVER_NAME="test-backend"
PORT=8080

echo "🛑 Stopping $SERVER_NAME server..."

# First, check for processes using the port (more reliable)
PORT_PID=$(lsof -ti:$PORT 2>/dev/null || true)

# Also find process by name
NAME_PID=$(pgrep -f "$SERVER_NAME" || true)

# Use port-based PID if available, otherwise use name-based
if [ -n "$PORT_PID" ]; then
    PID=$PORT_PID
    echo "📋 Found process using port $PORT (PID: $PID)"
elif [ -n "$NAME_PID" ]; then
    PID=$NAME_PID
    echo "📋 Found $SERVER_NAME process (PID: $PID)"
else
    echo "ℹ️  No process found on port $PORT or named $SERVER_NAME"
    exit 0
fi

# Try graceful shutdown with SIGTERM
echo "📤 Sending SIGTERM for graceful shutdown..."
kill -TERM "$PID" 2>/dev/null || true

# Wait for process to terminate (max 10 seconds)
for i in {1..10}; do
    if ! kill -0 "$PID" 2>/dev/null; then
        echo "✅ Server stopped gracefully"
        exit 0
    fi
    sleep 1
    echo "⏳ Waiting for shutdown... ($i/10)"
done

# If still running, force kill
if kill -0 "$PID" 2>/dev/null; then
    echo "⚠️  Process did not stop gracefully, forcing shutdown..."
    kill -9 "$PID" 2>/dev/null || true
    sleep 1
    
    if kill -0 "$PID" 2>/dev/null; then
        echo "❌ Failed to stop process"
        exit 1
    else
        echo "✅ Server stopped (force kill)"
    fi
fi

# Verify port is free
if lsof -ti:$PORT >/dev/null 2>&1; then
    echo "⚠️  Warning: Port $PORT is still in use"
else
    echo "✅ Port $PORT is now free"
fi

