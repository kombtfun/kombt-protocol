# Getting Started

## Devnet walkthrough

```bash
# 1. install toolchains
solana-install init 1.18.20
avm install 0.30.1
avm use 0.30.1

# 2. clone + build
git clone https://github.com/kombtfun/kombt-protocol
cd kombt-protocol
anchor build

# 3. devnet deploy
solana config set --url https://api.devnet.solana.com
anchor deploy --provider.cluster devnet

# 4. SDK install
cd sdk/typescript
npm install
npm run build
```

## Smoke matches

```ts
import { KombtClient, MatchType, StakeToken } from "kombt-sdk";

const client = await KombtClient.connect({
  rpc: "https://api.devnet.solana.com",
  programId: "ELbd3fkvCeqDCrHdoZ2FLzCyo5jNynik2bLzpSFyGcqs",
});

const tx = await client.match.create({
  challenger: wallet.publicKey,
  stakeAmount: 1_000_000n,
  stakeToken: StakeToken.Kombt,
  matchType: MatchType.Vrf,
  expiresInMinutes: 30,
});
```

<!-- rev2 -->

<!-- rev14 -->

<!-- rev19 -->

<!-- rev30 -->

<!-- rev31 -->

<!-- rev53 -->

<!-- rev57 -->

<!-- rev98 -->

<!-- rev99 -->

<!-- rev161 -->
