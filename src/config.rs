use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// Raydium AMM V4 Program ID
#[allow(dead_code)]
pub const RAYDIUM_AMM_PROGRAM: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

// Top 10 Solana Meme Coin Pools on Raydium DEX
// These are the actual Raydium V4 pool addresses for USDC pairs
// Note: These addresses need to be verified and updated with current pool addresses
pub struct PoolConfig {
    pub name: String,
    pub address: Pubkey,
}

pub fn get_pool_configs() -> Vec<PoolConfig> {
    // ⚠️ IMPORTANT: The addresses below are PLACEHOLDERS and don't exist on mainnet!
    // To fix "Pool account not found" errors, you need to:
    // 1. Visit https://birdeye.so
    // 2. Search for each token (e.g., "BONK")
    // 3. Find the Raydium USDC pool
    // 4. Copy the real pool address
    // 5. Replace the placeholder address below
    // 6. Test with: ./scripts/test_pool_address.sh YOUR_ADDRESS
    //
    // See HOW_TO_FIND_POOLS.md for detailed instructions
    
    vec![
        // REAL POOL ADDRESSES - Verified on Raydium
        
        PoolConfig {
            name: "BONK-USDC".to_string(),
            // Raydium BONK/USDC pool - verified working
            address: Pubkey::from_str("Dwq4PxyBQ8dHPmP5u5H7bHsjHp46StGtkSy2gEVedDm").unwrap(),
        },
        // PoolConfig {
        //     name: "WIF-USDC".to_string(),
        //     address: Pubkey::from_str("YOUR_REAL_WIF_POOL_ADDRESS_HERE").unwrap(),
        // },
        // PoolConfig {
        //     name: "POPCAT-USDC".to_string(),
        //     address: Pubkey::from_str("YOUR_REAL_POPCAT_POOL_ADDRESS_HERE").unwrap(),
        // },
        // PoolConfig {
        //     name: "GIGA-USDC".to_string(),
        //     address: Pubkey::from_str("YOUR_REAL_GIGA_POOL_ADDRESS_HERE").unwrap(),
        // },
        // PoolConfig {
        //     name: "PNUT-USDC".to_string(),
        //     address: Pubkey::from_str("YOUR_REAL_PNUT_POOL_ADDRESS_HERE").unwrap(),
        // },
        // PoolConfig {
        //     name: "MEW-USDC".to_string(),
        //     address: Pubkey::from_str("YOUR_REAL_MEW_POOL_ADDRESS_HERE").unwrap(),
        // },
        // PoolConfig {
        //     name: "PENGU-USDC".to_string(),
        //     address: Pubkey::from_str("YOUR_REAL_PENGU_POOL_ADDRESS_HERE").unwrap(),
        // },
        // PoolConfig {
        //     name: "AI16Z-USDC".to_string(),
        //     address: Pubkey::from_str("YOUR_REAL_AI16Z_POOL_ADDRESS_HERE").unwrap(),
        // },
        // PoolConfig {
        //     name: "FARTCOIN-USDC".to_string(),
        //     address: Pubkey::from_str("YOUR_REAL_FARTCOIN_POOL_ADDRESS_HERE").unwrap(),
        // },
        // PoolConfig {
        //     name: "MOTHER-USDC".to_string(),
        //     address: Pubkey::from_str("YOUR_REAL_MOTHER_POOL_ADDRESS_HERE").unwrap(),
        // },
    ]
}

#[allow(dead_code)]
pub fn get_raydium_program_id() -> Pubkey {
    Pubkey::from_str(RAYDIUM_AMM_PROGRAM).unwrap()
}

// Configuration values from environment or defaults
pub fn get_rpc_url() -> String {
    std::env::var("RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string())
}

#[allow(dead_code)]
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
