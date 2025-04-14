use solana_sdk::signature::{Keypair, Signer};

pub fn generate_keypair() -> (String, [u8; 64]) {
    let keypair = Keypair::new();

    let public_key = keypair.pubkey().to_string();
    let secret_bytes = keypair.to_bytes();

    (public_key, secret_bytes)
}