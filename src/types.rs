use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairPrice {
    pub pair: String,
    pub price: f64,
    pub timestamp: u64,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub pool_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdate {
    pub pairs: Vec<PairPrice>,
    pub update_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageAlert {
    pub pair: String,
    pub deviation: f64,
    pub timestamp: u64,
}

// Raydium AMM V4 account structure (simplified)
// Based on https://github.com/raydium-io/raydium-amm
#[derive(Debug)]
pub struct RaydiumAmmInfo {
    pub status: u64,
    pub nonce: u64,
    pub order_num: u64,
    pub depth: u64,
    pub coin_decimals: u64,
    pub pc_decimals: u64,
    pub state: u64,
    pub reset_flag: u64,
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub amount_wave_ratio: u64,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub sys_decimal_value: u64,
    // Key fields for price calculation
    pub pool_coin_token_account: [u8; 32],
    pub pool_pc_token_account: [u8; 32],
    pub coin_mint: [u8; 32],
    pub pc_mint: [u8; 32],
    pub lp_mint: [u8; 32],
    pub open_orders: [u8; 32],
    pub market_id: [u8; 32],
    pub market_program_id: [u8; 32],
    pub target_orders: [u8; 32],
    pub withdraw_queue: [u8; 32],
    pub token_temp_lp: [u8; 32],
    pub amm_owner: [u8; 32],
    pub lp_amount: u64,
}

impl RaydiumAmmInfo {
    pub fn parse_from_account_data(data: &[u8]) -> anyhow::Result<Self> {
        // Raydium AMM V4 uses a packed binary format
        // This is a simplified parser - actual implementation may need adjustments
        if data.len() < 752 {
            anyhow::bail!("Account data too small for Raydium AMM");
        }

        // Parse key fields from the account data
        // Note: This is a basic implementation. You may need to adjust offsets
        // based on the actual Raydium AMM V4 account structure
        
        Ok(Self {
            status: u64::from_le_bytes(data[0..8].try_into()?),
            nonce: u64::from_le_bytes(data[8..16].try_into()?),
            order_num: u64::from_le_bytes(data[16..24].try_into()?),
            depth: u64::from_le_bytes(data[24..32].try_into()?),
            coin_decimals: u64::from_le_bytes(data[32..40].try_into()?),
            pc_decimals: u64::from_le_bytes(data[40..48].try_into()?),
            state: u64::from_le_bytes(data[48..56].try_into()?),
            reset_flag: u64::from_le_bytes(data[56..64].try_into()?),
            min_size: u64::from_le_bytes(data[64..72].try_into()?),
            vol_max_cut_ratio: u64::from_le_bytes(data[72..80].try_into()?),
            amount_wave_ratio: u64::from_le_bytes(data[80..88].try_into()?),
            coin_lot_size: u64::from_le_bytes(data[88..96].try_into()?),
            pc_lot_size: u64::from_le_bytes(data[96..104].try_into()?),
            min_price_multiplier: u64::from_le_bytes(data[104..112].try_into()?),
            max_price_multiplier: u64::from_le_bytes(data[112..120].try_into()?),
            sys_decimal_value: u64::from_le_bytes(data[120..128].try_into()?),
            pool_coin_token_account: data[128..160].try_into()?,
            pool_pc_token_account: data[160..192].try_into()?,
            coin_mint: data[192..224].try_into()?,
            pc_mint: data[224..256].try_into()?,
            lp_mint: data[256..288].try_into()?,
            open_orders: data[288..320].try_into()?,
            market_id: data[320..352].try_into()?,
            market_program_id: data[352..384].try_into()?,
            target_orders: data[384..416].try_into()?,
            withdraw_queue: data[416..448].try_into()?,
            token_temp_lp: data[448..480].try_into()?,
            amm_owner: data[480..512].try_into()?,
            lp_amount: u64::from_le_bytes(data[512..520].try_into()?),
        })
    }
}


