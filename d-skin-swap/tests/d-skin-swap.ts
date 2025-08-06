import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DSkinSwap } from "../target/types/d_skin_swap";
import { PublicKey, Keypair } from "@solana/web3.js";
import { expect } from "chai";

describe("d-skin-swap", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.dSkinSwap as Program<DSkinSwap>;
  const provider = anchor.AnchorProvider.env();

  // Test accounts
  let authority: Keypair;
  let treasury: Keypair;
  let marketplaceConfigPDA: PublicKey;

  before(async () => {
    authority = provider.wallet.payer;
    treasury = Keypair.generate();

    // Derive marketplace config PDA
    [marketplaceConfigPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace")],
      program.programId
    );
  });

  it("Initialize marketplace", async () => {
    const feeRate = 100; // 1% fee rate

    const tx = await program.methods
      .initialize(feeRate)
      .accounts({
        // @ts-ignore
        marketplaceConfig: marketplaceConfigPDA,
        authority: authority.publicKey,
        treasury: treasury.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    console.log("Marketplace initialized with transaction:", tx);

    // Verify the marketplace config
    const marketplaceConfig = await program.account.marketplaceConfig.fetch(marketplaceConfigPDA);
    console.log("Marketplace config:", marketplaceConfig);

    // Assertions
    expect(marketplaceConfig.authority.toString()).to.equal(authority.publicKey.toString());
    expect(marketplaceConfig.feeRate).to.equal(feeRate);
    expect(marketplaceConfig.treasury.toString()).to.equal(treasury.publicKey.toString());
    expect(marketplaceConfig.totalTrades.toNumber()).to.equal(0);
    expect(marketplaceConfig.totalVolume.toNumber()).to.equal(0);
  });
});
