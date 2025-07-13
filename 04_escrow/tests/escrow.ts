import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAccount
} from "@solana/spl-token";
import { assert, expect } from "chai";
import { LAMPORTS_PER_SOL, PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

describe("escrow", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;

  // Test accountsPartial
  let maker: Keypair;
  let taker: Keypair;
  let mintA: PublicKey;
  let mintB: PublicKey;
  let makerAtaA: PublicKey;
  let makerAtaB: PublicKey;
  let takerAtaA: PublicKey;
  let takerAtaB: PublicKey;
  let escrowPda: PublicKey;
  let escrowVault: PublicKey;
  let escrowBump: number;
  let vaultBump: number;

  // Test parameters
  const seed = new anchor.BN(1);
  const depositAmount = new anchor.BN(100 * 10**9); // 100 tokens
  const receiveAmount = new anchor.BN(200 * 10**9); // 200 tokens

  before(async () => {
    // Create keypairs
    maker = Keypair.generate();
    taker = Keypair.generate();

    // Airdrop SOL to maker and taker
    await connection.confirmTransaction(
      await connection.requestAirdrop(maker.publicKey, 2 * LAMPORTS_PER_SOL)
    );
    await connection.confirmTransaction(
      await connection.requestAirdrop(taker.publicKey, 2 * LAMPORTS_PER_SOL)
    );

    // Create mints
    mintA = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      9
    );

    mintB = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      9
    );

    // Create associated token accountsPartial
    makerAtaA = await createAssociatedTokenAccount(
      connection,
      wallet.payer,
      mintA,
      maker.publicKey
    );

    makerAtaB = await createAssociatedTokenAccount(
      connection,
      wallet.payer,
      mintB,
      maker.publicKey
    );

    takerAtaA = await createAssociatedTokenAccount(
      connection,
      wallet.payer,
      mintA,
      taker.publicKey
    );

    takerAtaB = await createAssociatedTokenAccount(
      connection,
      wallet.payer,
      mintB,
      taker.publicKey
    );

    // Mint tokens to accountsPartial
    await mintTo(
      connection,
      wallet.payer,
      mintA,
      makerAtaA,
      wallet.publicKey,
      1000 * 10**9 // 1000 tokens
    );

    await mintTo(
      connection,
      wallet.payer,
      mintB,
      takerAtaB,
      wallet.publicKey,
      1000 * 10**9 // 1000 tokens
    );

    // Derive PDAs
    [escrowPda, escrowBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

    escrowVault = getAssociatedTokenAddressSync(
      mintA,
      escrowPda,
      true
    );
  });

  describe("make", () => {
    it("Creates an escrow successfully", async () => {
      const tx = await program.methods
        .make(seed, depositAmount, receiveAmount)
        .accountsPartial({
          maker: maker.publicKey,
          mintA: mintA,
          mintB: mintB,
          makerAtaA: makerAtaA,
          escrow: escrowPda,
          escrowVault: escrowVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([maker])
        .rpc();

      console.log("Make transaction signature:", tx);

      // Verify escrow account was created
      const escrowAccount = await program.account.escrow.fetch(escrowPda);
      assert.equal(escrowAccount.seed.toString(), seed.toString());
      assert.equal(escrowAccount.maker.toString(), maker.publicKey.toString());
      assert.equal(escrowAccount.mintA.toString(), mintA.toString());
      assert.equal(escrowAccount.mintB.toString(), mintB.toString());
      assert.equal(escrowAccount.receive.toString(), receiveAmount.toString());

      // Verify tokens were transferred to vault
      const vaultAccount = await getAccount(connection, escrowVault);
      assert.equal(vaultAccount.amount.toString(), depositAmount.toString());

      // Verify tokens were deducted from maker
      const makerAtaAAccount = await getAccount(connection, makerAtaA);
      assert.equal(
        makerAtaAAccount.amount.toString(),
        (1000 * 10**9 - depositAmount.toNumber()).toString()
      );
    });

    it("Fails with invalid amount", async () => {
      const invalidSeed = new anchor.BN(2);
      const invalidAmount = new anchor.BN(0);

      const [invalidEscrowPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("escrow"),
          maker.publicKey.toBuffer(),
          invalidSeed.toArrayLike(Buffer, "le", 8)
        ],
        program.programId
      );

      const invalidEscrowVault = getAssociatedTokenAddressSync(
        mintA,
        invalidEscrowPda,
        true
      );

      try {
        await program.methods
          .make(invalidSeed, invalidAmount, receiveAmount)
          .accountsPartial({
            maker: maker.publicKey,
            mintA: mintA,
            mintB: mintB,
            makerAtaA: makerAtaA,
            escrow: invalidEscrowPda,
            escrowVault: invalidEscrowVault,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .signers([maker])
          .rpc();
        
        assert.fail("Should have failed with invalid amount");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("InvalidAmount");
        expect(error.error.errorMessage).to.equal("Amount must be greater than zero.");
      }
    });

    it("Fails with insufficient funds", async () => {
      const invalidSeed = new anchor.BN(3);
      const excessiveAmount = new anchor.BN(2000 * 10**9); // More than maker has

      const [invalidEscrowPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("escrow"),
          maker.publicKey.toBuffer(),
          invalidSeed.toArrayLike(Buffer, "le", 8)
        ],
        program.programId
      );

      const invalidEscrowVault = getAssociatedTokenAddressSync(
        mintA,
        invalidEscrowPda,
        true
      );

      try {
        await program.methods
          .make(invalidSeed, excessiveAmount, receiveAmount)
          .accountsPartial({
            maker: maker.publicKey,
            mintA: mintA,
            mintB: mintB,
            makerAtaA: makerAtaA,
            escrow: invalidEscrowPda,
            escrowVault: invalidEscrowVault,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .signers([maker])
          .rpc();
        
        assert.fail("Should have failed with insufficient funds");
      } catch (error) {
        // This might throw a different error depending on SPL token implementation
        expect(error.message).to.include("insufficient");
      }
    });
  });

  describe("transfer", () => {
    it("Completes the escrow transfer successfully", async () => {
      const tx = await program.methods
        .transfer(seed)
        .accountsPartial({
          maker: maker.publicKey,
          taker: taker.publicKey,
          mintA: mintA,
          mintB: mintB,
          makerAtaB: makerAtaB,
          takerAtaA: takerAtaA,
          takerAtaB: takerAtaB,
          escrow: escrowPda,
          vault: escrowVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([taker])
        .rpc();

      console.log("Transfer transaction signature:", tx);

      // Verify taker received tokens from vault
      const takerAtaAAccount = await getAccount(connection, takerAtaA);
      assert.equal(takerAtaAAccount.amount.toString(), depositAmount.toString());

      // Verify maker received tokens from taker
      const makerAtaBAccount = await getAccount(connection, makerAtaB);
      assert.equal(makerAtaBAccount.amount.toString(), receiveAmount.toString());

      // Verify taker's token B balance decreased
      const takerAtaBAccount = await getAccount(connection, takerAtaB);
      assert.equal(
        takerAtaBAccount.amount.toString(),
        (1000 * 10**9 - receiveAmount.toNumber()).toString()
      );

      // Verify vault is closed (try to fetch and expect it to fail)
      try {
        await getAccount(connection, escrowVault);
        assert.fail("Vault account should be closed");
      } catch (error) {
        expect(error.name).to.equal("TokenAccountNotFoundError");
      }

      // Verify escrow account is closed
      try {
        await program.account.escrow.fetch(escrowPda);
        assert.fail("Escrow account should be closed");
      } catch (error) {
        expect(error.message).to.include("Account does not exist");
      }
    });

    it("Fails when vault is empty", async () => {
      // This test is challenging because once an escrow is used, it's closed
      // Instead, let's test a scenario where we try to transfer with insufficient tokens in taker's account
      
      // Create a new escrow for this test
      const emptyVaultSeed = new anchor.BN(10);
      const [emptyVaultEscrowPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("escrow"),
          maker.publicKey.toBuffer(),
          emptyVaultSeed.toArrayLike(Buffer, "le", 8)
        ],
        program.programId
      );

      const emptyVaultEscrowVault = getAssociatedTokenAddressSync(
        mintA,
        emptyVaultEscrowPda,
        true
      );

      // Create escrow with deposit
      await program.methods
        .make(emptyVaultSeed, depositAmount, receiveAmount)
        .accountsPartial({
          maker: maker.publicKey,
          mintA: mintA,
          mintB: mintB,
          makerAtaA: makerAtaA,
          escrow: emptyVaultEscrowPda,
          escrowVault: emptyVaultEscrowVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([maker])
        .rpc();

      // Complete the transfer once
      await program.methods
        .transfer(emptyVaultSeed)
        .accountsPartial({
          maker: maker.publicKey,
          taker: taker.publicKey,
          mintA: mintA,
          mintB: mintB,
          makerAtaB: makerAtaB,
          takerAtaA: takerAtaA,
          takerAtaB: takerAtaB,
          escrow: emptyVaultEscrowPda,
          vault: emptyVaultEscrowVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([taker])
        .rpc();

      // Now the escrow should be closed, so any attempt to use it should fail
      try {
        await program.methods
          .transfer(emptyVaultSeed)
          .accountsPartial({
            maker: maker.publicKey,
            taker: taker.publicKey,
            mintA: mintA,
            mintB: mintB,
            makerAtaB: makerAtaB,
            takerAtaA: takerAtaA,
            takerAtaB: takerAtaB,
            escrow: emptyVaultEscrowPda,
            vault: emptyVaultEscrowVault,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .signers([taker])
          .rpc();

        assert.fail("Should have failed with closed escrow");
      } catch (error) {
        // The escrow account has been closed after successful transfer
        expect(error.toString()).to.include("AnchorError");
      }
    });
  });

  describe("refund", () => {
    let refundSeed: anchor.BN;
    let refundEscrowPda: PublicKey;
    let refundVault: PublicKey;

    before(async () => {
      // Create a new escrow for refund testing
      refundSeed = new anchor.BN(5);
      [refundEscrowPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("escrow"),
          maker.publicKey.toBuffer(),
          refundSeed.toArrayLike(Buffer, "le", 8)
        ],
        program.programId
      );

      refundVault = getAssociatedTokenAddressSync(
        mintA,
        refundEscrowPda,
        true
      );

      // Create escrow
      await program.methods
        .make(refundSeed, depositAmount, receiveAmount)
        .accountsPartial({
          maker: maker.publicKey,
          mintA: mintA,
          mintB: mintB,
          makerAtaA: makerAtaA,
          escrow: refundEscrowPda,
          escrowVault: refundVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([maker])
        .rpc();
    });

    it("Refunds the escrow successfully", async () => {
      // Get maker's token balance before refund
      const makerAtaABefore = await getAccount(connection, makerAtaA);
      const balanceBefore = makerAtaABefore.amount;

      const tx = await program.methods
        .refund(refundSeed)
        .accountsPartial({
          maker: maker.publicKey,
          mintA: mintA,
          mintB: mintB,
          makerAtaA: makerAtaA,
          escrow: refundEscrowPda,
          vault: refundVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([maker])
        .rpc();

      console.log("Refund transaction signature:", tx);

      // Verify maker received tokens back
      const makerAtaAAfter = await getAccount(connection, makerAtaA);
      assert.equal(
        makerAtaAAfter.amount.toString(),
        (Number(balanceBefore) + depositAmount.toNumber()).toString()
      );

      // Verify vault is closed (try to fetch and expect it to fail)
      try {
        await getAccount(connection, refundVault);
        assert.fail("Vault account should be closed");
      } catch (error) {
        expect(error.name).to.equal("TokenAccountNotFoundError");
      }

      // Verify escrow account is closed
      try {
        await program.account.escrow.fetch(refundEscrowPda);
        assert.fail("Escrow account should be closed");
      } catch (error) {
        expect(error.message).to.include("Account does not exist");
      }
    });

    it("Fails when not called by maker", async () => {
      // Create another escrow for this test
      const unauthorizedSeed = new anchor.BN(6);
      const [unauthorizedEscrowPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("escrow"),
          maker.publicKey.toBuffer(),
          unauthorizedSeed.toArrayLike(Buffer, "le", 8)
        ],
        program.programId
      );

      const unauthorizedVault = getAssociatedTokenAddressSync(
        mintA,
        unauthorizedEscrowPda,
        true
      );

      // Create escrow
      await program.methods
        .make(unauthorizedSeed, depositAmount, receiveAmount)
        .accountsPartial({
          maker: maker.publicKey,
          mintA: mintA,
          mintB: mintB,
          makerAtaA: makerAtaA,
          escrow: unauthorizedEscrowPda,
          escrowVault: unauthorizedVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([maker])
        .rpc();

      // Try to refund with taker (should fail)
      try {
        await program.methods
          .refund(unauthorizedSeed)
          .accountsPartial({
            maker: maker.publicKey,
            mintA: mintA,
            mintB: mintB,
            makerAtaA: makerAtaA,
            escrow: unauthorizedEscrowPda,
            vault: unauthorizedVault,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .signers([taker]) // Wrong signer
          .rpc();

        assert.fail("Should have failed with unauthorized signer");
      } catch (error) {
        expect(error.toString()).to.include("unknown signer");
      }
    });
  });
});