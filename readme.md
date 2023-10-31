## BDK example with Esplora API

This is a basic example of using the [BDK]() crate with the Blockstream Esplora REST API.

You'll need a recent version of Rust/Cargo, then just: `cargo run`

Example output:

```
Setup Esplora API source: https://blockstream.info/testnet/api
Setup key-value database at wallet/db.dat
Create a Testnet descriptor wallet
Get the last unused address
Address: tb1qnrfpnfyruvhgqn09qs7mpr5cd6k6j4g6fm9h3c
Sync the wallet from the blockchain
Get the wallet balance
Descriptor balance (satoshis): { immature: 0, trusted_pending: 0, untrusted_pending: 0, confirmed: 7553220 }
```

**You might get an Esplora 429 error if you get rate-limited.**
