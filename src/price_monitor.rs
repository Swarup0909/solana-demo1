use crate::config::{get_poll_interval_ms, get_pool_configs, get_price_deviation_threshold, get_rpc_url};
use crate::solana_client::SolanaClient;
use crate::types::{ArbitrageAlert, PairPrice, PriceUpdate};
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast::Sender;
use tokio::time::{sleep, Duration};

pub async fn start_monitoring(tx: Sender<String>) -> Result<()> {
    tracing::info!("Starting price monitoring");

    let rpc_url = get_rpc_url();
    let client = SolanaClient::new(rpc_url);
    let pool_configs = get_pool_configs();
    let poll_interval = Duration::from_millis(get_poll_interval_ms());

    let mut previous_prices: HashMap<String, f64> = HashMap::new();

    loop {
        match fetch_and_broadcast_prices(&client, &pool_configs, &tx, &mut previous_prices).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Error fetching prices: {:?}", e);
            }
        }

        sleep(poll_interval).await;
    }
}

async fn fetch_and_broadcast_prices(
    client: &SolanaClient,
    pool_configs: &[crate::config::PoolConfig],
    tx: &Sender<String>,
    previous_prices: &mut HashMap<String, f64>,
) -> Result<()> {
    let pool_pubkeys: Vec<Pubkey> = pool_configs.iter().map(|p| p.address).collect();

    // Fetch all pool accounts in a single RPC call
    let accounts = client.get_multiple_accounts(&pool_pubkeys).await?;

    let mut pair_prices = Vec::new();

    for (idx, account_opt) in accounts.iter().enumerate() {
        let pool_config = &pool_configs[idx];

        if let Some(account) = account_opt {
            match parse_pool_price(&account.data, pool_config, previous_prices) {
                Ok(pair_price) => {
                    pair_prices.push(pair_price);
                }
                Err(e) => {
                    tracing::warn!("Failed to parse pool {}: {:?}", pool_config.name, e);
                }
            }
        } else {
            tracing::warn!("Pool account {} not found", pool_config.name);
        }
    }

    // Check for arbitrage opportunities
    check_arbitrage(&pair_prices, tx).await?;

    // Create price update message
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();

    let price_update = PriceUpdate {
        pairs: pair_prices,
        update_time: timestamp,
    };

    let json = serde_json::to_string(&price_update)?;

    // Broadcast to all WebSocket clients
    // It's okay if there are no receivers
    let _ = tx.send(json);

    Ok(())
}

fn parse_pool_price(
    data: &[u8],
    pool_config: &crate::config::PoolConfig,
    previous_prices: &mut HashMap<String, f64>,
) -> Result<PairPrice> {
    // For Raydium AMM V4, we need to get the token account reserves
    // The pool account contains references to token accounts
    // We need to fetch those separately or parse from the pool state
    
    // Simplified approach: Extract reserves from pool account data
    // Note: Raydium pools store reserves at specific offsets
    // This may need adjustment based on actual Raydium V4 structure
    
    let base_reserve = extract_u64_from_data(data, 0x140)?; // Offset for base reserve
    let quote_reserve = extract_u64_from_data(data, 0x148)?; // Offset for quote reserve

    // Calculate price (quote/base)
    // Adjust for decimals if needed (most USDC pairs use 6 decimals for USDC)
    let price = if base_reserve > 0 {
        (quote_reserve as f64) / (base_reserve as f64)
    } else {
        0.0
    };

    // Update previous prices
    previous_prices.insert(pool_config.name.clone(), price);

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(PairPrice {
        pair: pool_config.name.clone(),
        price,
        timestamp,
        base_reserve,
        quote_reserve,
        pool_address: pool_config.address.to_string(),
    })
}

fn extract_u64_from_data(data: &[u8], offset: usize) -> Result<u64> {
    if data.len() < offset + 8 {
        anyhow::bail!("Data too short for u64 extraction at offset {}", offset);
    }
    
    let bytes: [u8; 8] = data[offset..offset + 8]
        .try_into()
        .map_err(|_| anyhow::anyhow!("Failed to convert to u64 array"))?;
    
    Ok(u64::from_le_bytes(bytes))
}

async fn check_arbitrage(pair_prices: &[PairPrice], _tx: &Sender<String>) -> Result<()> {
    let threshold = get_price_deviation_threshold();

    // Check for price deviations that could indicate arbitrage opportunities
    for (i, price1) in pair_prices.iter().enumerate() {
        for price2 in pair_prices.iter().skip(i + 1) {
            if price1.pair == price2.pair {
                continue;
            }

            let deviation = ((price1.price - price2.price) / price1.price).abs() * 100.0;

            if deviation > threshold {
                let alert = ArbitrageAlert {
                    pair: format!("{} vs {}", price1.pair, price2.pair),
                    deviation,
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };

                let json = serde_json::to_string(&alert)?;
                tracing::warn!("Arbitrage opportunity detected: {}", json);
                
                // Could send alert to a separate channel if needed
                // let _ = tx.send(json);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_u64() {
        let data = vec![0u8; 400];
        let result = extract_u64_from_data(&data, 0x140);
        assert!(result.is_ok());
    }
}

