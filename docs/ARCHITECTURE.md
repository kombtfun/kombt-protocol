# Architecture

```mermaid
%%{init: {"theme":"base","themeVariables":{"background":"#1a0f08","primaryColor":"#1f1409","primaryBorderColor":"#d69342","primaryTextColor":"#e8d4a8","lineColor":"#c99830","secondaryColor":"#8b2727","tertiaryColor":"#8d6f3f","fontFamily":"monospace"}}}%%
flowchart TB
    SDK[kombt-sdk TypeScript] --> RPC[(Solana RPC)]
    RPC --> PG[Anchor Program kombt]
    PG --> ME[(MatchAccount PDA)]
    PG --> CH[(ChallengeAccount PDA)]
    ME --> SB[Switchboard VRF Queue]
    ME --> JP[Jupiter / Pyth Snapshot]
    PG --> BR[KOMBT Burn Wallet]
```

The Anchor program is the only authority for moving stakes between
participants. The TypeScript SDK exposes account derivation, transaction
builders, and helper formatting utilities.

## Layered responsibilities

- `programs/kombt`: on-chain primitives. Owns escrow, settlement, and burn.
- `sdk/typescript`: client-side helpers used by the website, telegram bot, or
  external integrations.
- `docs`: human-facing reference for integrators.

## Settlement primitives

Each match is settled by exactly one of three primitives:

- VRF — Switchboard randomness.
- Price — single integer snapshot at expiry.
- Time — first wallet that touches its balance loses.

<!-- rev0 -->

<!-- rev1 -->

<!-- rev29 -->

<!-- rev43 -->

<!-- rev80 -->

<!-- rev82 -->

<!-- rev87 -->

<!-- rev95 -->

<!-- rev106 -->

<!-- rev115 -->

<!-- rev126 -->

<!-- rev133 -->

<!-- rev134 -->

<!-- rev149 -->

<!-- rev159 -->

<!-- rev178 -->
