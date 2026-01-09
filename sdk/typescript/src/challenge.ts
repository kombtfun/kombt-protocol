import { PublicKey, Transaction, SystemProgram } from "@solana/web3.js";
import type { KombtClient } from "./client";
import { MatchType, StakeToken } from "./types";

export interface IssueChallengeInput {
  issuer: PublicKey;
  stakeAmount: bigint;
  stakeToken: StakeToken;
  matchType: MatchType;
  oracleTarget?: string;
  durationMinutes: number;
}

export class ChallengeModule {
  constructor(private readonly client: KombtClient) {}

  async issue(input: IssueChallengeInput): Promise<Transaction> {
    const tx = new Transaction();
    const ts = BigInt(Math.floor(Date.now() / 1000));
    const [pda] = this.client.deriveChallengePda(input.issuer, ts);
    tx.add(
      SystemProgram.transfer({
        fromPubkey: input.issuer,
        toPubkey: pda,
        lamports: Number(input.stakeAmount),
      }),
    );
    return tx;
  }

  async cancel(pda: PublicKey, authority: PublicKey): Promise<Transaction> {
    const tx = new Transaction();
    tx.add(SystemProgram.transfer({ fromPubkey: authority, toPubkey: pda, lamports: 0 }));
    return tx;
  }
}

// rev1

// rev3

// rev6
