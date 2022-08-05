import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OrbitMarketAccounts } from "../target/types/orbit_market_accounts";

describe("orbit-market-accounts", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.OrbitMarketAccounts as Program<OrbitMarketAccounts>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
