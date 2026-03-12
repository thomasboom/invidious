//! Invidious main entry point.
//!
//! A Rust rewrite of the Invidious YouTube alternative front-end.

use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Command line arguments for Invidious.
#[derive(Parser, Debug)]
#[command(name = "invidious")]
#[command(version = "0.1.0")]
#[command(about = "Invidious - An alternative front-end to YouTube", long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: PathBuf,

    /// Port to listen on
    #[arg(short, long)]
    port: Option<u16>,

    /// Host to bind to
    #[arg(long)]
    host: Option<String>,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

/// Main entry point.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Load configuration
    let config = if let Ok(invidious_config_file) = std::env::var("INVIDIOUS_CONFIG_FILE") {
        invidious::Config::load_from_file(&PathBuf::from(invidious_config_file))?
    } else if args.config.exists() {
        invidious::Config::load_from_file(&args.config)?
    } else if let Ok(config_str) = std::env::var("INVIDIOUS_CONFIG") {
        serde_yaml::from_str(&config_str)?
    } else {
        eprintln!("Warning: No configuration file found, using defaults");
        invidious::Config::default()
    };

    // Override config with CLI arguments
    let port = args.port.unwrap_or(config.port);
    let host = args.host.unwrap_or_else(|| config.host_binding.clone());
    let db_url = config.database_url();
    let log_level = if args.debug {
        "debug"
    } else {
        config.log_level.as_str()
    };
    let output = config.output.clone();

    // Initialize logging
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

    if output == "STDOUT" {
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    } else {
        let file_appender = tracing_appender::rolling::daily("logs", "invidious.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
            .init();

        // Keep the guard alive for the duration of the program
        Box::leak(Box::new(_guard));
    }

    tracing::info!("Starting Invidious on {}:{}", host, port);

    // Initialize database if configured
    if !db_url.is_empty() {
        match invidious::db::init_db(&db_url).await {
            Ok(pool) => {
                tracing::info!("Database connected successfully");
                // Store pool somewhere (in a state struct)
                let _ = pool;
            }
            Err(e) => {
                tracing::warn!("Failed to connect to database: {}", e);
            }
        }
    } else {
        tracing::warn!("No database configured, running without database");
    }

    // Initialize template engine
    let templates = invidious::templates::TemplateEngine::new("templates/**/*")?;
    tracing::info!("Templates loaded successfully");

    // Create router
    let app = invidious::routes::create_router(config, templates);

    // Create address
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], port)));

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
