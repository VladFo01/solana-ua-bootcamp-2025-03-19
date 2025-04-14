use dotenv::dotenv;
use serde_json;
use solana_sdk::signature::{Keypair, Signer};

pub fn load_keypair() -> (String, [u8; 64]) {
    dotenv().ok();

    let private_key_str = std::env::var("PK").expect("PK not found in .env file");
    let private_key_bytes: Vec<u8> = serde_json::from_str(&private_key_str).expect("Invalid PK format");

    if private_key_bytes.len() != 64 {
        panic!("Invalid private key length");
    }

    let keypair = Keypair::from_bytes(&private_key_bytes).unwrap();

    let public_key = keypair.pubkey().to_string();
    let secret_bytes = keypair.to_bytes();

    (public_key, secret_bytes)
}