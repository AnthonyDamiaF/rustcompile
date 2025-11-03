# Cloud Run Deployment - Quick Guide

## Step 1: Deploy Your Service

### Via Console:

1. Go to [Cloud Run Console](https://console.cloud.google.com/run)
2. Click **"Create Service"**
3. **Service settings:**

   - **Container image**: `gcr.io/projecttrade-471406/rustcompile:latest`
   - **Service name**: `rustcompile`
   - **Region**: Choose your region (e.g., `us-central1`)
   - **Port**: `8080`

4. **Increase startup timeout (if needed):**

   - Go to **"Container"** tab
   - Under **"Startup timeout"**, set to `600s` (10 minutes) or higher
   - This helps if your app takes time to connect to database

5. Click **"Create"** or **"Deploy"** to deploy

## Step 2: Set Environment Variables

After the service is created, click **"Edit & Deploy New Revision"**:

### Go to "Variables & Secrets" tab

Add these environment variables:

**Required:**

- `DB_HOST` = `34.72.242.81` (or `/cloudsql/projecttrade-471406:REGION:INSTANCE_NAME` for Cloud SQL)
- `DB_NAME` = `sedge_core`
- `DB_USERNAME` = `sedge_user`
- `DB_PASSWORD` = `1yopainTony_`
- `DB_PORT` = `5432`

**Optional:**

- `FRONTEND_ORIGIN` = `https://your-frontend-url.com` (or leave default)

## Step 3: Connect Cloud SQL (if using Cloud SQL)

1. In "Edit & Deploy New Revision"
2. Go to **"Connections"** tab
3. Under **"Cloud SQL connections"**, click **"Add Connection"**
4. Select your Cloud SQL instance
5. Click **"Deploy"**

## Step 4: Verify Deployment

After deployment, test your endpoints:

```bash
# Replace YOUR_SERVICE_URL with your actual Cloud Run URL
curl https://YOUR_SERVICE_URL/health
curl https://YOUR_SERVICE_URL/hello
curl https://YOUR_SERVICE_URL/api/db-test
```

---

## Option 2: Deploy via gcloud CLI

```bash
# Deploy the service with increased startup timeout
gcloud run deploy rustcompile \
  --image gcr.io/projecttrade-471406/rustcompile:latest \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars DB_HOST=34.72.242.81,DB_NAME=sedge_core,DB_USERNAME=sedge_user,DB_PASSWORD=1yopainTony_,DB_PORT=5432 \
  --port 8080 \
  --timeout 600s

# If using Cloud SQL, also add:
gcloud run services update rustcompile \
  --add-cloudsql-instances projecttrade-471406:REGION:INSTANCE_NAME \
  --region us-central1
```

## Important Notes:

- **PORT**: Cloud Run sets this automatically, your app reads it from `PORT` env var
- **Database**: Make sure your database is accessible from Cloud Run
- **Security**: For production, use Google Secret Manager for DB_PASSWORD instead of plain env vars
