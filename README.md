StellarScout — On-Chain Audit Oracle

Developer tooling for the Stellar/Soroban ecosystem.

This repository contains:

- `analyzer/` — Rust CLI that performs static analysis of compiled WASM.
- `contract/` — Soroban registry contract skeleton to store security attestations.

Quick start (once Rust is installed):

```bash
cd analyzer
cargo run -- path/to/contract.wasm
```
