import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HoloPresale } from "../target/types/holo_presale";
import {
  ComputeBudgetProgram,
  Connection,
  PublicKey,
  sendAndConfirmRawTransaction,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";

describe("holo-presale", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HoloPresale as Program<HoloPresale>;
  const provider = anchor.AnchorProvider.env();
  const admin = provider.wallet.publicKey;

  let pool: PublicKey;

  console.log("program", program.programId.toBase58());

  it("Initializes the pool", async () => {
    const [poolConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from("AdminConfiguration")],
      program.programId
    );

    pool = poolConfig;

    await program.methods
      .initializePool(
        false, // is_active
        new anchor.BN(100), // referral_fee_percentage
        false, // referral_lockdown
        admin, // admin_wallet
        admin, // fund_wallet
        new anchor.BN(1000), // sale_amount
        new anchor.BN(Date.now() / 1000), // start_time
        new anchor.BN(Date.now() / 1000 + 3600) // end_time
      )
      .accounts({
        pool,
        admin,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Pool initialized with address:", pool.toBase58());
  });

  it("Updates the pool", async () => {
    await program.methods
      .updatePool(
        true, // is_active
        new anchor.BN(200), // referral_fee_percentage
        true, // referral_lockdown
        admin, // admin_wallet
        admin, // fund_wallet
        new anchor.BN(2000), // sale_amount
        new anchor.BN(Date.now() / 1000), // start_time
        new anchor.BN(Date.now() / 1000 + 7200) // end_time
      )
      .accounts({
        pool,
        admin,
      })
      .rpc();

    console.log("Pool updated");
  });

  it("Buys from the pool", async () => {
    const user = anchor.web3.Keypair.generate();

    // Airdrop some SOL to the user for testing
    await provider.connection.requestAirdrop(user.publicKey, 1e9);
    const tx = new Transaction().add(
      ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: 1_200_000,
      }),
      ComputeBudgetProgram.setComputeUnitLimit({
        units: 20_000,
      }),
      await program.methods
        .buy(new anchor.BN(100))
        .accounts({
          user: user.publicKey,
          fundWallet: admin,
          referralAccount: null,
          pool,
          systemProgram: SystemProgram.programId,
        })
        .instruction()
    );

    tx.feePayer = user.publicKey;
    tx.recentBlockhash = (
      await provider.connection.getLatestBlockhash()
    ).blockhash;
    const sig = await sendAndConfirmTransaction(
      provider.connection,
      tx,
      [user],
      {
        skipPreflight: true,
      }
    );
    console.log("sig", sig);
    console.log("Purchase completed");
  });

  it("Buy with referral", async () => {
    const referral = anchor.web3.Keypair.generate();
    const user = anchor.web3.Keypair.generate();

    // Airdrop some SOL to the user for testing
    await provider.connection.requestAirdrop(user.publicKey, 1e9);

    const tx = new Transaction().add(
      ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: 1_200_000,
      }),
      ComputeBudgetProgram.setComputeUnitLimit({
        units: 20_000,
      }),
      await program.methods
        .buy(new anchor.BN(100))
        .accounts({
          user: user.publicKey,
          fundWallet: admin,
          referralAccount: referral.publicKey,
          pool,
          systemProgram: SystemProgram.programId,
        })
        .instruction()
    );

    tx.feePayer = user.publicKey;
    tx.recentBlockhash = (
      await provider.connection.getLatestBlockhash()
    ).blockhash;
    const sig = await sendAndConfirmTransaction(
      provider.connection,
      tx,
      [user],
      {
        skipPreflight: true,
      }
    );
    console.log("sig", sig);
    console.log("Purchase with referral completed");
  });
});
