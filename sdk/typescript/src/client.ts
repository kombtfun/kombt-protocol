import { Connection, PublicKey } from "@solana/web3.js";
import { MatchModule } from "./match";
import { ChallengeModule } from "./challenge";

export interface KombtClientConfig {
  rpc: string;
  programId: string;
  commitment?: "processed" | "confirmed" | "finalized";
}

export class KombtClient {
  readonly connection: Connection;
  readonly programId: PublicKey;
  readonly match: MatchModule;
  readonly challenge: ChallengeModule;

  private constructor(connection: Connection, programId: PublicKey) {
    this.connection = connection;
    this.programId = programId;
    this.match = new MatchModule(this);
    this.challenge = new ChallengeModule(this);
  }

  static async connect(cfg: KombtClientConfig): Promise<KombtClient> {
    const connection = new Connection(cfg.rpc, cfg.commitment ?? "confirmed");
    const programId = new PublicKey(cfg.programId);
    return new KombtClient(connection, programId);
  }

  deriveMatchPda(challenger: PublicKey, ts: bigint): [PublicKey, number] {
    const seedBuf = Buffer.alloc(8);
    seedBuf.writeBigInt64LE(ts);
    return PublicKey.findProgramAddressSync(
      [Buffer.from("match"), challenger.toBuffer(), seedBuf],
      this.programId,
    );
  }

  deriveChallengePda(issuer: PublicKey, ts: bigint): [PublicKey, number] {
    const seedBuf = Buffer.alloc(8);
    seedBuf.writeBigInt64LE(ts);
    return PublicKey.findProgramAddressSync(
      [Buffer.from("challenge"), issuer.toBuffer(), seedBuf],
      this.programId,
    );
  }
}

// rev11

// rev27

// rev46

// rev50

// rev76
