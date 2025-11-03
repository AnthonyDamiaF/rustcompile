# Cloud Run Deployment - Step by Step Guide

## Step 1: Open Cloud Run Console

1. Go to: https://console.cloud.google.com/run
2. Make sure you're in the correct project: `projecttrade-471406`
3. Click the **"Create Service"** button (top of the page)

---

## Step 2: Configure Container Image

1. Under **"Container"** section:

   - **Container image URL**: Click **"Select"**
   - In the dialog, find: `gcr.io/projecttrade-471406/rustcompile`
   - Select tag: **`latest`** (or use specific commit SHA like `2b770a5`)
   - Click **"Select"**

   **Or manually enter:**

   ```
   gcr.io/projecttrade-471406/rustcompile:latest
   ```

---

## Step 3: Service Basic Settings

1. **Service name**: `rustcompile`
2. **Region**: Choose your region (e.g., `us-central1`, `us-east1`)
3. **Platform**: `Cloud Run (fully managed)` ✅

---

## Step 4: Configure Container Settings

Click **"Container"** tab:

1. **Port**: `8080`
2. **Container port**: `8080`
3. **Startup timeout**: `600` seconds (10 minutes) - **Important!**
4. **Request timeout**: `300` seconds (5 minutes)

---

## Step 5: Set Environment Variables

Click **"Variables & Secrets"** tab → **"Add Variable"**

Add each variable one by one:

1. **DB_HOST**

   - Name: `DB_HOST`
   - Value: `34.72.242.81`

2. **DB_NAME**

   - Name: `DB_NAME`
   - Value: `sedge_core`

3. **DB_USERNAME**

   - Name: `DB_USERNAME`
   - Value: `sedge_user`

4. **DB_PASSWORD**

   - Name: `DB_PASSWORD`
   - Value: `1yopainTony_`

5. **DB_PORT**

   - Name: `DB_PORT`
   - Value: `5432`

6. **FRONTEND_ORIGIN** (Optional)
   - Name: `FRONTEND_ORIGIN`
   - Value: `https://your-frontend-url.com` (or leave default)

---

## Step 6: Configure Scaling (Optional)

1. Click **"Scaling"** tab
2. **Min instances**: `0` (default - no cost when idle)
3. **Max instances**: `10` (adjust as needed)
4. **Concurrency**: `80` (default is fine)

---

## Step 7: Set Service Permissions

1. Click **"Security"** tab
2. **Authentication**:
   - Select **"Allow unauthenticated invocations"** (if you want public access)
   - OR **"Require authentication"** (for private access)

---

## Step 8: Deploy

1. Click **"Create"** or **"Deploy"** button at the bottom
2. Wait for deployment (usually 1-3 minutes)
3. You'll see: **"Service rustcompile has been deployed"**

---

## Step 9: Get Your Service URL

After deployment completes:

1. You'll see your service in the Cloud Run services list
2. Click on `rustcompile` service
3. Copy the **Service URL** (looks like: `https://rustcompile-xxxxx-uc.a.run.app`)

---

## Step 10: Test Your Deployment

Open your browser or use curl:

```bash
# Health check
curl https://YOUR-SERVICE-URL/health

# Hello endpoint
curl https://YOUR-SERVICE-URL/hello

# Database test
curl https://YOUR-SERVICE-URL/api/db-test
```

---

## Troubleshooting

### If deployment fails:

1. **Check Cloud Run logs**:

   - Go to your service → **"Logs"** tab
   - Look for error messages

2. **Common issues**:

   - **Port not listening**: Make sure app binds to `0.0.0.0:PORT`
   - **Startup timeout**: Increase startup timeout to 600s
   - **Database connection**: Verify environment variables are set correctly

3. **View real-time logs**:
   - Cloud Run → Your service → **"Logs"** tab
   - Filter by revision if needed

---

## Update/Deploy New Revision

To deploy an update:

1. Go to Cloud Run → `rustcompile` service
2. Click **"Edit & Deploy New Revision"**
3. Update container image tag (if you have a new build)
4. Update environment variables if needed
5. Click **"Deploy"**

---

## Quick Reference

**Service URL format**: `https://rustcompile-xxxxx-REGION.a.run.app`

**Endpoints**:

- `/health` - Health check
- `/hello` - Hello World message
- `/api/db-test` - Database connection test

**Required Environment Variables**:

- `DB_HOST=34.72.242.81`
- `DB_NAME=sedge_core`
- `DB_USERNAME=sedge_user`
- `DB_PASSWORD=1yopainTony_`
- `DB_PORT=5432`

---

## Next Steps After Deployment

1. Test all endpoints
2. Update your frontend to point to the Cloud Run URL
3. Set up custom domain (optional)
4. Configure monitoring and alerts (optional)
