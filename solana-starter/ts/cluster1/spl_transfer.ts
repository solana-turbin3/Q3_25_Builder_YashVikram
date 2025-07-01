import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("DNerTBaY7BLVUpkaFQ3YEkoPnGpqQtTJ8vaB4x3cS3XH");

// Recipient address
const to = new PublicKey("Fv6pfvTxAXECfs81aPNFTqiLvwkSkPUo3D3Zr7NrGfNP");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const sender_ata = await getOrCreateAssociatedTokenAccount(connection,keypair,mint,keypair.publicKey);

        // Get the token account of the toWallet address, and if it does not exist, create it
        const receiver_ata = await getOrCreateAssociatedTokenAccount(connection,keypair,mint,to);

        // Transfer the new token to the "toTokenAccount" we just created
        const tx = await transfer(connection,keypair,sender_ata.address,receiver_ata.address,keypair,10);

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();