import {
  createSolanaClient,
  createTransaction,
  generateKeyPairSigner,
  getExplorerLink,
  getMinimumBalanceForRentExemption,
  getSignatureFromTransaction,
  KeyPairSigner,
  signTransactionMessageWithSigners,
} from "gill";
import { loadKeypairSignerFromFile } from "gill/node";
import {
  getInitializeMintInstruction,
  getMintSize,
  TOKEN_PROGRAM_ADDRESS,
} from "gill/programs/token";
import {
  getCreateAccountInstruction,
  getCreateMetadataAccountV3Instruction,
  getTokenMetadataAddress,
} from "gill/programs";

const tokenProgram = TOKEN_PROGRAM_ADDRESS;

const { rpc, rpcSubscriptions, sendAndConfirmTransaction } = createSolanaClient(
  {
    urlOrMoniker: "devnet", // `mainnet`, `localnet`, etc
  }
);

(async () => {
  try {
    const signer: KeyPairSigner = await loadKeypairSignerFromFile();
    
    // Check account balance
    const balance = await rpc.getBalance(signer.address).send();
    console.log(`Signer balance: ${Number(balance.value) / 1_000_000_000} SOL`);
    
    if (balance.value < 100_000_000) { // Less than 0.1 SOL
      console.log("Insufficient balance. Please airdrop SOL to your account:");
      console.log(`solana airdrop 2 ${signer.address} --url devnet`);
      return;
    }

    const mint = await generateKeyPairSigner();
    const metadataAddress = await getTokenMetadataAddress(mint);

    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

    const space = getMintSize();
    const transaction = createTransaction({
      feePayer: signer,
      version: "legacy",
      instructions: [
        getCreateAccountInstruction({
          space,
          lamports: await getMinimumBalanceForRentExemption(space),
          newAccount: mint,
          payer: signer,
          programAddress: tokenProgram,
        }),
        getInitializeMintInstruction(
          {
            mint: mint.address,
            mintAuthority: signer.address,
            freezeAuthority: signer.address,
            decimals: 9,
          },
          {
            programAddress: tokenProgram,
          }
        ),
        getCreateMetadataAccountV3Instruction({
          collectionDetails: null,
          isMutable: true,
          updateAuthority: signer,
          mint: mint.address,
          metadata: metadataAddress,
          mintAuthority: signer,
          payer: signer,
          data: {
            sellerFeeBasisPoints: 0,
            collection: null,
            creators: null,
            uses: null,
            name: "yash",
            symbol: "ys",
            uri: "https://raw.githubusercontent.com/yashvikram30/token_metadata/main/metadata.json",
          },
        }),
      ],
      latestBlockhash,
    });

    const signedTransaction = await signTransactionMessageWithSigners(
      transaction
    );

    console.log(
      "Explorer:",
      getExplorerLink({
        cluster: "devnet",
        transaction: getSignatureFromTransaction(signedTransaction),
      })
    );
    
    const result = await sendAndConfirmTransaction(signedTransaction);
    console.log("Transaction confirmed:", result);
    
  } catch (error) {
    console.log("Error: ", error);
  }
})();