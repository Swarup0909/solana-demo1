mod config;
mod server;
mod solana_client;
mod price_monitor;
mod types;

use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "solana_price_monitor=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Solana Price Monitor");

    // Create broadcast channel for price updates
    let (tx, _rx) = tokio::sync::broadcast::channel(100);
    
    // Start price monitoring task
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        if let Err(e) = price_monitor::start_monitoring(tx_clone).await {
            tracing::error!("Price monitoring error: {:?}", e);
        }
    });

    // Start web server
    server::start_server(tx).await?;

    Ok(())
}


