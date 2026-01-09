import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { KudosMeter } from "../target/types/kudos_meter";
import { assert } from "chai";

describe("kudos-meter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.KudosMeter as Program<KudosMeter>;
  const user = provider.wallet;

  // Calculate the address (PDA) where the counter will live
  const [kudosPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("kudos"), user.publicKey.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        user: user.publicKey,
        kudosAccount: kudosPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    console.log("Your transaction signature", tx);
    
    // Fetch the account to check if it worked
    const account = await program.account.kudosAccount.fetch(kudosPda);
    console.log("Count is:", account.count.toString());
    assert.ok(account.count.eq(new anchor.BN(0)));
  });

  it("Gives Kudos!", async () => {
    await program.methods
      .giveKudos()
      .accounts({
        kudosAccount: kudosPda,
        recipient: user.publicKey, // Giving kudos to ourselves for the test
      })
      .rpc();

    const account = await program.account.kudosAccount.fetch(kudosPda);
    console.log("Count is:", account.count.toString());
    assert.ok(account.count.eq(new anchor.BN(1)));
  });
});