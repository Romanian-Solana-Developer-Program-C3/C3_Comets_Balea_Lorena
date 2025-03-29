import "dotenv/config";

// this sets up that secret key we need to check the balance
const secretKey = process.env.SECRET_KEY;
if (!secretKey) {
  throw new Error("SECRET_KEY is missing from environment variables!");
}


import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";

import {
  airdropIfRequired,
  getKeypairFromEnvironment,
} from "@solana-developers/helpers";

const keypair = getKeypairFromEnvironment("SECRET_KEY");
const connection = new Connection(clusterApiUrl("devnet"));

const pubkey = keypair.publicKey;

await airdropIfRequired(connection, pubkey, LAMPORTS_PER_SOL, LAMPORTS_PER_SOL);

const balanceInLamports = await connection.getBalance(pubkey);

console.log(`${pubkey.toString()} has balance ${balanceInLamports}`);
