export enum MatchType {
  Price = 0,
  Time = 1,
  Vrf = 2,
}

export enum StakeToken {
  Kombt = 0,
  Sol = 1,
  Usdc = 2,
}

export enum MatchState {
  Pending = 0,
  Active = 1,
  Settled = 2,
  Cancelled = 3,
}

export interface MatchSummary {
  pda: string;
  challenger: string;
  acceptor: string | null;
  stakeAmount: bigint;
  stakeToken: StakeToken;
  matchType: MatchType;
  state: MatchState;
  expiresAt: number;
}

export interface ChallengeSummary {
  pda: string;
  issuer: string;
  targetHandle?: string;
  stakeAmount: bigint;
  stakeToken: StakeToken;
  matchType: MatchType;
  durationMinutes: number;
  expiresAt: number;
}

// rev2

// rev40

// rev44

// rev117

// rev135

// rev181

// rev231
