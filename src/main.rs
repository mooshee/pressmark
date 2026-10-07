//! Pressmark Binary Entrypoint
//!
//! Author: Daniel Hallman

use pressmark::engine::find_typst_executable;
use pressmark::server::{create_router, AppState};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("Pressmark - High-Performance Vector Document Engine");
        println!("Author: Daniel Hallman");
        println!();
        println!("Usage: pressmark [OPTIONS]");
        println!();
        println!("Options:");
        println!("  -h, --help       Show this help message and exit");
        println!("  -v, --version    Show version information and exit");
        println!();
        println!("Environment Variables:");
        println!("  HOST             Server host (default: 127.0.0.1)");
        println!("  PORT             Server port (default: 3842)");
        println!("  BASE_URL         Base URL for public links (default: http://HOST:PORT)");
        return Ok(());
    }
    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("pressmark {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if args.iter().any(|a| a == "--stdio" || a == "mcp" || a == "--mcp") {
        use std::io::{BufRead, Write};
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        for line in stdin.lock().lines() {
            if let Ok(line_str) = line {
                if line_str.trim().is_empty() {
                    continue;
                }
                if let Ok(req) = serde_json::from_str::<pressmark::mcp::JsonRpcRequest>(&line_str) {
                    let resp = pressmark::mcp::McpHandler::handle_request(req);
                    let _ = serde_json::to_writer(&mut out, &resp);
                    let _ = out.write_all(b"\n");
                    let _ = out.flush();
                }
            }
        }
        return Ok(());
    }

    // Load local environment files if present
    let _ = dotenvy::from_filename(".local/operator.env");
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pressmark=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let host = std::env::var("PRESSMARK_HOST")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("PRESSMARK_PORT")
        .or_else(|_| std::env::var("PORT"))
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3842);

    let typst_path = find_typst_executable().unwrap_or_else(|| {
        eprintln!("WARNING: Vector engine binary not detected in PATH or standard directories.");
        std::path::PathBuf::from("typst")
    });

    let addr = format!("{}:{}", host, port);
    let base_url = std::env::var("PRESSMARK_BASE_URL")
        .or_else(|_| std::env::var("BASE_URL"))
        .unwrap_or_else(|_| format!("http://{}", addr));

    println!("============================================================");
    println!("  PRESSMARK · Sub-Millisecond Vector Document Engine        ");
    println!("  Author: Daniel Hallman                                    ");
    println!("============================================================");
    println!("  Vector Engine    : {:?}", typst_path);
    println!("  Server Address   : {}", base_url);
    println!("  Dashboard Studio : {}/", base_url);
    println!("  OpenAPI 3.1 Spec : {}/api/v1/openapi.json", base_url);
    println!("  ChatGPT Plugin   : {}/.well-known/ai-plugin.json", base_url);
    println!("  MCP JSON-RPC     : {}/mcp", base_url);
    println!("  ChatGPT Auth     : {}/api/v1/auth/chatgpt/login", base_url);
    println!("  Pricing Catalog  : {}/api/v1/pricing", base_url);
    println!("  Health Check     : {}/health", base_url);
    println!("  Cache Pruner     : Active (1800s interval, 3600s TTL)");
    println!("  Database Pruner  : Active (3600s interval, 3d Stripe TTL)");
    println!("  Rate Limiter     : Active (Sliding Window, DoS Defense)");
    println!("============================================================");

    // Start background maintenance workers
    pressmark::engine::start_cache_pruner_task(1800, 3600);

    let app_state = Arc::new(AppState::new(base_url.clone()));
    app_state.rate_limiter.start_cleanup_task(300);
    pressmark::storage::start_db_pruner_task(Arc::clone(&app_state.storage), 3600, 3);

    let router = create_router(app_state);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
