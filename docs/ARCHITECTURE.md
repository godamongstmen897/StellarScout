# StellarScout Architecture

## Overview

StellarScout is a modular WASM static analysis tool designed to provide comprehensive insights into Soroban smart contracts. The architecture separates concerns into distinct analysis modules that work together to produce actionable security and performance insights.

## Project Structure

```
StellarScout/
├── analyzer/           # Main analysis engine
│   ├── src/
│   │   ├── lib.rs     # Public API exports
│   │   ├── main.rs    # CLI entry point
│   │   ├── analysis.rs        # Core analyzer orchestration
│   │   ├── metrics.rs         # Module metrics collection
│   │   ├── complexity.rs      # Complexity analysis
│   │   ├── performance.rs     # Performance analysis
│   │   └── report.rs          # Report formatting
│   └── Cargo.toml
├── contract/          # Soroban registry contract (planned)
└── docs/              # Documentation
```

## Core Modules

### 1. **WasmAnalyzer** (`analysis.rs`)
The main orchestrator that coordinates all analysis modules.

**Responsibilities:**
- Parse WASM binary data using `wasmparser`
- Aggregate analysis results from all modules
- Generate comprehensive report

**Key API:**
```rust
pub struct WasmAnalyzer {
    data: Vec<u8>,
}

impl WasmAnalyzer {
    pub fn new(data: &[u8]) -> Result<Self>
    pub fn analyze(&self) -> Result<Report>
}
```

### 2. **Metrics Module** (`metrics.rs`)
Collects static module metrics from WASM binary structure.

**Metrics Collected:**
- Total function count
- Imported vs exported functions
- Code section size
- Total module size
- Import/export counts
- Memory pages allocated
- Table count

**Output:** `ModuleMetrics` struct serializable to JSON

### 3. **Complexity Module** (`complexity.rs`)
Analyzes code complexity at the function level.

**Metrics Computed:**
- **Cyclomatic Complexity**: Number of decision points (if/loop operators)
- **Maximum Nesting Depth**: Deepest block nesting level

**Algorithm:**
1. Iterate through each function's operators
2. Count decision points (If, Loop operators)
3. Track nesting depth (Block entry/exit)
4. Flag functions exceeding thresholds (CC > 10, nesting > 5)

**Output:** `ComplexityAnalysis` with high-complexity function list

### 4. **Performance Module** (`performance.rs`)
Detects expensive operations and estimates gas costs.

**Expensive Operations Tracked:**
- `memory.grow` - Allocates new memory (3000 gas per operation)
- `table.grow` - Extends table (3000 gas per operation)
- `memory.copy` - Copies memory regions (1 gas per byte)
- `memory.fill` - Fills memory (1 gas per byte)

**Risk Assessment:**
- HIGH: > 50,000 total estimated gas
- MEDIUM: 10,000 - 50,000 gas
- LOW: < 10,000 gas

**Output:** `PerformanceAnalysis` with risk level and operation details

### 5. **Report Module** (`report.rs`)
Formats analysis results for human and machine consumption.

**Supported Formats:**
- **JSON**: Full structured output for tooling integration
- **Human**: Formatted text with Unicode box drawing for CLI display

**Output Example:**
```
╔══════════════════════════════════════════════╗
║         STELLAR SCOUT WASM ANALYSIS          ║
╚══════════════════════════════════════════════╝

📊 MODULE METRICS
─────────────────────────────────────
Total Functions:        15
Imported Functions:     3
...
```

## Data Flow

```
WASM Binary
    ↓
WasmAnalyzer::new()  [validates binary]
    ↓
WasmAnalyzer::analyze()
    ├→ WpParser::parse_all()  [tokenize WASM]
    ├→ ModuleMetrics::from_payloads()
    ├→ ComplexityAnalysis::from_payloads()
    ├→ PerformanceAnalysis::from_payloads()
    └→ Report { metrics, complexity, performance }
    ↓
Report::format()
    ├→ ReportFormat::Json
    └→ ReportFormat::Human
    ↓
Output to user
```

## Public API

Users interact with StellarScout through the library's public API:

```rust
// File-based analysis
pub fn analyze_file(path: &Path) -> Result<Report>

// Direct binary analysis (used internally by CLI)
pub struct WasmAnalyzer { ... }
impl WasmAnalyzer {
    pub fn new(data: &[u8]) -> Result<Self>
    pub fn analyze(&self) -> Result<Report>
}

// Report formatting
pub enum ReportFormat { Json, Human }
impl Report {
    pub fn format(&self, format: ReportFormat) -> Result<String>
}
```

## Error Handling

All modules use Rust's `Result<T>` for error propagation:
- WASM parsing errors are surfaced from `wasmparser`
- Invalid binary format returns descriptive error
- All serialization errors are captured

## Dependencies

### Core Dependencies:
- **wasmparser 0.89.0** - Official WebAssembly binary parser
- **serde/serde_json** - Serialization framework
- **clap** - Command-line argument parsing
- **anyhow** - Error handling

### Design Principles:
1. **Modularity**: Each analysis type is independent
2. **Composability**: Results aggregate into unified report
3. **Performance**: Single-pass WASM parsing where possible
4. **Usability**: Both programmatic (JSON) and human (text) outputs

## Future Enhancements

### Contract Module (`contract/`)
A Soroban smart contract to store and verify security attestations:
- On-chain registry of audited contracts
- Attestation storage and validation
- Integration with Soroban chain

### Security Analysis (Planned)
- Pattern-based vulnerability detection
- Known malicious patterns database
- Security score computation
