#![allow(unused)]
//use bitcoin::hex::DisplayHex;
use bitcoincore_rpc::bitcoin::{Amount, Network}; //Network::Regtest is used when calling .require_network() on addresses
use bitcoincore_rpc::{Auth, Client, RpcApi};
// use serde::Deserialize;
// use serde_json::json;
use std::fs::File;
use std::io::Write;

//setting up output struct and also implementing respective function to write struct
struct TxOutput {
    txid: String,
    miner_input_address: String,
    miner_input_amount: f64,
    trader_address: String,
    trader_amount: f64,
    miner_change_address: String,
    miner_change_amount: f64,
    fee: f64,
    block_height: u64, //so that it can never be negative
    block_hash: String,
}

impl TxOutput {
    // &self = borrow this struct without consuming it
    // std::io::Result is separate from bitcoincore_rpc::Result
    fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "{}", self.txid)?;
        writeln!(file, "{}", self.miner_input_address)?;
        writeln!(file, "{}", self.miner_input_amount)?;
        writeln!(file, "{}", self.trader_address)?;
        writeln!(file, "{}", self.trader_amount)?;
        writeln!(file, "{}", self.miner_change_address)?;
        writeln!(file, "{}", self.miner_change_amount)?;
        writeln!(file, "{}", self.fee)?;
        writeln!(file, "{}", self.block_height)?;
        write!(file, "{}", self.block_hash)?;
        Ok(())
    }
}

// Node access params
const RPC_URL: &str = "http://127.0.0.1:18443"; // Default regtest RPC port
const RPC_USER: &str = "alice";
const RPC_PASS: &str = "password";

// You can use calls not provided in RPC lib API using the generic `call` function.
// An example of using the `send` RPC call, which doesn't have exposed API.
// You can also use serde_json `Deserialize` derivation to capture the returned json result.

