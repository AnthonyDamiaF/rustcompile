# Minimal Test Application - Quick Summary

## 🎯 Goal
Create a **minimal backend** with only critical components to isolate the build failure issue.

## 📦 What to Create

**Location:** `Trade/test-backend/`

**Files:**
1. `Cargo.toml` - Minimal dependencies (10 crates only)
2. `src/main.rs` - Simple Actix-web server + database connection
3. `Dockerfile` - Copy from main app, adapt for test-backend
4. `Cargo.lock` - Generate and commit

## 🔑 Key Components (Copy from Main App)

### 1. Database Connection Logic
- Exact same database URL construction from `config.rs`
- Same URL encoding for username/password
- Same Unix socket format for Cloud SQL
- Same 5-second timeout

### 2. Dockerfile Structure
- Exact same multi-stage build
- Same base images (rust:latest → debian:bullseye-slim)
- Same libraries (libssl1.1, ca-certificates, libc6)
- Same entrypoint script pattern

### 3. Server Setup
- Same PORT binding (`0.0.0.0:PORT`)
- Same environment variable loading
- Same diagnostic logging

## 📋 Minimal Dependencies

```toml
actix-web = "4.4"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
sqlx = { version = "0.6", features = ["runtime-tokio-rustls", "postgres"] }
dotenvy = "0.15"
env_logger = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## ✅ Success Criteria

The test app should:
1. Compile successfully
2. Build Docker image successfully  
3. Connect to same Cloud SQL database
4. Deploy to Cloud Run successfully

## 🔍 What This Tests

- ✅ If test app WORKS → Problem is in main app code
- ✅ If test app FAILS → Problem is systemic (Docker/Cloud Run/config)

## 📝 For Agent Instructions

See `MINIMAL_TEST_APP_INSTRUCTIONS.md` for detailed step-by-step guide.

**Key Points:**
- Use EXACT same database connection logic
- Use EXACT same Dockerfile structure  
- Use EXACT same PORT binding method
- Include same diagnostic logging
- Test locally first, then Docker, then Cloud Run

