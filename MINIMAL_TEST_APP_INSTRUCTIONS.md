# Instructions for Creating Minimal Test Backend Application

## 🎯 Objective

Create a **minimal backend application** that includes all critical components from the main backend to isolate and identify the build/deployment issue. This test app will be used to determine if the problem is:
- **Systemic** (affects any Rust/Actix-web app on Cloud Run) OR
- **Application-specific** (something in the main application code)

## 📋 Application Requirements

### Structure to Create:
```
Trade/test-backend/
├── Cargo.toml
├── Cargo.lock (generated, should be committed)
├── Dockerfile (identical structure to main backend)
└── src/
    └── main.rs
```

### Key Components to Include:

#### 1. **Minimal Actix-web Server**
- Health check endpoint (`/health`)
- Must bind to `0.0.0.0:PORT` (Cloud Run requirement)
- Use PORT environment variable (default 8080)

#### 2. **Database Connection (Same as Main App)**
- Use `sqlx` with PostgreSQL
- Use `runtime-tokio-rustls` feature
- Support Unix socket connection (Cloud SQL style)
- Same database URL construction logic
- Connection timeout: 5 seconds
- **Important:** Should connect to the same Cloud SQL database as main app

#### 3. **Configuration Loading**
- Use `dotenvy` for environment variables
- Same environment variable names as main app:
  - `PORT`
  - `DB_HOST`
  - `DB_NAME`
  - `DB_USERNAME`
  - `DB_PASSWORD`
  - `DB_PORT` (default 5432)
- Same database URL format as main app

