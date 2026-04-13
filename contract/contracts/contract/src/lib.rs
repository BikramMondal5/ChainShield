#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Map, String};

#[contract]
pub struct CounterfeitDetection;

#[contractimpl]
impl CounterfeitDetection {

    // Register a product with unique ID and metadata
    pub fn register_product(env: Env, product_id: String, metadata: String) {
        let key = symbol_short!("PRODUCTS");

        let mut products: Map<String, String> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        // Prevent duplicate entries
        if products.contains_key(product_id.clone()) {
            panic!("Product already registered");
        }

        products.set(product_id, metadata);
        env.storage().instance().set(&key, &products);
    }

    // Verify if a product exists
    pub fn verify_product(env: Env, product_id: String) -> bool {
        let key = symbol_short!("PRODUCTS");

        let products: Map<String, String> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        products.contains_key(product_id)
    }

    // Get product details
    pub fn get_product(env: Env, product_id: String) -> Option<String> {
        let key = symbol_short!("PRODUCTS");

        let products: Map<String, String> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        products.get(product_id)
    }
}