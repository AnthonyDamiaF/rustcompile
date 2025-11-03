# Cloud Run Deployment Guide

## Step 1: Build the Docker Image

The Cloud Build trigger will automatically build your Docker image when you push to GitHub.

**Build Configuration:**
- Type: Dockerfile
- Dockerfile directory: `backend/`
- Dockerfile name: `Dockerfile`
- Image name: `gcr.io/projecttrade-471406/rustcompile:$COMMIT_SHA`

## Step 2: Deploy to Cloud Run

After the build completes, deploy the image to Cloud Run:

### Option A: Deploy via Console

1. Go to [Cloud Run Console](https://console.cloud.google.com/run)
2. Click "Create Service" or select existing service
3. Configure:
   - **Container image**: Select the image from Container Registry
   - **Service name**: `rustcompile` (or your preferred name)
   - **Region**: Choose your region
   - **Port**: `8080` (default)

### Option B: Deploy via gcloud CLI

```bash
gcloud run deploy rustcompile \
  --image gcr.io/projecttrade-471406/rustcompile:latest \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated
```

## Step 3: Set Environment Variables

**In Cloud Run Console:**
1. Go to your Cloud Run service
2. Click "Edit & Deploy New Revision"
3. Go to "Variables & Secrets" tab
4. Add these environment variables:

### Required Environment Variables:

```
DB_HOST = /cloudsql/projecttrade-471406:REGION:INSTANCE_NAME
```
- For Cloud SQL Unix socket connection
- Format: `/cloudsql/PROJECT_ID:REGION:INSTANCE_NAME`
- Example: `/cloudsql/projecttrade-471406:us-central1:my-instance`

```
DB_NAME = sedge_core
```
- Your PostgreSQL database name

```
DB_USERNAME = sedge_user
```
- Your PostgreSQL username

```
DB_PASSWORD = 1yopainTony_
```
- Your PostgreSQL password
- **⚠️ For production, use Google Secret Manager instead!**

```
DB_PORT = 5432
```
- PostgreSQL port (default: 5432)
- Only needed for TCP connections (not Unix socket)

### Optional Environment Variables:

```
FRONTEND_ORIGIN = https://your-frontend-domain.com
```
- Frontend URL for CORS configuration
- Default: `http://localhost:3000`

```
PORT = 8080
```
- Server port (Cloud Run sets this automatically)

## Step 4: Connect Cloud SQL (If Using Cloud SQL)

**In Cloud Run Console:**
1. Go to your Cloud Run service
2. Click "Edit & Deploy New Revision"
3. Go to "Connections" tab
4. Under "Cloud SQL connections", click "Add Connection"
5. Select your Cloud SQL instance
6. Click "Deploy"

**Or via gcloud CLI:**
```bash
gcloud run services update rustcompile \
  --add-cloudsql-instances projecttrade-471406:REGION:INSTANCE_NAME \
  --region us-central1
```

## Step 5: Using Google Secret Manager (Recommended for Production)

For sensitive values like `DB_PASSWORD`, use Secret Manager:

### Create Secret:
```bash
echo -n "1yopainTony_" | gcloud secrets create db-password \
  --data-file=- \
  --replication-policy="automatic"
```

### Grant Access:
```bash
gcloud secrets add-iam-policy-binding db-password \
  --member="serviceAccount:SERVICE_ACCOUNT_EMAIL" \
  --role="roles/secretmanager.secretAccessor"
```

### Use in Cloud Run:
1. Go to "Variables & Secrets" tab
2. Under "Secrets", click "Add Secret"
3. Select: `db-password`
4. Version: `latest`
5. Variable name: `DB_PASSWORD`

## Step 6: Verify Deployment

After deployment, test your endpoints:

```bash
# Health check
curl https://YOUR-SERVICE-URL.run.app/health

# Hello World
curl https://YOUR-SERVICE-URL.run.app/hello

# Database test
curl https://YOUR-SERVICE-URL.run.app/api/db-test
```

## Environment Variables Summary

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `PORT` | No | `8080` | Server port (Cloud Run sets automatically) |
| `DB_HOST` | **Yes** | - | Cloud SQL socket path or IP address |
| `DB_NAME` | **Yes** | - | PostgreSQL database name |
| `DB_USERNAME` | **Yes** | - | PostgreSQL username |
| `DB_PASSWORD` | **Yes** | - | PostgreSQL password |
| `DB_PORT` | No | `5432` | PostgreSQL port (for TCP connections) |
| `FRONTEND_ORIGIN` | No | `http://localhost:3000` | Frontend URL for CORS |

## Troubleshooting

### Database Connection Fails
1. Check Cloud SQL instance is running
2. Verify Cloud SQL connection in Cloud Run service
3. Check environment variables are set correctly
4. Verify database credentials
5. Check Cloud SQL instance allows connections from Cloud Run

### Build Fails
1. Check Cloud Build logs
2. Verify Dockerfile is correct
3. Check Rust compilation errors
4. Verify Cargo.lock is committed

### Service Won't Start
1. Check Cloud Run logs
2. Verify PORT is set (or use default 8080)
3. Check database connection timeout
4. Verify all required environment variables are set

