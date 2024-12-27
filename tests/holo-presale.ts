import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HoloPresale } from "../target/types/holo_presale";
import {
  ComputeBudgetProgram,
  LAMPORTS_PER_SOL,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { expect } from "chai";

describe("holo-presale", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HoloPresale as Program<HoloPresale>;
  // Set the provider to use the Devnet RPC URL
  // const provider = new anchor.AnchorProvider(
  //   new anchor.web3.Connection(
  //     "https://rpc.ankr.com/solana_devnet/8c795f9a30522e491742b37ff77731c1e3909fec0fc5ff4f0f529ff57b6910bb"
  //   ),
  //   anchor.Wallet.local(),
  //   { commitment: "confirmed" }
  // );
  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection("http://localhost:8899"),
    anchor.Wallet.local(),
    { commitment: "confirmed" }
  );
  const admin = provider.wallet.publicKey;
  const user = anchor.web3.Keypair.generate();
  console.log("admin", admin.toBase58());
  let pool: PublicKey;

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
        false, // referral_lockdown
        admin, // admin_wallet
        admin, // fund_wallet
        new anchor.BN(100000 * LAMPORTS_PER_SOL), // sale_amount
        new anchor.BN(Date.now() / 1000), // start_time
        new anchor.BN(Date.now() / 1000 + 999999999999) // end_time
      )
      .accounts({
        pool,
        admin,
      })
      .rpc();

    console.log("Pool updated");
  });

  it("Buys from the pool", async () => {
    const [userPurchase, bump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("user_purchase"),
        user.publicKey.toBuffer(),
        pool.toBuffer(),
      ],
      program.programId
    );
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
          referralAccount: admin,
          userPurchase,
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

    // log userPurchase data
    const userPurchaseData = await program.account.userPurchase.fetch(
      userPurchase,
      "confirmed"
    );
    console.log("userPurchase", userPurchaseData.totalPurchased.toString());
  });

  it("Buy with referral", async () => {
    const referral = anchor.web3.Keypair.generate();
    const [userPurchase, bump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("user_purchase"),
        user.publicKey.toBuffer(),
        pool.toBuffer(),
      ],
      program.programId
    );

    console.log("referral", referral.publicKey.toBase58());
    // Airdrop some SOL to the user for testing
    await provider.connection.requestAirdrop(user.publicKey, 1e9);
    await provider.connection.requestAirdrop(referral.publicKey, 1e9);


    const tx = new Transaction().add(
      ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: 1_200_000,
      }),
      ComputeBudgetProgram.setComputeUnitLimit({
        units: 20_000,
      }),
      await program.methods
        .buy(new anchor.BN(0.5 * LAMPORTS_PER_SOL))
        .accounts({
          user: user.publicKey,
          fundWallet: admin,
          referralAccount: referral.publicKey,
          userPurchase,
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
    ).catch((e) => {
      console.log("error", e);
    });
    console.log("sig", sig);

    // log userPurchase data
    const userPurchaseData = await program.account.userPurchase.fetch(
      userPurchase,
      "confirmed"
    );
    // get sol off referral
    const balance = await provider.connection.getBalance(referral.publicKey);
    console.log("referral balance", balance);
    console.log("userPurchase", userPurchaseData.totalPurchased.toString());
  });
});
