#![allow(unused)]
use bitcoin::hex::DisplayHex;
use bitcoincore_rpc::bitcoin::Amount;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::Write;

/*imports
constants
helper function: ensure_wallet_loaded()
main() {
    1. connect (base rpc)
    2. ensure wallets
    3. wallet-scoped clients
    4. mining address + mine 101 blocks
    5. print balance + comment
    6. trader address
    7. send 20 BTC
    8. check mempool
    9. mine 1 block to confirm
    10. get tx details
    11. trace input source
    12. identify trader vs change output
    13. build TxOutput struct
    14. write out.txt
}
struct TxOutput { all 10 fields }
impl TxOutput { fn write_to_file() } */

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
//When you call an RPC method, you can scope it to a particular wallet — so the command only affects that wallet’s data.
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

//work
fn list_wallets(&self) -> Result<Vec<String>> { ... }

//work
fn load_wallet(&self, wallet: &str) -> Result<LoadWalletResult> { ... }//load wallet

//work
    fn create_wallet(
        &self,
        wallet: &str,
        disable_private_keys: Option<bool>,
        blank: Option<bool>,
        passphrase: Option<&str>,
        avoid_reuse: Option<bool>,
    ) -> Result<LoadWalletResult> { ... }
//work
    let miner_rpc = Client::new(
    &format!("{}/wallet/Miner", RPC_URL),
    Auth::UserPass(RPC_USER.to_owned(), RPC_PASS.to_owned()),
)?;
// same pattern for trader_rpc

    let tx = miner_rpc.get_transaction(&txid, None)?;
    let decoded = miner_rpc.get_raw_transaction_info(&txid, None)?;
//work
let mining_address = miner_rpc.get_new_address(Some("Mining Reward"), None)?
    .require_network(bitcoin::Network::Regtest)?;
//work
miner_rpc.generate_to_address(101, &mining_address)?;
//work
let balance = miner_rpc.get_balance(None, None)?;
println!("Miner balance: {}", balance);
// Bitcoin coinbase maturity rule: newly mined block rewards are locked
// for 100 confirmations before they become spendable. This means after
// mining block N, the reward from block N only unlocks at block N+100.
// So we need to mine at least 101 blocks before the FIRST reward is spendable.

//work
send_to_address(
    &self,
    address: &Address<NetworkChecked>,
    amount: Amount,
    comment: Option<&str>,
    comment_to: Option<&str>,
    subtract_fee_from_amount: Option<bool>,
    replaceable: Option<bool>,
    confirmation_target: Option<u32>,
    estimate_mode: Option<EstimateMode>,
) -> Result<Txid>
//work
let trader_address = trader_rpc.get_new_address(Some("Received"), None)?
    .require_network(bitcoin::Network::Regtest)?;

let txid = miner_rpc.send_to_address(
    &trader_address,
    Amount::from_btc(20.0)?,
    None, None, None, None, None, None
)?;
println!("Sent txid: {}", txid);

//crate details
// send_to_address(
//     &self,
//     address: &Address<NetworkChecked>,
//     amount: Amount,
//     comment: Option<&str>,
//     comment_to: Option<&str>,
//     subtract_fee_from_amount: Option<bool>,
//     replaceable: Option<bool>,
//     confirmation_target: Option<u32>,
//     estimate_mode: Option<EstimateMode>,
// ) -> Result<Txid>
//work
let trader_address = trader_rpc.get_new_address(Some("Received"), None)?
    .require_network(bitcoin::Network::Regtest)?;

let txid = miner_rpc.send_to_address(
    &trader_address,
    Amount::from_btc(20.0)?,
    None, None, None, None, None, None
)?;
println!("Sent txid: {}", txid);
//fee, details, blockhash, blockheight fields at the top level, and inside decoded → vin and vout arrays
///verifying with "F:\BITCOIN_CORE\bitcoin\bin\bitcoin-cli.exe" -regtest -rpcuser=alice -rpcpassword=password -rpcwallet=Miner gettransaction <YOUR_TXID>

//work
//fetching unconfirmed transaction from mempool
let mempool_entry = rpc.get_mempool_entry(&txid)?;
println!("Mempool entry: {:?}", mempool_entry);

//work
let tx_info = miner_rpc.get_transaction(&txid, Some(true))?;
let fee = tx_info.fee.unwrap();           // negative number = fee paid
let block_hash = tx_info.info.blockhash.unwrap();
let block_height = tx_info.info.blockheight.unwrap();

//work
let raw_tx = miner_rpc.get_raw_transaction_info(&txid, Some(&block_hash))?;
// raw_tx.vout is a Vec of outputs — each has .value (Amount) and .script_pub_key.address
// raw_tx.vin is a Vec of inputs — each has .txid and .vout (the output index of prev tx)


//work - finding the input source address
//looking up and getting input source address
let prev_txid = raw_tx.vin[0].txid.unwrap();
let prev_vout_index = raw_tx.vin[0].vout.unwrap() as usize;

let prev_tx = miner_rpc.get_raw_transaction_info(&prev_txid, None)?;
let input_output = &prev_tx.vout[prev_vout_index];

let miner_input_amount = input_output.value;
let miner_input_address = input_output.script_pub_key.address.as_ref().unwrap();

//work -IDENTIFYING trader's output vs miner's change
for output in &raw_tx.vout {
    let addr = output.script_pub_key.address.as_ref().unwrap();
    if addr == &trader_address {
        // this is trader output
    } else {
        // this is miner change
    }
}

// Mine 1 block to confirm
miner_rpc.generate_to_address(1, &mining_address)?;

        // trace input
    let prev_txid = decoded.vin[0].txid...;
    let prev_tx = miner_rpc.get_raw_transaction_info(&prev_txid, None)?;
    // match vout index to get input address + amount
    // Write the data to ../out.txt in the specified format given in readme.md
// identify trader vout vs change vout by comparing addresses
    // write out.txt
//work - Rust file I/O.
let mut file = File::create("out.txt")?;
writeln!(file, "{}", txid)?;
writeln!(file, "{}", miner_input_address)?;
writeln!(file, "{}", miner_input_amount.to_btc())?;
writeln!(file, "{}", trader_address)?;
writeln!(file, "{}", trader_amount.to_btc())?;
writeln!(file, "{}", miner_change_address)?;
writeln!(file, "{}", miner_change_amount.to_btc())?;
writeln!(file, "{}", fee.to_btc())?;     // will be negative, test takes abs()
writeln!(file, "{}", block_height)?;
writeln!(file, "{}", block_hash)?;

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
