use std::str::FromStr;

use bitcoin::hashes::hex::{FromHex, ToHex};
use bitcoin::hashes::{ripemd160, sha256, Hash};
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::address::Address;
use bitcoin::util::key::{PrivateKey, PublicKey};
use rand::Rng;

pub fn generate_address_testnet() {
    let network = Network::Testnet;
    // Generate a random 32-byte private key
    let mut rng = rand::thread_rng();
    let private_key_bytes: [u8; 32] = rng.gen();
    let private_key = PrivateKey::from_wif("cSP6oqRguYh1e1hEKNJ4jwL5kUVQhJVKQzm8LniZihEyySeiJR15")
        .expect("Invalid private key");

    println!("Private key: {}", { private_key.to_wif() });
    // Generate the public key from the private key
    let public_key = PublicKey::from_private_key(&Secp256k1::new(), &private_key);
    println!("Public key: {}", { public_key });

    // Create a testnet address directly from the public key
    let address = Address::p2pkh(&public_key, network);

    // Print the address
    println!("Testnet Bitcoin Address: {}", address);

    match Address::from_str(address.to_string().as_str()) {
        Ok(address) => {
            if address.network == Network::Testnet {
                println!("The address is a valid testnet Bitcoin address.");
            } else {
                println!("The address is not a testnet Bitcoin address.");
            }
        }
        Err(e) => println!("Invalid address: {}", e),
    }
}
