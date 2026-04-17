# Match Types

## Price

A price match commits to a 32-byte oracle target (e.g. `SOL/USD`) and a
challenger side. At creation, the program records `snapshot_start` from the
configured oracle. Settlement requires the authority to submit
`snapshot_end`. The challenger wins if their declared direction matches the
sign of `snapshot_end - snapshot_start`.

## Time

A time match is the simplest. Both stakes are locked. The first wallet that
sends a SPL transfer touching its escrow before the timeout loses. If the
timeout passes with no movement, the match resolves into a draw and refunds
both stakes minus the protocol fee.

## VRF

A VRF match requests randomness from the Switchboard queue declared in
`Anchor.toml`. On callback the program reads the first byte of the
randomness buffer and uses bit zero to pick the winner. Both wallets are
held in escrow until the callback resolves.

## Cancel paths

Open challenges (no acceptor) can be cancelled by the issuer once they are
past their expiry. Active matches can be cancelled by either party only if
the configured oracle has been unavailable for longer than the cancellation
window (default 24h).

<!-- rev12 -->

<!-- rev13 -->

<!-- rev20 -->

<!-- rev21 -->

<!-- rev30 -->

<!-- rev74 -->

<!-- rev77 -->

<!-- rev89 -->

<!-- rev123 -->

<!-- rev128 -->

<!-- rev141 -->

<!-- rev143 -->

<!-- rev146 -->

<!-- rev213 -->

<!-- rev215 -->

<!-- rev223 -->

<!-- rev233 -->

<!-- rev238 -->

<!-- rev262 -->
