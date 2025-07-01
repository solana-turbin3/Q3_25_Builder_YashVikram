#[cfg(test)]
mod tests {
    use solana_sdk::{ message::Message, signature::{read_keypair_file, Keypair, Signer}, system_program, transaction::Transaction,     instruction::{Instruction, AccountMeta},};
    use bs58; 
    use std::{ io::{self, BufRead},};
    use solana_client::rpc_client::RpcClient;
    use std::str::FromStr;
    use solana_program::{ hash::hash, pubkey::Pubkey, system_instruction::transfer};

    #[test]
    fn keygen() {
        let kp: Keypair = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn print_pubkey() {
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
        println!("Your dev_wallet public key: {}", keypair.pubkey());
    }
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn airdrop() {  
        const RPC_URL: &str ="https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
            println!("Success! Check your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
            println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn check_balance() {
        const RPC_URL: &str ="https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        
        match client.get_balance(&keypair.pubkey()) {
            Ok(balance) => println!("Wallet balance: {} lamports ({} SOL)", balance, balance as f64 / 1_000_000_000.0),
            Err(err) => println!("Failed to get balance: {}", err),
        }
    }
   
    #[test]
    fn transfer_sol() {
        const RPC_URL: &str ="https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
        let pubkey = keypair.pubkey();

        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        
        // Verify the signature using the public key and original message
        match sig.verify(&pubkey.to_bytes(), message_bytes) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
        
        let to_pubkey = Pubkey::from_str("Fv6pfvTxAXECfs81aPNFTqiLvwkSkPUo3D3Zr7NrGfNP").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

#[test]
fn empty_devnet_wallet() {
    const RPC_URL: &str ="https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

    let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
    let to_pubkey = Pubkey::from_str("Fv6pfvTxAXECfs81aPNFTqiLvwkSkPUo3D3Zr7NrGfNP").unwrap();
    let rpc_client = RpcClient::new(RPC_URL);
    
    // Get current balance
    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
    
    println!("Current balance: {} lamports", balance);
    
    if balance == 0 {
        println!("Wallet is already empty!");
        return;
    }
    
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // Build a mock transaction to calculate fee
    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );

    // Estimate transaction fee
    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");
    
    println!("Transaction fee: {} lamports", fee);
    
    if balance <= fee {
        println!("Balance is too low to cover transaction fee!");
        return;
    }

    // Create final transaction with balance minus fee
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );

    // Send transaction and verify
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send final transaction");
    
    println!(
        "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
    
    // Verify wallet is now empty
    let final_balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get final balance");
    
    println!("Final balance: {} lamports", final_balance);
    }

    #[test]
fn submit_turbin3() {
    const RPC_URL: &str = "https://api.devnet.solana.com";
    
    // Step 1: Create RPC client
    let rpc_client = RpcClient::new(RPC_URL);
    
    // Step 2: Load your signer keypair
    let signer = read_keypair_file("Turbin3-wallet.json")
        .expect("Couldn't find wallet file");
    
    // Step 3: Define program and account public keys
    let mint = Keypair::new();
    let turbin3_prereq_program = 
        Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection = 
        Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program = 
        Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let system_program = system_program::id();
    
    // Step 4: Get the PDA (Program Derived Address)
    let signer_pubkey = signer.pubkey();
    let seeds: &[&[u8]] = &[b"prereqs", signer_pubkey.as_ref()];
    let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
    
    // Get authority PDA using "collection" seed + collection pubkey (from your TS code)
    let authority_seeds: &[&[u8]] = &[b"collection", collection.as_ref()];
    let (authority, _auth_bump) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);
    
    // Step 5: Prepare the instruction data (discriminator)
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
    
    // Step 6: Define the accounts metadata
    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),        // user signer
        AccountMeta::new(prereq_pda, false),            // PDA account
        AccountMeta::new(mint.pubkey(), true),          // mint keypair
        AccountMeta::new(collection, false),            // collection
        AccountMeta::new_readonly(authority, false),    // authority (PDA)
        AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
        AccountMeta::new_readonly(system_program, false),   // system program
    ];
    
    // Step 7: Get the recent blockhash
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    
    // Step 8: Build the instruction
    let instruction = Instruction {
        program_id: turbin3_prereq_program,
        accounts,
        data,
    };
    
    // Step 9: Create and sign the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[&signer, &mint],
        blockhash,
    );
    
    // Step 10: Send and confirm the transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    
    println!(
        "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
} 