#### 4. **Critical Dependencies (Exact Versions from Main App)**
Include these exact dependencies from main app's Cargo.toml:
```toml
actix-web = "4.4"
actix-cors = "0.6"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
sqlx = { version = "0.6", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid"] }
dotenvy = "0.15"
env_logger = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

#### 5. **Dockerfile (Copy from Main App)**
- Use **exact same Dockerfile structure** as `Trade/backend/Dockerfile`
- Same multi-stage build (rust:latest → debian:bullseye-slim)
- Same entrypoint script
- Same library dependencies (libssl1.1, ca-certificates, libc6)
- Same binary name pattern (but can be `test-backend`)

#### 6. **Diagnostic Logging**
- Add extensive `println!` statements like main app
- Log PORT environment variable before and after dotenv
- Log database connection attempts
- Log server binding attempts
- All logs should flush with `std::io::stdout().flush().ok()`

## 📝 Step-by-Step Implementation Instructions

### Step 1: Create Project Structure

```bash
cd Trade
mkdir test-backend
cd test-backend
```

### Step 2: Create Cargo.toml

Create `Cargo.toml` with minimal dependencies:

```toml
[package]
name = "test-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.4"
actix-cors = "0.6"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
sqlx = { version = "0.6", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid"] }
dotenvy = "0.15"
env_logger = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

### Step 3: Create Minimal main.rs

**Key Requirements:**
1. Load environment variables with `dotenvy`
2. Construct database URL exactly like main app
3. Attempt database connection (with timeout)
4. Start Actix-web server on `0.0.0.0:PORT`
5. Health check endpoint that works
6. Extensive logging throughout

**Database URL Construction:**
Use the exact same logic from `Trade/backend/src/config.rs`:
- Format: `postgresql://{username}:{password}@localhost/{db_name}?host={db_host}`
- URL encode username and password if they contain special characters

**Example Structure:**
```rust
use actix_web::{web, App, HttpServer, HttpResponse};
use dotenvy::dotenv;
use std::io::Write;

// Database connection function (simplified from main app)
async fn connect_db(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    sqlx::PgPool::connect(database_url).await
}

// Health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "OK",
        "service": "test-backend"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Flush logs immediately
    std::io::stdout().flush().ok();
    
    println!("🚀 TEST BACKEND: Starting application...");
    println!("📋 Process ID: {}", std::process::id());
    
    // Check PORT before dotenv
    let port_before = std::env::var("PORT");
    println!("📋 PORT before dotenv: {:?}", port_before);
    
    // Load environment
    dotenv().ok();
    println!("✅ Environment variables loaded");
    
    // Check PORT after dotenv
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("Invalid PORT");
    println!("📋 PORT after dotenv: {}", port);
    
    // Construct database URL (same logic as main app)
    let db_host = std::env::var("DB_HOST").expect("DB_HOST not set");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME not set");
    let db_username = std::env::var("DB_USERNAME").expect("DB_USERNAME not set");
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
    
    // URL encode credentials (simplified version)
    let encode_url = |s: &str| -> String {
        s.chars().map(|c| match c {
            ':' => "%3A".to_string(),
            '@' => "%40".to_string(),
            _ => c.to_string(),
        }).collect()
    };
    
    let database_url = format!(
        "postgresql://{}:{}@localhost/{}?host={}",
        encode_url(&db_username),
        encode_url(&db_password),
        db_name,
        db_host
    );
    
    println!("📊 Attempting database connection...");
    let pool = match tokio::time::timeout(
        std::time::Duration::from_secs(5),
        connect_db(&database_url)
    ).await {
        Ok(Ok(pool)) => {
            println!("✅ Database connected successfully");
            pool
        }
        Ok(Err(e)) => {
            eprintln!("❌ Database connection failed: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                format!("Database failed: {}", e)
            ));
        }
        Err(_) => {
            eprintln!("❌ Database connection timed out");
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Database timeout"
            ));
        }
    };
    
    // Start server
    println!("🌐 Starting HTTP server on 0.0.0.0:{}", port);
    std::io::stdout().flush().ok();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
    })
    .bind(("0.0.0.0", port))
    .map_err(|e| {
        eprintln!("❌ Failed to bind: {}", e);
        e
    })?
    .run()
    .await
}
```

### Step 4: Create Dockerfile (Copy from Main App)

**Important:** Use the **EXACT SAME Dockerfile structure** as `Trade/backend/Dockerfile`:

```dockerfile
# Stage 1: Build
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Create empty project
RUN cargo init --bin .

# Copy manifests
COPY ./Cargo.toml ./Cargo.lock ./

# Build dependencies (cached)
RUN cargo build --release

# Copy source
COPY ./src ./src

# Build application
RUN cargo build --release

# Check libraries
RUN ldd /usr/src/app/target/release/test-backend || true

# Stage 2: Runtime
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /usr/src/app/target/release/test-backend /app/test-backend

# Make executable
RUN chmod +x /app/test-backend

# Create entrypoint script
RUN printf '#!/bin/sh\n\
set -e\n\
echo "🔍 Container starting..."\n\
echo "📋 Working directory: $(pwd)"\n\
echo "📋 PORT environment variable: ${PORT:-8080}"\n\
echo "📋 Checking binary..."\n\
if [ ! -f "/app/test-backend" ]; then\n\
    echo "❌ ERROR: Binary not found"\n\
    exit 1\n\
fi\n\
if [ ! -x "/app/test-backend" ]; then\n\
    echo "❌ ERROR: Binary not executable"\n\
    exit 1\n\
fi\n\
echo "✅ Binary found and executable"\n\
echo "✅ Starting application..."\n\
/app/test-backend &\n\
sleep 300\n\
' > /app/entrypoint.sh && chmod +x /app/entrypoint.sh

EXPOSE 8080
ENV PORT=8080
WORKDIR /app
CMD ["/app/entrypoint.sh"]
```

### Step 5: Generate Cargo.lock

```bash
cd Trade/test-backend
cargo build
# This will generate Cargo.lock
git add Cargo.lock  # Commit this file!
```

### Step 6: Create .env File (for local testing)

Create `.env` file with same variables as main app:
```
PORT=8080
DB_HOST=/cloudsql/PROJECT_ID:REGION:INSTANCE_NAME
DB_NAME=your_db_name
DB_USERNAME=your_username
DB_PASSWORD=your_password
DB_PORT=5432
```

### Step 7: Test Locally First

```bash
cd Trade/test-backend
cargo run
# Should start on http://localhost:8080
# Test: curl http://localhost:8080/health
```

### Step 8: Build Docker Image Locally

```bash
cd Trade/test-backend
docker build -t test-backend:local .
docker run -p 8080:8080 --env-file .env test-backend:local
```

## 🎯 Success Criteria

The test application should:
1. ✅ Compile successfully with `cargo build --release`
2. ✅ Build Docker image successfully
3. ✅ Connect to Cloud SQL database (same as main app)
4. ✅ Bind to `0.0.0.0:PORT` successfully
5. ✅ Start HTTP server and respond to `/health`
6. ✅ Deploy to Cloud Run successfully

## 🔍 What to Compare

After creating and testing the minimal app, compare:

### If Minimal App WORKS but Main App FAILS:
- Problem is **application-specific**
- Focus on differences:
  - Additional dependencies in main app
  - Additional route handlers
  - Additional services
  - Different initialization order

### If Minimal App ALSO FAILS:
- Problem is **systemic**
- Likely issues:
  - Docker build configuration
  - Cloud Run environment
  - Database connection method
  - PORT binding mechanism

## 📊 Key Differences to Test

1. **Database Connection Method**
   - Same Unix socket path
   - Same URL encoding/decoding
   - Same timeout settings

2. **PORT Binding**
   - Same `0.0.0.0:PORT` binding
   - Same environment variable reading

3. **Docker Build**
   - Same base images
   - Same library installation
   - Same entrypoint script

4. **Dependencies**
   - Minimal app uses only critical deps
   - Main app has 32 dependencies

## 🚨 Critical Notes for Agent

1. **DO NOT** simplify the database connection - use EXACT same logic
2. **DO NOT** change the Dockerfile structure - copy it exactly
3. **DO** use the same environment variable names
4. **DO** use the same URL encoding/decoding logic
5. **DO** include extensive logging like main app
6. **DO** commit `Cargo.lock` to Git

## 📝 Expected Output

The minimal app should produce logs like:
```
🚀 TEST BACKEND: Starting application...
📋 Process ID: 12345
📋 PORT before dotenv: Ok("8080")
✅ Environment variables loaded
📋 PORT after dotenv: 8080
📊 Attempting database connection...
✅ Database connected successfully
🌐 Starting HTTP server on 0.0.0.0:8080
```

## ✅ Final Checklist

- [ ] Cargo.toml created with minimal dependencies
- [ ] main.rs created with database connection and server
- [ ] Dockerfile copied from main app (adapted for test-backend)
- [ ] Cargo.lock generated and committed
- [ ] .env file created for local testing
- [ ] Local test successful (`cargo run`)
- [ ] Docker build successful (`docker build`)
- [ ] Docker run successful (`docker run`)
- [ ] Ready for Cloud Run deployment

---

**This minimal application will help identify if the issue is:**
- ✅ Build configuration problem
- ✅ Database connection problem
- ✅ PORT binding problem
- ✅ Application-specific code problem
- ✅ Dependency-related problem

