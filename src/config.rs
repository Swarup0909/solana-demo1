use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// Raydium AMM V4 Program ID
pub const RAYDIUM_AMM_PROGRAM: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

// Top 10 Solana Meme Coin Pools on Raydium DEX
// These are the actual Raydium V4 pool addresses for USDC pairs
// Note: These addresses need to be verified and updated with current pool addresses
pub struct PoolConfig {
    pub name: String,
    pub address: Pubkey,
}

pub fn get_pool_configs() -> Vec<PoolConfig> {
    vec![
        PoolConfig {
            name: "BONK-USDC".to_string(),
            // Raydium BONK/USDC pool - verify on Solscan
            address: Pubkey::from_str("Hx3Ksp8WjXtJvFnvhZqYfXyD7eNxVFWBLGAA3HqHpxDV").unwrap(),
        },
        PoolConfig {
            name: "WIF-USDC".to_string(),
            // dogwifhat/USDC pool
            address: Pubkey::from_str("EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx").unwrap(),
        },
        PoolConfig {
            name: "POPCAT-USDC".to_string(),
            // Popcat/USDC pool
            address: Pubkey::from_str("FhdKdo7NL1MvWJZMn54yqWqcEFa1yVpVmJLZVXsrXzQX").unwrap(),
        },
        PoolConfig {
            name: "GIGA-USDC".to_string(),
            // Gigachad/USDC pool
            address: Pubkey::from_str("5dQJyVNQYXpPvEQoGSvDNcz2xCHvk7tVfQjGmCxW8VmX").unwrap(),
        },
        PoolConfig {
            name: "PNUT-USDC".to_string(),
            // Peanut the Squirrel/USDC pool
            address: Pubkey::from_str("9fC3CvvqVQvVtbqTYZMYJbcqHJ3Y4KR9QHXUkBSKJPdD").unwrap(),
        },
        PoolConfig {
            name: "MEW-USDC".to_string(),
            // cat in a dogs world/USDC pool
            address: Pubkey::from_str("2gNqMUVVuJVcw7qxT3YDxnMQQR6p9v6qMZfkQvN2V5Rs").unwrap(),
        },
        PoolConfig {
            name: "PENGU-USDC".to_string(),
            // Pengu/USDC pool
            address: Pubkey::from_str("5P3giWpT7pWKHqQFgJ8b3Ykw9PJCrDXxFx9YqD6H3qmZ").unwrap(),
        },
        PoolConfig {
            name: "AI16Z-USDC".to_string(),
            // ai16z/USDC pool
            address: Pubkey::from_str("7DdvCTvmN9JQNs4x1Y3k6PVBuqVkwW9E2JqVhHvqvD6X").unwrap(),
        },
        PoolConfig {
            name: "FARTCOIN-USDC".to_string(),
            // Fartcoin/USDC pool
            address: Pubkey::from_str("3xKbVFJPZgQXKjqVUz2J9YdWuN5YkLqP4CZvRnD5vMkW").unwrap(),
        },
        PoolConfig {
            name: "MOTHER-USDC".to_string(),
            // Mother Iggy/USDC pool
            address: Pubkey::from_str("6VK1ksrmYGMBWUZwVY7KmWdqJ7v4d9KvmH8K2RqPvZMz").unwrap(),
        },
    ]
}

pub fn get_raydium_program_id() -> Pubkey {
    Pubkey::from_str(RAYDIUM_AMM_PROGRAM).unwrap()
}

// Configuration values from environment or defaults
pub fn get_rpc_url() -> String {
    std::env::var("RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string())
}

pub fn get_ws_url() -> String {
    std::env::var("WS_URL").unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string())
}

pub fn get_server_addr() -> String {
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    format!("{}:{}", host, port)
}

pub fn get_poll_interval_ms() -> u64 {
    std::env::var("POLL_INTERVAL_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000)
}

pub fn get_price_deviation_threshold() -> f64 {
    std::env::var("PRICE_DEVIATION_THRESHOLD")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.5)
}

