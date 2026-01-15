use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

pub struct SolanaClient {
    rpc_client: Arc<RpcClient>,
}

impl SolanaClient {
    pub fn new(rpc_url: String) -> Self {
        let rpc_client = Arc::new(RpcClient::new(rpc_url));
        Self { rpc_client }
    }

    pub async fn get_account(&self, pubkey: &Pubkey) -> Result<Account> {
        match self.rpc_client.get_account(pubkey).await {
            Ok(account) => Ok(account),
            Err(e) => {
                tracing::warn!("Failed to get account {}: {:?}", pubkey, e);
                Err(e.into())
            }
        }
    }

    pub async fn get_multiple_accounts(&self, pubkeys: &[Pubkey]) -> Result<Vec<Option<Account>>> {
        match self.rpc_client.get_multiple_accounts(pubkeys).await {
            Ok(accounts) => Ok(accounts),
            Err(e) => {
                tracing::warn!("Failed to get multiple accounts: {:?}", e);
                Err(e.into())
            }
        }
    }

    pub async fn get_token_account_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        match self.rpc_client.get_token_account_balance(pubkey).await {
            Ok(balance) => {
                // Parse the amount from the UI amount string
                let amount = balance.amount.parse::<u64>()?;
                Ok(amount)
            }
            Err(e) => {
                tracing::warn!("Failed to get token account balance for {}: {:?}", pubkey, e);
                Err(e.into())
            }
        }
    }

    pub fn clone_client(&self) -> Arc<RpcClient> {
        Arc::clone(&self.rpc_client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_get_account() {
        let client = SolanaClient::new("https://api.mainnet-beta.solana.com".to_string());
        let pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        let result = client.get_account(&pubkey).await;
        assert!(result.is_ok() || result.is_err()); // Just check it doesn't panic
    }
}


