#!/usr/bin/env python3
"""
Fetch real Raydium pool addresses from Raydium API
This script will help you find the correct pool addresses for your config
"""

import requests
import json

def fetch_raydium_pools():
    """Fetch pool data from Raydium API"""
    print("Fetching Raydium pools...")
    try:
        # Raydium API endpoint for pool info
        url = "https://api.raydium.io/v2/main/pairs"
        response = requests.get(url, timeout=30)
        response.raise_for_status()
        return response.json()
    except Exception as e:
        print(f"Error fetching pools: {e}")
        return None

def find_usdc_pools(pools, tokens):
    """Find USDC pools for specific tokens"""
    results = {}
    
    if not pools:
        return results
    
    for token in tokens:
        for pool in pools:
            pool_name = pool.get('name', '')
            # Look for token-USDC pairs
            if token in pool_name and 'USDC' in pool_name:
                if token not in results:  # Only take the first match
                    results[token] = {
                        'name': pool_name,
                        'address': pool.get('ammId', ''),
                        'liquidity': pool.get('liquidity', 0),
                    }
                    break
    
    return results

def main():
    tokens = ['BONK', 'WIF', 'POPCAT', 'GIGA', 'PNUT', 'MEW', 'PENGU', 'AI16Z', 'FARTCOIN', 'MOTHER']
    
    pools = fetch_raydium_pools()
    
    if not pools:
        print("\n❌ Failed to fetch pools from Raydium API")
        print("\n📝 Manual steps to find pool addresses:")
        print("1. Visit https://birdeye.so")
        print("2. Search for each token")
        print("3. Find the Raydium USDC pool")
        print("4. Copy the pool address")
        return
    
    print(f"\n✅ Fetched {len(pools)} pools from Raydium")
    
    usdc_pools = find_usdc_pools(pools, tokens)
    
    print("\n" + "="*70)
    print("FOUND RAYDIUM POOLS")
    print("="*70 + "\n")
    
    if not usdc_pools:
        print("❌ No matching pools found. Try manual search.")
        return
    
    for token in tokens:
        if token in usdc_pools:
            pool = usdc_pools[token]
            print(f"✅ {token}-USDC:")
            print(f"   Pool Name: {pool['name']}")
            print(f"   Address: {pool['address']}")
            print(f"   Liquidity: ${pool['liquidity']:,.2f}")
            print()
        else:
            print(f"❌ {token}-USDC: Not found")
            print()
    
    # Generate Rust config code
    print("\n" + "="*70)
    print("RUST CONFIG CODE")
    print("="*70 + "\n")
    
    for token in tokens:
        if token in usdc_pools:
            pool = usdc_pools[token]
            print(f'''        PoolConfig {{
            name: "{token}-USDC".to_string(),
            address: Pubkey::from_str("{pool['address']}").unwrap(),
        }},''')

if __name__ == "__main__":
    main()

