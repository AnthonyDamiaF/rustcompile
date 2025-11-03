# Backend Components Breakdown

This document lists all components, modules, and dependencies of the stock-trading-backend to help diagnose build and deployment issues.

## 📦 **Core Application**

**Binary Name:** `stock-trading-backend`  
**Entry Point:** `src/main.rs`  
**Framework:** Actix-web 4.4 (Rust async web framework)  
**Runtime:** Tokio (async runtime)

---

## 🏗️ **Core Modules (src/)**

### 1. **config.rs**
- **Purpose:** Environment configuration and settings
- **Key Features:**
  - Loads environment variables
  - Database URL construction
  - JWT secret configuration
  - Polygon API key management
  - Frontend origin CORS settings
- **Dependencies:** None (pure config)
- **Potential Issues:** Missing environment variables, invalid database URL format

### 2. **db.rs**
- **Purpose:** PostgreSQL database connection pooling
- **Key Features:**
  - Connection pool creation (sqlx)
  - Unix socket connection support (Cloud SQL)
  - URL encoding/decoding for credentials
- **Dependencies:** `sqlx`, `tokio`
- **Potential Issues:** Database connection string parsing, Cloud SQL socket path, connection timeouts

### 3. **models.rs**
- **Purpose:** Database models and structs
- **Key Features:**
  - User models
  - Stock models
  - Transaction models
  - Portfolio models
- **Dependencies:** `serde`, `sqlx`, `chrono`, `uuid`
- **Potential Issues:** Serialization/deserialization errors, type mismatches

### 4. **security.rs**
- **Purpose:** Authentication and authorization
- **Key Features:**
  - Password hashing (Argon2)
  - JWT token generation/validation
  - Password reset token generation
- **Dependencies:** `argon2`, `jsonwebtoken`, `rand`
- **Potential Issues:** JWT secret configuration, hashing algorithm issues

### 5. **email.rs**
- **Purpose:** Email sending functionality
- **Key Features:**
  - Password reset emails
  - SMTP configuration
- **Dependencies:** `lettre`
- **Potential Issues:** SMTP configuration, email server connection

### 6. **polygon.rs**
- **Purpose:** Polygon.io API integration
- **Key Features:**
  - Market data fetching
  - Real-time data connection testing
  - Delayed data connection testing
- **Dependencies:** `reqwest`, `tokio`
- **Potential Issues:** API key authentication, network requests

### 7. **websocket.rs**
- **Purpose:** WebSocket server for real-time data
- **Key Features:**
  - WebSocket connection handling
  - Real-time stock price updates
- **Dependencies:** `actix-web-actors`, `tokio-tungstenite`, `actix`
- **Potential Issues:** WebSocket protocol handling, connection management

### 8. **websocket_polygon.rs**
- **Purpose:** Polygon.io WebSocket integration
- **Key Features:**
  - Polygon.io WebSocket client
- **Dependencies:** `tokio-tungstenite`
- **Potential Issues:** WebSocket connection to external service

---

## 🛣️ **Routes (src/routes/)**

### 1. **auth.rs**
- **Endpoints:**
  - `POST /auth/register` - User registration
  - `POST /auth/login` - User login
  - `POST /auth/request-reset` - Request password reset
  - `POST /auth/reset-password` - Reset password
- **Dependencies:** `models`, `security`, `db`, `email`

### 2. **me.rs**
- **Endpoints:**
  - `GET /me` - Get current user info
- **Dependencies:** `models`, `security`, `db`

### 3. **stocks.rs**
- **Endpoints:**
  - `GET /api/stocks` - Get all stocks
  - `POST /api/stocks` - Create stock
  - `GET /api/stocks/{symbol}` - Get stock by symbol
  - `PUT /api/stocks/{symbol}` - Update stock
  - `DELETE /api/stocks/{symbol}` - Delete stock
- **Dependencies:** `models`, `db`

