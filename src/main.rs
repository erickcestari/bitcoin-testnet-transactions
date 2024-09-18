use std::str::FromStr;

use bitcoin::blockdata::script::{Builder, Script};
use bitcoin::blockdata::transaction::{SigHashType, Transaction, TxIn, TxOut};
use bitcoin::consensus::encode;
use bitcoin::hashes::hex::FromHex;
use bitcoin::hashes::sha256d;
use bitcoin::secp256k1::{Message, Secp256k1};
use bitcoin::util::address::Address;
use bitcoin::util::bip143::SigHashCache;
use bitcoin::util::key::{PrivateKey, PublicKey};
use reqwest::Client;
use serde_json::json;

mod generate_address_testnet;

#[tokio::main]
async fn main() {
    let sender_address = Address::from_str("mgLghdorxWmxr4AqmMXeaTUGAfvP81AfBh").unwrap();
    let recipient_address =
        Address::from_str("tb1qn9rvr53m7qvrpysx48svuxsgahs88xfsskx367").unwrap();

    let private_key_wif = "cSP6oqRguYh1e1hEKNJ4jwL5kUVQhJVKQzm8LniZihEyySeiJR15";
    let private_key = PrivateKey::from_wif(private_key_wif).unwrap();

    let public_key = PublicKey::from_private_key(&Secp256k1::new(), &private_key);

    let txid =
        sha256d::Hash::from_hex("748440e752db2d146b5b935ef58afc03b4432c7767c26cebc7f381cf371a7be4")
            .unwrap()
            .into();
    let vout = 0;
    let utxo_value = 10_000;

    let txin = TxIn {
        previous_output: bitcoin::OutPoint { txid, vout },
        script_sig: Script::new(),
        sequence: 0xFFFFFFFF,
        witness: vec![],
    };

    let fee = 1000;

    let recipient_value = 5_000;
    let txout_recipient = TxOut {
        value: recipient_value,
        script_pubkey: recipient_address.script_pubkey(),
    };

    let change_value = utxo_value - recipient_value - fee;
    let txout_change = TxOut {
        value: change_value,
        script_pubkey: sender_address.script_pubkey(),
    };

    let mut tx = Transaction {
        version: 2,
        lock_time: 0,
        input: vec![txin],
        output: vec![txout_recipient, txout_change],
    };

    // Sign the transaction
    let sighash = SigHashCache::new(&tx).signature_hash(
        0,
        &sender_address.script_pubkey(),
        utxo_value,
        SigHashType::All,
    );

    let secp = Secp256k1::new();
    let message = Message::from_slice(&sighash[..]).unwrap();
    let signature = secp.sign(&message, &private_key.key);
    let mut sig_with_hash_type = signature.serialize_der().to_vec();
    sig_with_hash_type.push(SigHashType::All as u8);

    tx.input[0].script_sig = Builder::new()
        .push_slice(&sig_with_hash_type)
        .push_slice(&public_key.to_bytes())
        .into_script();

    let raw_tx = encode::serialize_hex(&tx);
    let txid = tx.txid();
    println!("Raw signed transaction: {}", raw_tx);
    println!("Transaction ID: {}", txid);

    println!("ScriptPubKey: {:?}", sender_address.script_pubkey());
    println!("Sighash: {:?}", sighash);

    // Send the transaction to the Bitcoin Testnet network
    let client = Client::new();
    let response = client
        .post("https://api.bitcore.io/api/BTC/testnet/tx/send")
        .json(&json!({ "rawTx": raw_tx }))
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        println!("Transaction sent successfully!");
    } else {
        println!(
            "Failed to send transaction: {:?}",
            response.text().await.unwrap()
        );
    }
}
