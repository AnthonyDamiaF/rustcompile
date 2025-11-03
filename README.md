# Rust Compilation Test Application

This is a minimal test application to diagnose Rust backend compilation issues in cloud builds.

## Project Structure

```
rustcompilationapp/
├── backend/           # Rust backend (Actix-web + PostgreSQL)
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   ├── Cargo.lock    # Generated after first build
│   └── Dockerfile
├── frontend/          # Simple HTML frontend
│   └── index.html
├── .env.example       # Environment variables template
└── README.md
```

## Features

- **Backend**: Minimal Actix-web server with:
  - Health check endpoint (`/health`)
  - Hello World endpoint (`/hello`)
  - Database test endpoint (`/api/db-test`)
  - Full database connection logic (same as main app)
  
- **Frontend**: Simple HTML page with:
  - Hello World display
  - Backend connectivity test
  - Database connectivity test

- **Database**: PostgreSQL connection with:
  - Unix socket support (Cloud SQL)
  - TCP connection support (local)
  - URL encoding for credentials
  - 5-second connection timeout

## Setup Instructions

### 1. Local Development

#### Prerequisites
- Rust (latest stable)
- PostgreSQL (or Cloud SQL connection)
- Cargo

#### Steps

1. **Copy environment file**:
   ```bash
   cp .env.example .env
   ```

2. **Edit `.env`** with your database credentials:
   ```bash
   PORT=8080
   DB_HOST=/cloudsql/PROJECT_ID:REGION:INSTANCE_NAME
   DB_NAME=your_db_name
   DB_USERNAME=your_username
   DB_PASSWORD=your_password
   ```

3. **Build and run backend**:
   ```bash
   cd backend
   cargo build
   cargo run
   ```

4. **Open frontend**:
   - Open `frontend/index.html` in your browser
   - Or serve it with a simple HTTP server:
     ```bash
     cd frontend
     python3 -m http.server 3000
     ```
   - Update the API URL in `index.html` if needed

### 2. Docker Build

#### Build Docker Image
```bash
cd backend
docker build -t test-backend:local .
```

#### Run Docker Container
```bash
docker run -p 8080:8080 \
  -e PORT=8080 \
  -e DB_HOST=/cloudsql/PROJECT_ID:REGION:INSTANCE_NAME \
  -e DB_NAME=your_db_name \
  -e DB_USERNAME=your_username \
  -e DB_PASSWORD=your_password \
  test-backend:local
```

Or use `.env` file:
```bash
docker run -p 8080:8080 --env-file ../.env test-backend:local
```

### 3. Cloud Run Deployment

1. **Build and push to Google Container Registry**:
   ```bash
   gcloud builds submit --tag gcr.io/PROJECT_ID/test-backend
   ```

2. **Deploy to Cloud Run**:
   ```bash
   gcloud run deploy test-backend \
     --image gcr.io/PROJECT_ID/test-backend \
     --platform managed \
     --region us-central1 \
     --allow-unauthenticated \
     --set-env-vars DB_HOST=/cloudsql/PROJECT_ID:REGION:INSTANCE_NAME \
     --set-env-vars DB_NAME=your_db_name \
     --set-env-vars DB_USERNAME=your_username \
     --set-env-vars DB_PASSWORD=your_password \
     --add-cloudsql-instances PROJECT_ID:REGION:INSTANCE_NAME
   ```

## API Endpoints

- `GET /health` - Health check endpoint
- `GET /hello` - Hello World message
- `GET /api/db-test` - Test database connection

## Testing

### Test Backend Locally
```bash
# Health check
curl http://localhost:8080/health

# Hello endpoint
curl http://localhost:8080/hello

# Database test
curl http://localhost:8080/api/db-test
```

### Expected Responses

**Health Check**:
```json
{
  "status": "OK",
  "service": "test-backend",
  "message": "Backend is running"
}
```

**Hello**:
```json
{
  "message": "Hello World from Rust Backend!",
  "status": "success"
}
```

**Database Test**:
```json
{
  "status": "success",
  "message": "Database connection is working",
  "test": "passed"
}
```

## Dependencies

The backend uses minimal dependencies:
- `actix-web = "4.4"` - Web framework
- `actix-cors = "0.6"` - CORS middleware
- `tokio = "1.0"` - Async runtime
- `sqlx = "0.6"` - PostgreSQL async driver
- `dotenvy = "0.15"` - Environment variables
- `serde` / `serde_json` - JSON serialization
- `chrono` - Date/time handling
- `url` - URL encoding

## Troubleshooting

### Database Connection Issues

1. **Check environment variables**:
   ```bash
   echo $DB_HOST
   echo $DB_NAME
   echo $DB_USERNAME
   ```

2. **Verify Cloud SQL connection**:
   - Ensure Cloud SQL instance exists
   - Check Unix socket path format: `/cloudsql/PROJECT_ID:REGION:INSTANCE_NAME`
   - Verify Cloud Run service has Cloud SQL connection

3. **Test connection string**:
   - The app logs the sanitized connection string on startup
   - Check for URL encoding issues in credentials

### Build Issues

1. **Generate Cargo.lock**:
   ```bash
   cd backend
   cargo build
   # This generates Cargo.lock
   ```

2. **Check Rust version**:
   ```bash
   rustc --version
   cargo --version
   ```

3. **Clean build**:
   ```bash
   cd backend
   cargo clean
   cargo build --release
   ```

### Port Binding Issues

- Ensure `PORT` environment variable is set
- The app binds to `0.0.0.0:PORT` (required for Cloud Run)
- Default port is `8080`

## Success Criteria

✅ Application compiles successfully  
✅ Docker image builds successfully  
✅ Application connects to database  
✅ Application binds to `0.0.0.0:PORT`  
✅ `/health` endpoint responds  
✅ `/hello` endpoint responds  
✅ `/api/db-test` endpoint responds  
✅ Deploys to Cloud Run successfully  

## Next Steps

If this minimal app works but the main app fails:
- Problem is **application-specific**
- Compare dependencies, routes, and initialization

If this minimal app also fails:
- Problem is **systemic**
- Check Docker configuration, Cloud Run environment, or database connection method

