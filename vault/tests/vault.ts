import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { expect } from "chai";

describe("vault", () => {
  // step:1 set the provider here
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.vault as Program<Vault>;

  // step:2 we are declaring the pdas here
  let vault: anchor.web3.PublicKey;
  let vaultState: anchor.web3.PublicKey;

  // step:3 derive the signer
  const signer = provider.wallet;

  before(async () => {
    // step:4 Derive PDAs before running tests
    [vaultState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("state"), signer.publicKey.toBuffer()],
      program.programId
    );

    [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), vaultState.toBuffer()],
      program.programId
    );

    // step:5 initialize the transaction only once before tests
    try {
      await program.methods
        .initialize()
        .accountsPartial({
          signer: signer.publicKey,
          vault,
          vaultState,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    } catch (err: any) {
      // swallow 'already in use' error (happens if test re-runs)
      if (!err.message.includes("already in use")) throw err;
    }
  });

  it("Initializes vault", async () => {
    // step:6 Verify vault state account was created
    const vaultStateAccount = await program.account.vaultState.fetch(
      vaultState
    );
    expect(vaultStateAccount.vaultBump).to.be.a("number");
    expect(vaultStateAccount.stateBump).to.be.a("number");
  });

  // step:7 write a deposit transaction
  it("Deposit 1 Sol", async () => {
    const amount = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL);

    const tx = await program.methods
      .deposit(amount)
      .accountsPartial({
        signer: signer.publicKey,
        vault,
        vaultState,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Deposit tx:", tx);

    // step:8 check if this performed the expected output
    const vaultAccount = await provider.connection.getAccountInfo(vault);
    expect(vaultAccount).to.not.be.null;
    expect(vaultAccount!.lamports).to.be.greaterThanOrEqual(amount.toNumber());
  });

  it("Withdraws 0.5 Sol", async () => {
    const amount = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL / 2);

    // step:9 lamports in the accounts before withdrawal & transaction
    const beforeAccount = await provider.connection.getAccountInfo(vault);
    const beforeLamports = beforeAccount?.lamports ?? 0;

    const tx = await program.methods
      .withdraw(amount)
      .accountsPartial({
        signer: signer.publicKey,
        vault,
        vaultState,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    // step:10 lamports in the account after withdrawal
    const afterAccount = await provider.connection.getAccountInfo(vault);
    const afterLamports = afterAccount?.lamports ?? 0;
    
    // step:11 expect the lamports to be closeTo the difference of before and after lamports
    expect(beforeLamports - afterLamports).to.be.closeTo(
      anchor.web3.LAMPORTS_PER_SOL / 2,
      10_000 // tolerance for fees/slippage
    );
  });

  it("Closes the account",async()=>{

    // step:12 transaction for closing the account
    const tx = await program.methods
              .close()
              .accountsPartial({
                signer: signer.publicKey,
                vault,
                vaultState,
                systemProgram: anchor.web3.SystemProgram.programId
              })
              .rpc()
    
    const vaultAccount = await provider.connection.getAccountInfo(vault);
    const vaultStateAccount = await provider.connection.getAccountInfo(vaultState);

    // expecting the account to be closed
    expect(vaultAccount).to.be.null;
    expect(vaultStateAccount).to.be.null;
  })
});
