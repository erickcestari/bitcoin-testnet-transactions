use bitcoin::network::constants::Network;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::address::Address;
use bitcoin::util::key::{PrivateKey, PublicKey};
use rand::Rng;

pub fn generate_random_address_testnet() {
    let network = Network::Testnet;
    let mut rng = rand::thread_rng();
    let private_key_bytes: [u8; 32] = rng.gen();
    let private_key =
        PrivateKey::from_slice(&private_key_bytes, network).expect("Invalid private key");

    let public_key = PublicKey::from_private_key(&Secp256k1::new(), &private_key);

    let address = Address::p2pkh(&public_key, network);

    println!("Testnet Bitcoin Address: {}", address);
    println!("Public key: {}", { public_key });
    println!("Private key: {}", { private_key.to_wif() });
}
