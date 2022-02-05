import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaTokens } from "../target/types/solana_tokens";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  Token
} from "@solana/spl-token";

const NETWORK = anchor.web3.clusterApiUrl("devnet");
const Connection = new anchor.web3.Connection(NETWORK);
describe("solana-tokens", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaTokens as Program<SolanaTokens>;
  let mint = anchor.web3.Keypair.generate();
  let token_account = anchor.web3.Keypair.generate();
  it("Is initialized!", async () => {
    // Add your test here.
    let amount = 10;
    const tx = await program.rpc.createAndMint(
      new anchor.BN(amount * 10 ** 6),
      {
        accounts: {
          mint: mint.publicKey,
          payer: provider.wallet.publicKey,
          mintTo: token_account.publicKey,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [token_account, mint],
      }
    );
    console.log("TOKENS CREATED : ", mint.publicKey.toBase58());
    console.log("TOKENS MINTED TO : ", token_account.publicKey.toBase58());
    console.log("TRANSACTION SIGNATURE : ", tx);
  });

  it("Burn Tokens" , async () => {
    let amount = 2;
    const tx = await program.rpc.burnMint(new anchor.BN(amount *10 **6),{
      accounts:{
        mint : mint.publicKey,
        source : token_account.publicKey,
        payer : provider.wallet.publicKey,
        tokenProgram : TOKEN_PROGRAM_ID
      }
    })

    console.log("TOKENS BURNED")
    console.log("TRANSACTION SIGNATURE" , tx)
  })

  it("Burn Tokens" , async () => {
    let amount = 2;
    let destination_account = anchor.web3.Keypair.generate();
    let receiver = new anchor.web3.PublicKey("DLRrs6yTYZJkFiM5yV1mwDsU8b9bK4VHXA1NFdEiwHhz");
    const tx = await program.rpc.transfer(new anchor.BN(amount *10 **6),{
      accounts:{
       source: token_account.publicKey,
       mint : mint.publicKey,
       destination : destination_account.publicKey,
       destinationOwner : receiver,
       payer : provider.wallet.publicKey,
       tokenProgram : TOKEN_PROGRAM_ID,
       systemProgram : anchor.web3.SystemProgram.programId,
       rent : anchor.web3.SYSVAR_RENT_PUBKEY
      },
      signers : [destination_account]
    })

    console.log("TOKENS TRANSFERED")
    console.log("TRANSACTION SIGNATURE" , tx)
  })
});
