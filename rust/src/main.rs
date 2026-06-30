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
    //creates and loads a new wallet
    //wallet name -  a path, the wallet will be created at the path location
createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup ){}
//setting up wallet
 ensure_wallet_loaded(&rpc, "Miner")?;
 ensure_wallet_loaded(&rpc, "Trader")?;


    // Generate spendable balances in the Miner wallet. How many blocks needs to be mined?

    // Load Trader wallet and generate a new address

    // Send 20 BTC from Miner to Trader

    // Check transaction in mempool

    // Mine 1 block to confirm the transaction
 let mining_address = ...get_new_address with label "Mining Reward" on Miner...
let blocks_mined = mine_until_positive_balance(&miner_rpc, &mining_address)?;
    // send phase
    let trader_address = ...get_new_address with label "Received" on Trader...
    let txid = miner_rpc.send_to_address(...)?;
        // mempool check
    let mempool_entry = miner_rpc.get_mempool_entry(&txid)?;
    println!("{:?}", mempool_entry);
        // confirm
    miner_rpc.generate_to_address(1, &mining_address)?;
    // Extract all required transaction details
 // extract details
    let tx = miner_rpc.get_transaction(&txid, None)?;
    let decoded = miner_rpc.get_raw_transaction_info(&txid, None)?;
        // trace input
    let prev_txid = decoded.vin[0].txid...;
    let prev_tx = miner_rpc.get_raw_transaction_info(&prev_txid, None)?;
    // match vout index to get input address + amount
    // Write the data to ../out.txt in the specified format given in readme.md
// identify trader vout vs change vout by comparing addresses
    // write out.txt
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
