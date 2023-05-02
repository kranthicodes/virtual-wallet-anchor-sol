import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VirtualWallet } from "../target/types/virtual_wallet";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("virtual-wallet", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VirtualWallet as Program<VirtualWallet>;
  const provider = anchor.AnchorProvider.env();

  it("Is initialized!", async () => {
    //derive cash PDA
    const [cash, cashBump] = await PublicKey.findProgramAddress(
      [provider.wallet.publicKey.toBuffer(), Buffer.from("cash")],
      program.programId
    );

    //derive credit card PDA
    const [creditCard, creditCardBump] = await PublicKey.findProgramAddress(
      [provider.wallet.publicKey.toBuffer(), Buffer.from("credit_card")],
      program.programId
    );

    //derive debit card PDA
    const [debitCard, debitCardBump] = await PublicKey.findProgramAddress(
      [provider.wallet.publicKey.toBuffer(), Buffer.from("debit_card")],
      program.programId
    );

    const tx = await program.methods
      .initializeWallet()
      .accounts({
        user: provider.wallet.publicKey,
        cash: cash,
        creditCard: creditCard,
        debitCard: debitCard,
        systemProgram: SystemProgram.programId,
      })
      .signers([])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
