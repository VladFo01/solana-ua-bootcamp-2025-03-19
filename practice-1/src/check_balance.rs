use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey
};
use solana_client::rpc_client::RpcClient;

pub fn check_balance(pubkey_str: &String) -> u64 {
    let pubkey = Pubkey::from_str_const(pubkey_str);

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // println!("Requesting airdrop of 1 SOL to {}", pubkey);
    // let sig = client.request_airdrop(&pubkey, 1 * LAMPORTS_PER_SOL).unwrap();
    // client.confirm_transaction(&sig).expect("Failed to confirm transaction");

    let balance = client.get_balance(&pubkey).unwrap() / LAMPORTS_PER_SOL;

    balance
}