#![allow(unused)]
use bitcoin::hex::DisplayHex;
use bitcoincore_rpc::bitcoin::Amount;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::Write;

// Node access params
const RPC_URL: &str = "http://127.0.0.1:18443"; // Default regtest RPC port
const RPC_USER: &str = "alice";
const RPC_PASS: &str = "password";

// You can use calls not provided in RPC lib API using the generic `call` function.
// An example of using the `send` RPC call, which doesn't have exposed API.
// You can also use serde_json `Deserialize` derivation to capture the returned json result.
fn send(rpc: &Client, addr: &str) -> bitcoincore_rpc::Result<String> {
    let args = [
        json!([{addr : 100 }]), // recipient address
        json!(null),            // conf target
        json!(null),            // estimate mode
        json!(null),            // fee rate in sats/vb
        json!(null),            // Empty option object
    ];

    #[derive(Deserialize)]
    struct SendResult {
        complete: bool,
        txid: String,
    }
    let send_result = rpc.call::<SendResult>("send", &args)?;
    assert!(send_result.complete);
    Ok(send_result.txid)
}

fn main() -> bitcoincore_rpc::Result<()> {
    // Connect to Bitcoin Core RPC
    let rpc = Client::new(
        RPC_URL,
        Auth::UserPass(RPC_USER.to_owned(), RPC_PASS.to_owned()),
    )?;

    // Get blockchain info
    let blockchain_info = rpc.get_blockchain_info()?;
    println!("Blockchain Info: {:?}", blockchain_info);

    // Create/Load the wallets, named 'Miner' and 'Trader'. Have logic to optionally create/load them if they do not exist or not loaded already.
createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup ){}
    // Generate spendable balances in the Miner wallet. How many blocks needs to be mined?

    // Load Trader wallet and generate a new address

    // Send 20 BTC from Miner to Trader

    // Check transaction in mempool

    // Mine 1 block to confirm the transaction

    // Extract all required transaction details

    // Write the data to ../out.txt in the specified format given in readme.md

    Ok(())
}
/*
checklist for the path  way to follow
Create or load wallet Miner
Create or load wallet Trader
Mine enough blocks to the Miner wallet so its balance is spendable — research why newly mined coins aren't spendable immediately (search: "Bitcoin coinbase maturity 100 blocks")
Get a new address from Trader wallet
Send 20 BTC from Miner to Trader
Look at the transaction while unconfirmed (mempool) — get_mempool_entry or similar
Mine 1 more block to confirm it
Call get_transaction on the Miner wallet for that txid
Pull out: the input address/amount (this requires looking at the previous transaction the input references — RPC won't hand you "the input address" directly, you may need decoderawtransaction or gettxout on the previous output)
Identify which output is "change" vs which went to Trader by comparing addresses
Format everything into the 10-line file matching the exact order
















*/