### 4. **transactions.rs**
- **Endpoints:**
  - `GET /api/transactions` - Get all transactions
  - `POST /api/transactions` - Create transaction
  - `POST /api/transactions/debug` - Debug transaction
  - `GET /api/transactions/{stock_code}` - Get transactions by stock
  - `GET /api/positions` - Get all positions
  - `GET /api/positions/{stock_code}` - Get position by stock
- **Dependencies:** `models`, `db`

### 5. **portfolio.rs**
- **Endpoints:**
  - `GET /api/portfolio/summary/{user_id}` - Get portfolio summary
  - `GET /api/portfolio/metrics/{user_id}` - Get portfolio metrics
  - `GET /api/portfolio/summary-with-change/{user_id}` - Get summary with daily change
  - `POST /api/portfolio/update/{user_id}` - Update portfolio summary
  - `GET /api/portfolio/cash-balance/{user_id}` - Get cash balance
  - `POST /api/portfolio/initialize-cash/{user_id}` - Initialize cash balance
- **Dependencies:** `models`, `db`

### 6. **reports.rs**
- **Endpoints:**
  - `POST /reports/pre` - Create pre-market report
  - `POST /reports/post` - Create post-market report
- **Dependencies:** `models`, `db`

### 7. **configuration.rs**
- **Endpoints:**
  - `GET /api/config` - Get all configurations
  - `GET /api/config/app` - Get app configuration
  - `GET /api/config/{key}` - Get configuration by key
  - `GET /api/config/category/{category}` - Get configurations by category
  - `PUT /api/config/{key}` - Update configuration
  - `POST /api/config` - Create configuration
  - `DELETE /api/config/{key}` - Delete configuration
- **Dependencies:** `services::configuration`, `db`

### 8. **market_reports.rs**
- **Endpoints:**
  - `GET /api/market-reports` - Get all market reports
  - `POST /api/market-reports` - Create market report
  - `DELETE /api/market-reports` - Delete market report by date
  - `GET /api/market-reports/{id}` - Get market report by ID
  - `PUT /api/market-reports/{id}` - Update market report
  - `DELETE /api/market-reports/{id}` - Delete market report
- **Dependencies:** `services::market_reports`, `db`

---

## 🔧 **Services (src/services/)**

### 1. **configuration.rs**
- **Purpose:** Configuration management service
- **Key Features:**
  - CRUD operations on configuration table
  - Category-based configuration retrieval
- **Dependencies:** `sqlx`, `models`
- **Special Note:** Must be `Clone` to share across Actix-web workers

### 2. **market_reports.rs**
- **Purpose:** Market reports business logic
- **Key Features:**
  - Market report creation and management
- **Dependencies:** `sqlx`, `models`

### 3. **exchange_date_service.rs**
- **Purpose:** Exchange date calculations
- **Key Features:**
  - Trading day calculations
  - Holiday handling
- **Dependencies:** `chrono`

---

## 🎯 **Handlers (src/handlers/)**

### 1. **watchlist.rs**
- **Purpose:** Watchlist management
- **Endpoints:**
  - `GET /api/watchlist/{username}` - Get user watchlist
  - `POST /api/watchlist/{username}` - Add to watchlist
  - `DELETE /api/watchlist/{username}/{symbol}` - Remove from watchlist
- **Dependencies:** `models`, `db`

---

## 📚 **External Dependencies (Cargo.toml)**

### Core Framework:
- `actix-web = "4.4"` - Web framework
- `actix-cors = "0.6"` - CORS middleware
- `tokio = "1.0"` - Async runtime
- `tokio-tungstenite = "0.20"` - WebSocket (with native-tls)

### Database:
- `sqlx = "0.6"` - Async SQL (PostgreSQL, with rustls)
  - Features: `runtime-tokio-rustls`, `postgres`, `chrono`, `uuid`, `bigdecimal`

### Authentication:
- `argon2 = "0.4"` - Password hashing
- `jsonwebtoken = "8"` - JWT tokens
- `rand = "0.8"` - Random number generation

