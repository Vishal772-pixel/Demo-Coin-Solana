import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import { DemoCoin } from "../target/types/demo_coin";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress, createMint, mintTo } from "@solana/spl-token";

describe("demo_coin",()=>{
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DemoCoin as Program;

  it("Intialize  mint and mint tokeds", async ()=>{
    const mintKeypair = anchor.web3.Keypair.generate();
    const authority = provider.wallet;

    // Derive ATA (associated Token Account ) for user
    const recipientTokenAccount = await getAssociatedTokenAddress
    (
      mintKeypair.publicKey,
      authority.publicKey
    );


   // Call intialize_mint

   await program.methods
   .intializeMint(new anchor.BN(6)) // 6 decimels 
   .accounts({
    mint:mintKeypair.publicKey,
    authority:authority.publicKey,
    rent:anchor.web3.SYSVAR_RENT_PUBKEY,
    systemProgram:SystemProgram.programId,
    tokenProgram:TOKEN_PROGRAM_ID,
   })
   .signers([mintKeypair])
   .rpc();


console.log("Token Minted to :",recipientTokenAccount.toBase58());


  });
});1
