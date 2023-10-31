use bdk::bitcoin::Network;
use bdk::blockchain::EsploraBlockchain;
use bdk::sled;
use bdk::wallet::AddressIndex;
use bdk::SyncOptions;
use bdk::Wallet;

fn main() -> Result<(), bdk::Error> {
    let base_url = "https://blockstream.info/testnet/api";

    println!("Setup Esplora API source: {}", base_url);
    let blockchain = EsploraBlockchain::new(base_url, 20);

    println!("Setup key-value database at wallet/db.dat");
    let database = sled::open("wallet")?.open_tree("db.dat")?;

    println!("Create a Testnet descriptor wallet");
    let wallet = Wallet::new(
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
        Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
        Network::Testnet,
        database,
    )?;

    println!("Get the last unused address");
    let address = wallet.get_address(AddressIndex::LastUnused)?;
    println!("Address: {}", address);

    println!("Sync the wallet from the blockchain");
    wallet.sync(&blockchain, SyncOptions::default())?;

    println!("Get the wallet balance");
    let balance = wallet.get_balance()?;

    println!("Descriptor balance (satoshis): {}", balance);

    Ok(())
}
