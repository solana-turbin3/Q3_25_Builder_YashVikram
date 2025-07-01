import { PublicKey } from "@solana/web3.js";


// 1. Your program ID (Turbin3 prereq program)
const PROGRAM_ID = new PublicKey("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM");

// 2. Your wallet public key
const MY_PUBKEY = new PublicKey("Fv6pfvTxAXECfs81aPNFTqiLvwkSkPUo3D3Zr7NrGfNP");

// 3. Derive the PDA the same way your code does:
const [pda] = PublicKey.findProgramAddressSync(
  [Buffer.from("prereqs"), MY_PUBKEY.toBuffer()],
  PROGRAM_ID
);

console.log("Your ApplicationAccount PDA is:", pda.toBase58());
console.log("Explorer URL:", `https://explorer.solana.com/address/${pda.toBase58()}?cluster=devnet`);