### Serialization:
- `serde = "1.0"` - Serialization framework
- `serde_json = "1.0"` - JSON support

### Utilities:
- `dotenvy = "0.15"` - Environment variable loading
- `chrono = "0.4"` - Date/time handling
- `uuid = "1.0"` - UUID generation
- `url = "2.4"` - URL parsing
- `once_cell = "1.19"` - Lazy static initialization

### External APIs:
- `reqwest = "0.11"` - HTTP client
- `lettre = "0.10"` - Email sending (SMTP)

### Error Handling:
- `anyhow = "1.0"` - Error handling
- `thiserror = "1.0"` - Custom error types

### WebSocket:
- `actix-web-actors = "4.2"` - Actix actors for WebSocket
- `actix = "0.13"` - Actor framework
- `futures-util = "0.3"` - Futures utilities

### Numeric:
- `rust_decimal = "1.32"` - Decimal numbers
- `bigdecimal = "0.3"` - Big decimal numbers

### Logging:
- `env_logger = "0.10"` - Logging framework

---

## 🔍 **Build & Deployment Components**

### Dockerfile Components:
1. **Build Stage (rust:latest)**
   - Creates empty binary project with `cargo init --bin .`
   - Copies `Cargo.toml` and `Cargo.lock`
   - Builds dependencies (cached)
   - Copies source code (`src/`)
   - Builds final binary with `cargo build --release`
   - Checks libraries with `ldd`

2. **Runtime Stage (debian:bullseye-slim)**
   - Installs: `ca-certificates`, `libssl1.1`, `libc6`
   - Copies binary from build stage
   - Creates entrypoint script
   - Exposes port 8080

### Entrypoint Script:
- Checks binary existence and permissions
- Displays PORT environment variable
- Starts application (currently in background for debugging)
- Keeps container alive for 5 minutes (debugging mode)

---

## ⚠️ **Known Potential Issues by Component**

### Build-Time Issues:
1. **Cargo.lock version mismatch** ✅ (Fixed - using rust:latest)
2. **Missing Cargo.lock** ✅ (Fixed - added to Git)
3. **Dependency compilation errors** - Check if any crate fails to compile
4. **Native library linking** - SSL/TLS libraries (libssl1.1)

### Runtime Issues:
1. **Database Connection** (`db.rs`)
   - Unix socket path for Cloud SQL
   - URL encoding/decoding of credentials
   - Connection timeout (currently 5 seconds)

2. **PORT Binding** (`main.rs`)
   - Must bind to `0.0.0.0` (not `127.0.0.1`)
   - PORT environment variable from Cloud Run

3. **Configuration Loading** (`config.rs`)
   - Missing environment variables
   - Database URL construction
   - JWT secret availability

4. **Application Startup** (`main.rs`)
   - Database connection must succeed before server starts
   - Server must bind successfully
   - Server must call `.run().await` (blocking)

---

## 🎯 **Debugging Focus Areas**

### High Priority:
1. **Database Connection** - Most likely cause of startup failures
2. **PORT Binding** - Must bind to 0.0.0.0:PORT
3. **Configuration Loading** - Environment variables must be set

### Medium Priority:
4. **Binary Compilation** - Check if all dependencies compile
5. **Library Dependencies** - SSL/TLS libraries present

### Low Priority:
6. **WebSocket Initialization** - Only needed for real-time features
7. **Email Service** - Only needed for password resets
8. **Polygon Integration** - Only needed for market data

---

## 📝 **Compilation Order**

1. Core modules compile first (config, db, models, security)
2. Routes depend on core modules
3. Services depend on routes and core modules
4. Main.rs pulls everything together
5. Binary linked with native libraries (SSL/TLS)

---

## 🚀 **Minimal Test Version**

There's a `main_simple.rs` file that creates a minimal server without:
- Database connection
- Configuration loading
- Any routes except `/health`

This can be used to test if the basic Actix-web server works in Cloud Run.

