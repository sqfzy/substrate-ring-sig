# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

A Polkadot SDK parachain (Cumulus-based, parachain ID 1000) extended with a custom anonymous voting system. The core contribution is `pallets/ring_sig_voting` — a pallet that implements bLSAG ring-signature-based anonymous voting with ECDH-encrypted ballots and optimistic tallying.

## Build commands

```bash
# Build runtime WASM only (fast, for runtime-only changes)
cargo build --profile production

# Build the node binary (not in default-members, must pass --workspace)
cargo build --release --workspace

# Run all tests without building WASM (much faster for pallet dev)
SKIP_WASM_BUILD=1 cargo test

# Run tests for a specific pallet
SKIP_WASM_BUILD=1 cargo test -p ring-sig-voting

# Run a single test by name
SKIP_WASM_BUILD=1 cargo test -p ring-sig-voting vote_works

# Lint
SKIP_WASM_BUILD=1 cargo clippy --all-targets --all-features

# Run benchmarks for ring_sig_voting (requires release + runtime-benchmarks feature)
cargo build --release --features runtime-benchmarks --workspace
./target/release/parachain-template-node benchmark pallet \
  --pallet "ring_sig_voting" --extrinsic "*" \
  --steps 50 --repeat 20 --output pallets/ring_sig_voting/src/weights.rs
```

**Important**: `SKIP_WASM_BUILD=1` is required for fast iteration on pallets — omitting it triggers a full WASM compile on every `cargo test` or `cargo clippy` invocation.

## Workspace layout

- `node/` — binary node (not in default workspace members; compile with `--workspace`)
- `runtime/` — runtime composition, wires all pallets together
- `pallets/ring_sig_voting/` — core custom pallet (anonymous voting)
- `pallets/pallet_liquidity_pool/` — AMM/DEX pallet
- `pallets/template/`, `pallets/custom-pallet/` — boilerplate
- `frontend/` — SvelteKit UI + Rust→WASM crypto library

## Core pallet: `ring_sig_voting`

### Architecture

The pallet uses **unsigned transactions** (`ensure_none()` + `ValidateUnsigned`) for the `vote` extrinsic to prevent linking ballots to account identities at the network layer. Double-spend prevention is via key images stored in `UsedKeyImages<T>`.

Key-image and signature validation also runs in `validate_unsigned` (mempool pre-validation), so invalid votes are rejected before entering consensus.

### Storage

| Map | Key → Value | Purpose |
|---|---|---|
| `StudentKeys` | `StudentId(u32)` → `CompressedRistrettoWrapper` | Admin-registered public keys |
| `Rings` | `RingId(u32)` → `BoundedVec<CompressedRistretto, MaxRingSize>` | Voting groups |
| `Polls` | `PollId(u32)` → `Poll` struct | Voting metadata + state |
| `EncryptedVotes` | `PollId` → `BoundedVec<EncryptedVote>` | Ciphertexts |
| `UsedKeyImages` | `(PollId, KeyImage)` → `()` | Double-vote prevention |
| `Teachers` | `AccountId` → `()` | Poll-creation authorization |

### Poll state machine

`Active` → `Tallying` → `Completed` (or `Cancelled`/`Paused` from `Active`).

The `Active → Tallying` transition is scheduled automatically at `deadline` blocks using `pallet_scheduler`. The `Tallying → Completed` transition requires calling `tally(poll_id, claimed_tally, private_key)` — the chain verifies `private_key · G == poll_public_key` then stores the private key publicly for anyone to audit.

### Message binding for signatures

The signed message for bLSAG is:

```
message = genesis_hash || poll_id || key_image || ephemeral_public_key || ciphertext
```

The same concatenation is used as AAD for ChaCha20-Poly1305, binding each ciphertext to a specific chain instance, poll, and voter.

### Crypto dependencies

- `nazgul 2.1` — bLSAG ring signature (sign + verify)
- `curve25519-dalek 4.1.3` — Ristretto group operations
- `blake2 0.10` — hash function passed to `BLSAG::verify`

### Config constants (set in `runtime/src/configs/mod.rs`)

| Constant | Value | Meaning |
|---|---|---|
| `MaxRingSize` | 32 | Max ring members per poll |
| `MaxDescriptionLength` | 256 | Bytes for poll description |
| `MaxCiphertextLength` | 128 | Bytes per encrypted vote |
| `MaxVoteNum` | 1000 | Max ballots per poll |
| `AdminOrigin` | `EnsureRoot` | Only sudo can call admin extrinsics |

## Runtime

Pallet indices relevant to this project:

| Index | Pallet |
|---|---|
| 52 | `pallet_preimage` — stores poll metadata hashes |
| 53 | `pallet_scheduler` — drives automatic deadline transitions |
| 56 | `ring_sig_voting` |
| 57 | `pallet_liquidity_pool` |

Parachain ID is hardcoded as `PARACHAIN_ID = 1000` in `runtime/src/lib.rs`.

## Frontend (`frontend/`)

The frontend is a SvelteKit app with a Rust→WASM crypto module.

```bash
# Install deps
cd frontend && npm install

# Build the WASM crypto library (must do before running the app)
npm run build:wasm          # compiles frontend/crypto → frontend/src/lib/wasm/

# Start dev server
npm run dev
```

The `frontend/crypto/` crate uses the same `curve25519-dalek` + `nazgul` + `chacha20poly1305` stack as the pallet, compiled to WASM via `wasm-pack`. This keeps private keys client-side.

Routes: `/student` (voting), `/teacher` (create polls), `/admin` (key/ring management).

## Local development chain

Two options:

**Option A — Omni Node (simpler, no relay chain):**
```bash
# Generate chain spec
chain-spec-builder create --relay-chain "rococo-local" --para-id 1000 \
  --runtime target/release/wbuild/parachain-template-runtime/parachain_template_runtime.wasm \
  named-preset development

polkadot-omni-node --chain chain_spec.json --dev --dev-block-time 1000
```

**Option B — Full relay chain via Zombienet:**
```bash
# Requires polkadot, polkadot-prepare-worker, polkadot-execute-worker on PATH
zombienet --provider native spawn zombienet.toml          # uses parachain-template-node
zombienet --provider native spawn zombienet-omni-node.toml  # uses polkadot-omni-node
```

Parachain RPC: `ws://localhost:9988` — relay chain: `ws://localhost:9944`.

## Benchmark weights

Generated weight files live in `pallets/ring_sig_voting/src/weights.rs`. The `bench结果.txt` file contains historical results for `pallet_liquidity_pool`. When modifying any extrinsic in `ring_sig_voting`, re-run benchmarks and regenerate `weights.rs`.
