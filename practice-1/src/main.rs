mod generate_keypair;
use generate_keypair::generate_keypair;

mod load_keypair;
use load_keypair::load_keypair;

mod check_balance;
use check_balance::check_balance;

fn main() {
    let keypair = generate_keypair();
    println!("Generated Keypair: {:?}", keypair);

    let (public_key, private_key) = load_keypair();
    
    println!("Public Key: {}", public_key);
    println!("Private Key: {:?}", private_key);

    let balance = check_balance(&public_key);
    println!("Balance: {} SOL", balance);
}
