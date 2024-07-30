mod programs;

#[cfg(test)]
mod tests {
    use crate::programs::wba_prereq::{WbaPrereqProgram, CompleteArgs}; 
    use solana_sdk::signature::{read_keypair_file, Keypair, Signer};
    use std::io::{self, BufRead};
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::message::Message;
    use solana_program::native_token::LAMPORTS_PER_SOL;
    use solana_program::{
        pubkey::Pubkey,
        system_instruction::transfer,
        system_program,
    };
    use solana_sdk::transaction::Transaction;
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";
    const MY_WBA_WALLET_ADDRESS: &str = "9B5XszUGdMaxCZ7uSQhPzdks5ZQSmWxrmzCSvtJ6Ns6g";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet with public key: {}",
            kp.pubkey()
        );
        println!("To save your wallet, copy and paste the following byte array into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin.lock().lines().next().unwrap().unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
    
    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("./wallets/dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str(MY_WBA_WALLET_ADDRESS).unwrap();
        let client = RpcClient::new(RPC_URL);

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let ix = transfer(&keypair.pubkey(), &to_pubkey, LAMPORTS_PER_SOL / 10);

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&tx)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

    #[test]
    fn transfer_all_sol() {
        let keypair = read_keypair_file("./wallets/dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str(MY_WBA_WALLET_ADDRESS).unwrap();
        let client = RpcClient::new(RPC_URL);

        let from_balance = client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let mock_ix = transfer(&keypair.pubkey(), &to_pubkey, from_balance);
        let message =
            Message::new_with_blockhash(&[mock_ix], Some(&keypair.pubkey()), &recent_blockhash);
        let fee = client
            .get_fee_for_message(&message)
            .expect("Failed to get fee");

        let ix = transfer(&keypair.pubkey(), &to_pubkey, from_balance - fee);

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&tx)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

    #[test]
    fn last() {
        let signer =
            read_keypair_file("./wallets/my-wba-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        let pda_pubkey = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        let args = CompleteArgs {
            github: b"MuhammedAkinci".to_vec(),
        };

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let tx = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &pda_pubkey, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&tx)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }
}
