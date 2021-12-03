const anchor = require('@project-serum/anchor');
const { SystemProgram,  Connection, clusterApiUrl, } = require('@solana/web3.js');
const { assert } = require('chai');

describe('my-project', () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.MyProject;

  const loteryAccount = anchor.web3.Keypair.generate();
  it('Is initialized!', async () => {
    const tx = await program.rpc.initialize({
      accounts: {
        loteryAccount: loteryAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [loteryAccount]
    });
    console.log("Your transaction signature", tx);
  });

  it('Fetching lotery account data', async () => {
    const loteryData = await fetchProgramData();
    assert(loteryData.totalBets == 0);
  });

  it('Adding a new bet to lotery', async () => {
    await program.rpc.newBet("Hello, world", new anchor.BN(5), {
      accounts: {
        loteryAccount: loteryAccount.publicKey,
        user: provider.wallet.publicKey,
      }
    });
    const loteryData = await fetchProgramData();
    assert(loteryData.totalBets == 1);
    assert(loteryData.listBets[0].message === "Hello, world", "Message discrepancy")
  });

  it('Prize drawn', async () => {
    // TODO: Write prize drawn test
  });
  
  async function fetchProgramData() {
    return program.account.loteryAccount.fetch(loteryAccount.publicKey);
  }
});

