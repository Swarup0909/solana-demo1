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
                    tracing::info!(
                        "💰 {} | Price: ${:.8} | Base: {} | Quote: {}",
                        pair_price.pair,
                        pair_price.price,
                        pair_price.base_reserve,
                        pair_price.quote_reserve
                    );
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
    let receiver_count = tx.receiver_count();
    if receiver_count > 0 {
        tracing::info!("📡 Broadcasting price update to {} client(s)", receiver_count);
    }
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
    
    // Raydium AMM V4 pool structure offsets
    // Based on Raydium SDK: https://github.com/raydium-io/raydium-sdk
    // Pool token amounts are stored at specific offsets in the account data
    
    // Try multiple common offsets for Raydium V4
    // Offset 0x1D8 (472) and 0x1E0 (480) are common for pool PC and Coin amounts
    let base_reserve = extract_u64_from_data(data, 0x1D8)?; // Pool coin amount (base)
    let quote_reserve = extract_u64_from_data(data, 0x1E0)?; // Pool PC amount (quote/USDC)

    // Calculate price (quote/base) with decimal adjustment
    // BONK has 5 decimals, USDC has 6 decimals
    // Price = (quote_reserve / 10^6) / (base_reserve / 10^5)
    // Simplifies to: (quote_reserve / base_reserve) * (10^5 / 10^6) = (quote_reserve / base_reserve) * 0.1
    let price = if base_reserve > 0 {
        let raw_price = (quote_reserve as f64) / (base_reserve as f64);
        // Adjust for decimals: BONK (5) vs USDC (6)
        raw_price * 0.1 // This gives us price in USDC per BONK
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

