# API Reference

## Core Structs

### WasmAnalyzer

Main analysis orchestrator for WASM binaries.

```rust
pub struct WasmAnalyzer {
    data: Vec<u8>,
}
```

#### Methods

**`pub fn new(data: &[u8]) -> Result<Self>`**

Creates a new analyzer from raw WASM binary data.

- **Parameters:**
  - `data`: Raw bytes of a WASM module
- **Returns:** `Result<WasmAnalyzer, Error>`
- **Errors:** Returns error if data is not valid WASM

**Example:**
```rust
let wasm_bytes = std::fs::read("contract.wasm")?;
let analyzer = WasmAnalyzer::new(&wasm_bytes)?;
```

---

**`pub fn analyze(&self) -> Result<Report>`**

Performs complete analysis of the WASM module.

- **Returns:** `Result<Report, Error>` with full analysis
- **Errors:** Parse or analysis errors are returned

**Example:**
```rust
let report = analyzer.analyze()?;
println!("Functions: {}", report.metrics.total_functions);
```

---

### Report

Container for all analysis results.

```rust
pub struct Report {
    pub metrics: ModuleMetrics,
    pub complexity: ComplexityAnalysis,
    pub performance: PerformanceAnalysis,
}
```

#### Methods

**`pub fn format(&self, format: ReportFormat) -> Result<String>`**

Formats the report into human or JSON format.

- **Parameters:**
  - `format`: `ReportFormat::Human` or `ReportFormat::Json`
- **Returns:** Formatted string

**Example:**
```rust
let human = report.format(ReportFormat::Human)?;
let json = report.format(ReportFormat::Json)?;
println!("{}", human);
```

---

### ModuleMetrics

Static metrics about the WASM module structure.

```rust
pub struct ModuleMetrics {
    pub total_functions: usize,
    pub imported_functions: usize,
    pub exported_functions: usize,
    pub code_section_size: usize,
    pub total_module_size: usize,
    pub imports_count: usize,
    pub exports_count: usize,
    pub memory_pages: Option<u32>,
    pub table_count: usize,
}
```

**Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `total_functions` | `usize` | Total number of functions (imported + internal) |
| `imported_functions` | `usize` | Functions imported from external modules |
| `exported_functions` | `usize` | Functions exported for external callers |
| `code_section_size` | `usize` | Size of compiled code in bytes |
| `total_module_size` | `usize` | Total size of entire module in bytes |
| `imports_count` | `usize` | Total number of imports (any type) |
| `exports_count` | `usize` | Total number of exports (any type) |
| `memory_pages` | `Option<u32>` | Initial memory allocation (64KB per page) |
| `table_count` | `usize` | Number of table sections |

---

### ComplexityAnalysis

Code complexity metrics at the function level.

```rust
pub struct ComplexityAnalysis {
    pub functions_analyzed: usize,
    pub high_complexity_functions: Vec<FunctionComplexity>,
    pub average_complexity: f64,
}

pub struct FunctionComplexity {
    pub function_index: usize,
    pub cyclomatic_complexity: usize,
    pub max_nesting_depth: usize,
}
```

**ComplexityAnalysis Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `functions_analyzed` | `usize` | Total functions analyzed |
| `high_complexity_functions` | `Vec<FunctionComplexity>` | Functions exceeding complexity threshold (CC > 10) |
| `average_complexity` | `f64` | Mean cyclomatic complexity across all functions |

**FunctionComplexity Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `function_index` | `usize` | Index of function in module |
| `cyclomatic_complexity` | `usize` | Number of decision paths (lower is better) |
| `max_nesting_depth` | `usize` | Deepest block nesting level |

---

### PerformanceAnalysis

Performance and gas cost analysis.

```rust
pub struct PerformanceAnalysis {
    pub expensive_operations: Vec<ExpensiveOp>,
    pub memory_operations_count: usize,
    pub call_count: usize,
    pub gas_risk_level: String,
}

pub struct ExpensiveOp {
    pub operation: String,
    pub count: usize,
    pub estimated_gas_cost: usize,
}
```

**PerformanceAnalysis Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `expensive_operations` | `Vec<ExpensiveOp>` | List of expensive ops found (sorted by cost) |
| `memory_operations_count` | `usize` | Total memory operations (grow, copy, fill) |
| `call_count` | `usize` | Total function calls (direct and indirect) |
| `gas_risk_level` | `String` | Risk classification: "LOW", "MEDIUM", or "HIGH" |

**ExpensiveOp Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `operation` | `String` | Operation name (e.g., "memory.grow") |
| `count` | `usize` | Number of times operation appears |
| `estimated_gas_cost` | `usize` | Total estimated gas for all occurrences |

---

## Functions

**`pub fn analyze_file(path: &Path) -> Result<Report>`**

Convenience function to analyze a WASM file directly.

- **Parameters:**
  - `path`: File system path to WASM binary
- **Returns:** `Result<Report, Error>`
- **Errors:** File I/O or WASM parse errors

**Example:**
```rust
use std::path::Path;
let report = analyze_file(Path::new("contract.wasm"))?;
```

---

## Enums

### ReportFormat

Output format specification.

```rust
pub enum ReportFormat {
    Json,
    Human,
}
```

**Variants:**

- `Json`: Machine-readable JSON format (pretty-printed)
- `Human`: Human-readable text with Unicode formatting

---

## Error Handling

All operations return `Result<T>` using `anyhow::Result`.

```rust
pub type Result<T> = std::result::Result<T, anyhow::Error>;
```

**Common Errors:**

- Invalid WASM magic bytes
- Truncated WASM binary
- Unsupported WASM version
- Malformed sections
- Invalid operator sequences

**Error Propagation:**

```rust
match WasmAnalyzer::new(&data) {
    Ok(analyzer) => {
        match analyzer.analyze() {
            Ok(report) => println!("Analysis: {:?}", report),
            Err(e) => eprintln!("Analysis failed: {}", e),
        }
    }
    Err(e) => eprintln!("Invalid WASM: {}", e),
}
```

---

## Serialization

All structs implement `serde::Serialize` and `serde::Deserialize`.

```rust
use serde_json;

let json_str = serde_json::to_string_pretty(&report)?;
let report: Report = serde_json::from_str(&json_str)?;
```

---

## Usage Examples

### Minimal Example

```rust
use stellar_scout_analyzer::analyze_file;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let report = analyze_file(Path::new("contract.wasm"))?;
    println!("{:?}", report);
    Ok(())
}
```

### With Error Handling

```rust
use stellar_scout_analyzer::analyze_file;
use std::path::Path;

fn main() {
    match analyze_file(Path::new("contract.wasm")) {
        Ok(report) => {
            println!("Functions: {}", report.metrics.total_functions);
        }
        Err(e) => {
            eprintln!("Failed to analyze: {}", e);
            std::process::exit(1);
        }
    }
}
```

### Custom Analysis

```rust
use stellar_scout_analyzer::WasmAnalyzer;

fn check_contract_safety(data: &[u8]) -> Result<(), String> {
    let analyzer = WasmAnalyzer::new(data)
        .map_err(|e| format!("Invalid WASM: {}", e))?;
    
    let report = analyzer.analyze()
        .map_err(|e| format!("Analysis failed: {}", e))?;
    
    if report.performance.gas_risk_level == "HIGH" {
        return Err("Gas risk too high".to_string());
    }
    
    if !report.complexity.high_complexity_functions.is_empty() {
        return Err("High complexity functions detected".to_string());
    }
    
    Ok(())
}
```
