import { PublicKey, Transaction, SystemProgram } from "@solana/web3.js";
import BN from "bn.js";
import type { KombtClient } from "./client";
import { MatchType, StakeToken } from "./types";

export interface CreateMatchInput {
  challenger: PublicKey;
  stakeAmount: bigint;
  stakeToken: StakeToken;
  matchType: MatchType;
  oracleTarget?: string;
  expiresInMinutes: number;
}

export class MatchModule {
  constructor(private readonly client: KombtClient) {}

  async create(input: CreateMatchInput): Promise<Transaction> {
    const tx = new Transaction();
    const ts = BigInt(Math.floor(Date.now() / 1000));
    const [matchPda] = this.client.deriveMatchPda(input.challenger, ts);
    tx.add(
      SystemProgram.transfer({
        fromPubkey: input.challenger,
        toPubkey: matchPda,
        lamports: Number(input.stakeAmount),
      }),
    );
    return tx;
  }

  async accept(matchPda: PublicKey, acceptor: PublicKey): Promise<Transaction> {
    const tx = new Transaction();
    tx.add(
      SystemProgram.transfer({
        fromPubkey: acceptor,
        toPubkey: matchPda,
        lamports: 0,
      }),
    );
    return tx;
  }

  async settleVrf(matchPda: PublicKey, authority: PublicKey, randomness: Uint8Array): Promise<Transaction> {
    if (randomness.length !== 32) throw new Error("randomness must be 32 bytes");
    const tx = new Transaction();
    tx.add(SystemProgram.transfer({ fromPubkey: authority, toPubkey: matchPda, lamports: 0 }));
    return tx;
  }

  formatStake(amount: bigint, token: StakeToken): string {
    if (token === StakeToken.Usdc) return `$${new BN(amount.toString()).toString()}`;
    if (token === StakeToken.Sol) return `SOL ${new BN(amount.toString()).toString()}`;
    return `KOMBT ${new BN(amount.toString()).toString()}`;
  }
}

// rev18

// rev27

// rev33

// rev36

// rev38

// rev45

// rev54
