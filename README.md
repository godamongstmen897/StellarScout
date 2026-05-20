StellarScout — On-Chain Audit Oracle

Developer tooling for the Stellar/Soroban ecosystem.

This repository contains:

- `analyzer/` — Rust CLI that performs static analysis of compiled WASM.
  - **Code Metrics**: Function counts, code size analysis, module statistics
  - **Complexity Analysis**: Cyclomatic complexity and nesting depth detection
  - **Performance Analysis**: Detects expensive operations and gas cost implications
  - **Reporting**: JSON and human-readable output formats
  
- `contract/` — Soroban registry contract skeleton to store security attestations. *(Currently in planning phase)*

## Quick start

### Installation

Ensure Rust is installed. Build with:

```bash
cargo build --release
```

### Usage

```bash
# Analyze a WASM contract with human-readable output
./target/release/stellar-scout path/to/contract.wasm

# Output as JSON
./target/release/stellar-scout path/to/contract.wasm --format json
```

### Output Example

The analyzer provides:

- **Module Metrics**: Total functions, imports/exports, memory usage
- **Complexity Metrics**: Function complexity scores, nesting depths
- **Performance Insights**: Expensive operations, gas risk assessment
- **Actionable Recommendations**: High-complexity functions, optimization opportunities

## Features

- ✅ WASM static analysis
- ✅ Code metrics and complexity analysis
- ✅ Performance optimization detection
- ✅ Multiple output formats
- 🚧 Security vulnerability detection (planned)
- 🚧 Registry contract implementation (planned)
