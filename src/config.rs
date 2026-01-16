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
    // 🏆 WORKING RAYDIUM STANDARD AMM V4 POOLS
    // ✅ All addresses verified as Raydium AMM V4 (675kPX...)
    // Program: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
    // Account Size: 752 bytes
    // Updated: 2026-01-17
    
    vec![
        // ✅ VERIFIED WORKING POOLS
        
        // Token 1: BONK - Most popular Solana meme coin
        PoolConfig {
            name: "BONK-USDC".to_string(),
            address: Pubkey::from_str("Dwq4PxyBQ8dHPmP5u5H7bHsjHp46StGtkSy2gEVedDm").unwrap(),
        },
        
        // Token 2: MEW - Cat in a dogs world
        PoolConfig {
            name: "MEW-SOL".to_string(),
            address: Pubkey::from_str("879F697iuDJGMevRkRcnW21fcXiAeLJK1ffsw2ATebce").unwrap(),
        },
        
        // Token 3: RAY - Raydium's native token
        PoolConfig {
            name: "RAY-USDC".to_string(),
            address: Pubkey::from_str("6UmmUiYoBjSrhakAobJw8BvkmJtDVxaeBtbt7rxWo1mg").unwrap(),
        },
        
        // Token 4: SOL - Native Solana token
        PoolConfig {
            name: "SOL-USDC".to_string(),
            address: Pubkey::from_str("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2").unwrap(),
        },
        
        
        // 🔍 TO ADD MORE TOKENS:
        // 1. Go to https://dexscreener.com
        // 2. Search your token
        // 3. Find "Raydium" pool (NOT "Raydium CLMM")
        // 4. Copy pool address from URL
        // 5. Verify with: solana account ADDRESS
        // 6. Check Owner is: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
        // 7. Check Data Length: 752
        
        // ⚠️ REMOVED NON-WORKING POOLS:
        // The following tokens likely ONLY have CLMM pools (not Standard AMM):
        // - WIF, POPCAT, PYTH, PNUT, JTO, PENGU, FARTCOIN, JUP
        //
        // To monitor these, you need to either:
        // A) Find their Standard AMM pools on DexScreener (if they exist)
        // B) Add CLMM support to this app (requires significant changes)
        //
        // See FIND_WORKING_POOLS.md for detailed instructions
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