fn ensure_wallet_loaded(rpc: &Client, name: &str) -> bitcoincore_rpc::Result<()> {
    // list_wallets() returns the names of wallets currently loaded in memory
    // docs: https://developer.bitcoin.org/reference/rpc/listwallets.html
    let loaded = rpc.list_wallets()?;

    if loaded.contains(&name.to_string()) {
        // Already loaded, nothing to do
        println!("Wallet '{}' is already loaded.", name);
        return Ok(());
    }
    // Bitcoin Core errors if you create_wallet on something that already exists. And it errors if you load_wallet something that doesn't exist on disk. So you check the loaded list first, then try load, then fall back to create.
    // Try loading from disk first — wallet may exist but not be active
    // docs: https://developer.bitcoin.org/reference/rpc/loadwallet.html
    match rpc.load_wallet(name) {
        Ok(_) => {
            println!("Wallet '{}' loaded from disk.", name);
        }
        Err(_) => {
            // Wallet doesn't exist yet — create it fresh
            // None params: disable_private_keys, blank, passphrase, avoid_reuse
            // docs: https://developer.bitcoin.org/reference/rpc/createwallet.html
            rpc.create_wallet(name, None, None, None, None)?;
            println!("Wallet '{}' created.", name);
        }
    }
    Ok(())
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
    // createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup ){}

    //

    fn main() -> bitcoincore_rpc::Result<()> {
        //connection to base RPC
        let rpc = Client::new(
            RPC_URL,
            Auth::UserPass(RPC_USER.to_owned(), RPC_PASS.to_owned()),
        )?;

        let blockchain_info = rpc.get_blockchain_info()?;
        println!("Chain: {}", blockchain_info.chain);

        //creating and loading the wallets
        ensure_wallet_loaded(&rpc, "Miner")?;
        ensure_wallet_loaded(&rpc, "Trader")?;

        //setting up wallet scoped clients
        // Each wallet needs its own client pointed at /wallet/<name>
        // This scopes all RPC calls to that specific wallet
        // docs: wallet URL format http://host:port/wallet/<name>
        let miner_rpc = Client::new(
            &format!("{}/wallet/Miner", RPC_URL),
            Auth::UserPass(RPC_USER.to_owned(), RPC_PASS.to_owned()),
        )?;

        let trader_rpc = Client::new(
            &format!("{}/wallet/Trader", RPC_URL),
            Auth::UserPass(RPC_USER.to_owned(), RPC_PASS.to_owned()),
        )?;

        //mining address +101 blocks
        // Generate a new address in Miner wallet labeled "Mining Reward"
        // require_network() ensures this address is valid for regtest
        // docs: https://developer.bitcoin.org/reference/rpc/getnewaddress.html
        let mining_address = miner_rpc
            .get_new_address(Some("Mining Reward"), None)?
            .require_network(Network::Regtest)?;

        // Bitcoin coinbase maturity rule: block rewards are locked for 100
        // confirmations after they are mined. A miner receives the reward in
        // block N, but cannot spend it until block N+100 is also mined.
        // Therefore we mine 101 blocks so the very first reward becomes spendable.
        // docs: https://developer.bitcoin.org/reference/rpc/generatetoaddress.html
        miner_rpc.generate_to_address(101, &mining_address)?;

        let balance = miner_rpc.get_balance(None, None)?;
        println!("Miner balance after mining: {} BTC", balance.to_btc());

        //setting up the trader address
        // Generate receiving address in Trader wallet labeled "Received"
        let trader_address = trader_rpc
            .get_new_address(Some("Received"), None)?
            .require_network(Network::Regtest)?;

        // Generate spendable balances in the Miner wallet. How many blocks needs to be mined?

        // Load Trader wallet and generate a new address

        // Send 20 BTC from Miner to Trader
        // Send 20 BTC from Miner to Trader
        // Amount::from_btc returns a Result so we use ? to unwrap
        // docs: https://developer.bitcoin.org/reference/rpc/sendtoaddress.html
        let txid = miner_rpc.send_to_address(
            &trader_address,
            Amount::from_btc(20.0)?,
            None, // comment
            None, // comment_to
            None, // subtract_fee_from_amount
            None, // replaceable
            None, // confirmation_target
            None, // estimate_mode
        )?;
        println!("Transaction sent: {}", txid);

        //memory pool chack
        // Fetch the transaction from the mempool while it is still unconfirmed
        // The mempool is node-wide so we use the base rpc, not wallet-scoped
        // docs: https://developer.bitcoin.org/reference/rpc/getmempoolentry.html
        let mempool_entry = rpc.get_mempool_entry(&txid)?;
        println!("Mempool entry fees: {:?}", mempool_entry.fees);

        // Mine 1 block to confirm the transaction
        miner_rpc.generate_to_address(1, &mining_address)?;
        println!("Transaction confirmed.");

        //extracting tx details
        // get_transaction gives us top-level fields: fee, blockhash, blockheight
        // passing Some(true) means include watch-only addresses
        // docs: https://developer.bitcoin.org/reference/rpc/gettransaction.html
        let tx_info = miner_rpc.get_transaction(&txid, Some(true))?;
        let fee = tx_info.fee.expect("fee must exist for confirmed tx");
        let block_hash = tx_info.info.blockhash.expect("blockhash must exist");
        let block_height = tx_info.info.blockheight.expect("blockheight must exist");
        // get_raw_transaction_info gives us the decoded vin/vout structure
        // We pass block_hash as a hint so the node finds it faster
        // docs: https://developer.bitcoin.org/reference/rpc/getrawtransaction.html
        let raw_tx = miner_rpc.get_raw_transaction_info(&txid, Some(&block_hash))?;

        //Trace the input back to its source
        // vin[0].txid is the previous transaction this input is spending from
        // vin[0].vout is which output index of that previous transaction
        // We must look up that previous tx to find the actual address and amount
        let prev_txid = raw_tx.vin[0].txid.expect("vin txid must exist");
        let prev_vout_index = raw_tx.vin[0].vout.expect("vin vout index must exist") as usize;

        let prev_tx = miner_rpc.get_raw_transaction_info(&prev_txid, None)?;
        let input_source = &prev_tx.vout[prev_vout_index];

        let miner_input_amount = input_source.value;
        // .clone() copies the Option<Address> so we can call methods on it
        // .assume_checked() assures Rust that this address is already network-validated"
        let miner_input_address = input_source
            .script_pub_key
            .address
            .clone()
            .expect("input address must exist")
            .assume_checked();

        //identifying trader output
        // The transaction has 2 outputs: one to Trader, one as change back to Miner
        // We identify them by comparing each output address to trader_address
        let mut trader_amount = Amount::ZERO;
        let mut miner_change_address = trader_address.clone(); // placeholder
        let mut miner_change_amount = Amount::ZERO;

        for output in &raw_tx.vout {
            let addr = output
                .script_pub_key
                .address
                .clone()
                .expect("output address must exist")
                .assume_checked();

            if addr == trader_address {
                trader_amount = output.value;
            } else {
                miner_change_address = addr;
                miner_change_amount = output.value;
            }
        }
        //setting up my output struct
        let output = TxOutput {
            txid: txid.to_string(),
            miner_input_address: miner_input_address.to_string(),
            miner_input_amount: miner_input_amount.to_btc(),
            trader_address: trader_address.to_string(),
            trader_amount: trader_amount.to_btc(),
            miner_change_address: miner_change_address.to_string(),
            miner_change_amount: miner_change_amount.to_btc(),
            fee: fee.to_btc(), // negative value — test.spec.ts takes abs()
            block_height,
            block_hash: block_hash.to_string(),
        };
        //writing struct content to out.txt
        output.write_to_file("../out.txt")?;
        println!("Output written to ../out.txt");

        Ok(())
    }
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
