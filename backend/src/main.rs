use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenvy::dotenv;
use std::io::Write;

// Database connection function
async fn connect_db(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    println!("📊 Connecting to database...");
    println!("📊 Database URL (sanitized): postgresql://***@localhost/***");

    sqlx::PgPool::connect(database_url).await
}

// Health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "OK",
        "service": "test-backend",
        "message": "Backend is running"
    }))
}

// Hello endpoint
async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello World from Rust Backend!",
        "status": "success"
    }))
}

// Database test endpoint (with pool)
async fn db_test(pool: web::Data<sqlx::PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1 as test")
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "Database connection is working",
            "test": "passed"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Database query failed",
            "error": e.to_string()
        })),
    }
}

// Database test endpoint (without pool - fallback)
async fn db_test_no_db() -> HttpResponse {
    HttpResponse::ServiceUnavailable().json(serde_json::json!({
        "status": "error",
        "message": "Database connection not available",
        "error": "Database was not connected during startup"
    }))
}

// URL encoding helper (same as main app)
fn encode_url_component(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable panic hook for better error logging
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {:?}", panic_info);
        std::io::stderr().flush().ok();
    }));

    // Flush logs immediately
    std::io::stdout().flush().ok();
    std::io::stderr().flush().ok();

    println!("🚀 TEST BACKEND: Starting application...");
    eprintln!("🚀 TEST BACKEND: Starting application...");
    println!("📋 Process ID: {}", std::process::id());
    eprintln!("📋 Process ID: {}", std::process::id());

    // Check PORT before dotenv
    let port_before = std::env::var("PORT");
    println!("📋 PORT before dotenv: {:?}", port_before);
    std::io::stdout().flush().ok();

    // Load environment variables
    match dotenv() {
        Ok(_) => println!("✅ Environment variables loaded from .env"),
        Err(_) => println!("⚠️  No .env file found, using system environment variables"),
    }
    std::io::stdout().flush().ok();

    // Check PORT after dotenv
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("❌ Invalid PORT, defaulting to 8080");
            8080
        });
    println!("📋 PORT after dotenv: {}", port);
    std::io::stdout().flush().ok();

    // Construct database URL (same logic as main app)
    let db_host = std::env::var("DB_HOST").unwrap_or_else(|_| {
        eprintln!("⚠️  DB_HOST not set, using default");
        "/tmp".to_string()
    });
    let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| {
        eprintln!("⚠️  DB_NAME not set, using default");
        "testdb".to_string()
    });
    let db_username = std::env::var("DB_USERNAME").unwrap_or_else(|_| {
        eprintln!("⚠️  DB_USERNAME not set, using default");
        "postgres".to_string()
    });
    let db_password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| {
        eprintln!("⚠️  DB_PASSWORD not set, using default");
        "postgres".to_string()
    });

    println!("📊 Database configuration:");
    println!(
        "   DB_HOST: {}",
        if db_host.contains('/') {
            "[Unix socket path]"
        } else {
            &db_host
        }
    );
    println!("   DB_NAME: {}", db_name);
    println!("   DB_USERNAME: {}", db_username);
    std::io::stdout().flush().ok();

    // URL encode credentials (same as main app)
    let encoded_username = encode_url_component(&db_username);
    let encoded_password = encode_url_component(&db_password);

    // Construct database URL - same format as main app
    let database_url = if db_host.starts_with('/') {
        // Unix socket connection (Cloud SQL style)
        format!(
            "postgresql://{}:{}@localhost/{}?host={}",
            encoded_username, encoded_password, db_name, db_host
        )
    } else {
        // TCP connection
        let db_port = std::env::var("DB_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse::<u16>()
            .unwrap_or(5432);
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            encoded_username, encoded_password, db_host, db_port, db_name
        )
    };

    println!("📊 Attempting database connection with 5s timeout...");
    std::io::stdout().flush().ok();

    // Attempt database connection with timeout (non-blocking)
    // Allow server to start even if DB connection fails
    let pool_result =
        tokio::time::timeout(std::time::Duration::from_secs(5), connect_db(&database_url)).await;

    let pool = match pool_result {
        Ok(Ok(pool)) => {
            println!("✅ Database connected successfully");
            std::io::stdout().flush().ok();
            Some(pool)
        }
        Ok(Err(e)) => {
            eprintln!("⚠️  Database connection failed: {}", e);
            eprintln!("⚠️  Server will start without database connection");
            eprintln!("⚠️  /api/db-test endpoint will not be available");
            std::io::stdout().flush().ok();
            None
        }
        Err(_) => {
            eprintln!("⚠️  Database connection timed out after 5 seconds");
            eprintln!("⚠️  Server will start without database connection");
            eprintln!("⚠️  /api/db-test endpoint will not be available");
            std::io::stdout().flush().ok();
            None
        }
    };

    // Start server - ensure we're ready before binding
    println!("🌐 Starting HTTP server on 0.0.0.0:{}", port);
    eprintln!("🌐 Starting HTTP server on 0.0.0.0:{}", port);
    std::io::stdout().flush().ok();
    std::io::stderr().flush().ok();

    // Ensure server starts immediately
    println!("✅ Server configuration complete, binding to port...");
    eprintln!("✅ Server configuration complete, binding to port...");
    std::io::stdout().flush().ok();
    std::io::stderr().flush().ok();

    // Get frontend origin from environment or default to localhost:3000
    let frontend_origin =
        std::env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());

    println!("🌐 Frontend origin: {}", frontend_origin);
    std::io::stdout().flush().ok();

    // Move pool into the closure properly
    let pool_opt = pool;
    let server = HttpServer::new(move || {
        // Allow all localhost origins for development (more permissive)
        let cors = Cors::default()
            .allowed_origin(&frontend_origin)
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:5500")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://127.0.0.1:5500")
            .allowed_origin_fn(|origin, _req_head| {
                // Allow any localhost or 127.0.0.1 origin for development
                let origin_str = origin.to_str().unwrap_or("");
                origin_str.starts_with("http://localhost")
                    || origin_str.starts_with("http://127.0.0.1")
                    || origin_str == "null" // Allow file:// protocol
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        let mut app = App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/hello", web::get().to(hello));

        // Only add database pool if connection was successful
        if let Some(ref p) = pool_opt {
            let pool_clone = p.clone();
            app = app
                .app_data(web::Data::new(pool_clone))
                .route("/api/db-test", web::get().to(db_test));
        } else {
            // Add route without pool - will return error message
            app = app.route("/api/db-test", web::get().to(db_test_no_db));
        }

        app
    });

    // Bind to port
    println!("🔌 Binding to 0.0.0.0:{}...", port);
    eprintln!("🔌 Binding to 0.0.0.0:{}...", port);
    std::io::stdout().flush().ok();
    std::io::stderr().flush().ok();

    let bound_server = server.bind(("0.0.0.0", port)).map_err(|e| {
        eprintln!("❌ Failed to bind to 0.0.0.0:{}: {}", port, e);
        eprintln!("❌ Error details: {:?}", e);
        std::io::stderr().flush().ok();
        std::io::stdout().flush().ok();
        e
    })?;

    println!(
        "✅ Successfully bound to 0.0.0.0:{}, starting server...",
        port
    );
    eprintln!(
        "✅ Successfully bound to 0.0.0.0:{}, starting server...",
        port
    );
    std::io::stdout().flush().ok();
    std::io::stderr().flush().ok();

    // Start the server
    bound_server.run().await
}
