use axum::http::StatusCode;
use ethers::types::{H160, U256};
use ethers::utils::keccak256;
use secp256k1::{
    rand::{rngs, SeedableRng},
    Secp256k1, SecretKey,
};
use std::convert::TryInto;

pub fn calculate_amounts_from_percentages(
    percentages: &[u128],
    total_amount: u128,
) -> (StatusCode, Result<Vec<u128>, String>) {
    if percentages.iter().sum::<u128>() != 100 {
        return (
            StatusCode::BAD_REQUEST,
            Err("Sum of percentages must be 100".into()),
        );
    }

    let amounts: Vec<u128> = percentages
        .iter()
        .map(|&percent| total_amount * percent / 100) // approximate distribution
        .collect();

    (StatusCode::OK, Ok(amounts))
}

pub fn sum_u256_vector(amounts: Vec<U256>) -> U256 {
    let mut sum = U256::zero();

    for amount in amounts {
        sum = sum.saturating_add(amount);
    }

    sum
}

pub fn get_solidity_address(s: String) -> H160 {
    let address_str = s.trim_start_matches("0x");
    let address_bytes = hex::decode(address_str).expect("Invalid hex string");

    let address_bytes: [u8; 20] = address_bytes
        .as_slice()
        .try_into()
        .expect("Hex string must be exactly 20 bytes");

    H160::from(address_bytes)
}

fn generate_ethereum_keypair() -> (SecretKey, String) {
    let secp = Secp256k1::new();
    let random_seed = 12345;
    let mut rng = rngs::StdRng::seed_from_u64(random_seed);

    // Generate the keypair
    let (private_key, public_key) = secp.generate_keypair(&mut rng);

    // Get the public key as bytes (uncompressed form)
    let public_key = public_key.serialize_uncompressed();

    // Hash the public key with Keccak-256 and take the last 20 bytes
    let address_hash = keccak256(&public_key[1..]); // Skip the first byte which indicates the format
    let ethereum_address = format!("0x{}", hex::encode(&address_hash[12..])); // Take the last 20 bytes

    (private_key, ethereum_address)
}

pub fn generate_addresses(amount: u8) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut accounts: Vec<String> = Vec::new();

    for _ in 0..amount {
        let (_, ethereum_address) = generate_ethereum_keypair();
        accounts.push(ethereum_address);
    }

    Ok(accounts)
}
