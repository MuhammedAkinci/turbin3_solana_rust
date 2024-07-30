# Turbin3 Builders Course (Rust)

Here is my TX: https://explorer.solana.com/tx/3PeR6q7KKSKwViw9LjGDe1mqs9LETZ2bTSKHvkZx9fwF63SsQhnoM4McMN4Kf3QbyuwkP3BY2ZUSFYEKu7wvdimc?cluster=devnet

In this repo, development has been done using Solana's Rust libraries. This project is a Rust program that provides wallet creation on the Solana blockchain, receiving and transferring SOL tokens and interaction with programs.

# Generate Keypair
Creates a new Solana wallet and saves its private key in JSON format to dev-wallet.json. Base58 transformations are used to securely store and share private keys. 
`cargo test keygen -- --nocapture` command creates a new key pair and prints the public/private keys. Paste the secret key into wallets/dev-wallet.json.

# Airdrop
The `cargo test airdrop -- --nocapture` command requests 2 devnet SOL to the wallet. This provides the initial balance required to trade on the Solana blockchain.

# transfer_sol
Sends a certain amount of SOL tokens to a specific address.
`cargo test transfer_sol -- --nocapture`, transfers 0.1 SOL from the new account to the previous account. This transaction is secured by signing with the sender's secret key.

# transfer_all_sol
Sends the entire balance in the wallet to a specific address.
`cargo test transfer_all_sol -- --nocapture`, transfers the maximum possible amount of SOL from the new account to the previous account. To account for the transaction fee, a transaction is first created and the network is informed of how much the fee will be, then the fee is deducted from the total balance and the transfer is performed.

#Interaction with the WbaPrereq Programme  
Turbin3 Builders Course registration requires interaction with a program deployed on the network. Using the program's IDL (Interface Definition Language), it calls the program's method and stores the data by passing the GitHub username as an argument and deriving the PDA.

To account for the transaction fee, a fake transaction is first created and the fee for this transaction is learnt from the network. Then, this fee is subtracted from the total balance and the maximum SOL transfer is performed.

# My TXs
* Getting Devnet SOL: https://explorer.solana.com/tx/XYRwNJ37Qj8UJ8G1qb97FuepgSDMHjNzVodrf5G1ahC5HNDXiZguxQCKtVhSG4mq7qqFNaNpQazY5PVbzTJs18B?cluster=devnet
* Transferring 0.1 SOL: https://explorer.solana.com/tx/1SsqqCnPccTfHyiZvvwinauETP5ibzrE42rmp195wLMQaPJZyLrmDE1pX71TBT5wyavNYSDSbazjg3HPwma3vKb?cluster=devnet
* Transferring All SOL: https://explorer.solana.com/tx/MmAsACMyAuv4YmBQeDVBh8pv5AKu1mn473vD3ASspExajt4S1mzRvhVefNr3LkA9St9UsYzmGwJZ8eshG1zXPud?cluster=devnet
* Interacting with the program: https://explorer.solana.com/tx/3PeR6q7KKSKwViw9LjGDe1mqs9LETZ2bTSKHvkZx9fwF63SsQhnoM4McMN4Kf3QbyuwkP3BY2ZUSFYEKu7wvdimc?cluster=devnet
