#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};

// Structure to store asset information
#[contracttype]
#[derive(Clone)]
pub struct AssetInfo {
    pub asset_code: String,
    pub issuer: Address,
    pub total_supply: i128,
    pub description: String,
    pub is_active: bool,
    pub registration_time: u64,
}

// Mapping asset code to AssetInfo
#[contracttype]
pub enum AssetBook {
    Asset(String)
}

// Counter for total registered assets
const ASSET_COUNT: Symbol = symbol_short!("A_COUNT");

#[contract]
pub struct AssetExplorerContract;

#[contractimpl]
impl AssetExplorerContract {
    
    // Function to register a new asset on the explorer
    pub fn register_asset(
        env: Env, 
        asset_code: String, 
        issuer: Address,
        total_supply: i128,
        description: String
    ) -> bool {
        
        // Check if asset already exists
        let existing_asset = Self::get_asset_info(env.clone(), asset_code.clone());
        
        if existing_asset.is_active {
            log!(&env, "Asset already registered: {}", asset_code);
            panic!("Asset already exists!");
        }
        
        // Get current timestamp
        let time = env.ledger().timestamp();
        
        // Create new asset info
        let new_asset = AssetInfo {
            asset_code: asset_code.clone(),
            issuer: issuer.clone(),
            total_supply,
            description,
            is_active: true,
            registration_time: time,
        };
        
        // Store asset information
        env.storage().instance().set(&AssetBook::Asset(asset_code.clone()), &new_asset);
        
        // Update asset count
        let mut count: u64 = env.storage().instance().get(&ASSET_COUNT).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&ASSET_COUNT, &count);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Asset registered successfully: {}", asset_code);
        true
    }
    
    // Function to retrieve asset information by asset code
    pub fn get_asset_info(env: Env, asset_code: String) -> AssetInfo {
        let key = AssetBook::Asset(asset_code.clone());
        
        env.storage().instance().get(&key).unwrap_or(AssetInfo {
            asset_code: String::from_str(&env, "NOT_FOUND"),
            issuer: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            total_supply: 0,
            description: String::from_str(&env, "Asset not found"),
            is_active: false,
            registration_time: 0,
        })
    }
    
    // Function to update asset supply
    pub fn update_asset_supply(env: Env, asset_code: String, new_supply: i128) -> bool {
        let mut asset = Self::get_asset_info(env.clone(), asset_code.clone());
        
        if !asset.is_active {
            log!(&env, "Asset not found: {}", asset_code);
            panic!("Asset does not exist!");
        }
        
        asset.total_supply = new_supply;
        
        env.storage().instance().set(&AssetBook::Asset(asset_code.clone()), &asset);
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Asset supply updated for: {}", asset_code);
        true
    }
    
    // Function to get total number of registered assets
    pub fn get_total_assets(env: Env) -> u64 {
        env.storage().instance().get(&ASSET_COUNT).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_register_and_get_asset() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AssetExplorerContract);
        let client = AssetExplorerContractClient::new(&env, &contract_id);
        
        let asset_code = String::from_str(&env, "USDC");
        let issuer = Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"));
        let description = String::from_str(&env, "USD Coin");
        
        // Register asset
        let result = client.register_asset(&asset_code, &issuer, &1000000, &description);
        assert_eq!(result, true);
        
        // Get asset info
        let asset_info = client.get_asset_info(&asset_code);
        assert_eq!(asset_info.asset_code, asset_code);
        assert_eq!(asset_info.total_supply, 1000000);
    }
}