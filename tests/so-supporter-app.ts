import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SoSupporterApp } from "../target/types/so_supporter_app";
import { expect } from "chai";

describe("so-supporter-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const myAccount = anchor.web3.Keypair.generate();
  const program = anchor.workspace.SoSupporterApp as Program<SoSupporterApp>;

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.createPost("title", "image/url", "desc").accounts({
      userAccount: myAccount.publicKey
    }).signers([myAccount]).rpc();
    console.log("Your transaction signature", tx);

    const myAcc = await program.account.postAccount.fetch(myAccount.publicKey);
    // console.log(myAcc);
    expect("title").equal(myAcc.title);


  });

});
