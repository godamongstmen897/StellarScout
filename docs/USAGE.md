# Usage Guide

## Command Line Usage

### Basic Analysis

Analyze a WASM contract with human-readable output:

```bash
./target/release/stellar-scout path/to/contract.wasm
```

Output example:
```
╔══════════════════════════════════════════════╗
║         STELLAR SCOUT WASM ANALYSIS          ║
╚══════════════════════════════════════════════╝

📊 MODULE METRICS
─────────────────────────────────────
Total Functions:        25
Imported Functions:     5
Exported Functions:     8
Code Section Size:      4096 bytes
Total Module Size:      8192 bytes
...
```

### JSON Output

For programmatic consumption or integration with other tools:

```bash
./target/release/stellar-scout path/to/contract.wasm --format json
```

Output:
```json
{
  "metrics": {
    "total_functions": 25,
    "imported_functions": 5,
    "exported_functions": 8,
    "code_section_size": 4096,
    "total_module_size": 8192,
    "imports_count": 6,
    "exports_count": 8,
    "memory_pages": 1,
    "table_count": 0
  },
  "complexity": {
    "functions_analyzed": 25,
    "high_complexity_functions": [
      {
        "function_index": 12,
        "cyclomatic_complexity": 15,
        "max_nesting_depth": 6
      }
    ],
    "average_complexity": 4.2
  },
  "performance": {
    "expensive_operations": [
      {
        "operation": "memory.grow",
        "count": 3,
        "estimated_gas_cost": 9000
      }
    ],
    "memory_operations_count": 5,
    "call_count": 18,
    "gas_risk_level": "LOW"
  }
}
```

## Library Usage

### Basic File Analysis

```rust
use stellar_scout_analyzer::analyze_file;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let report = analyze_file(Path::new("contract.wasm"))?;
    println!("{:?}", report.metrics);
    Ok(())
}
```

### Binary Data Analysis

```rust
use stellar_scout_analyzer::WasmAnalyzer;

fn main() -> anyhow::Result<()> {
    let wasm_data = std::fs::read("contract.wasm")?;
    let analyzer = WasmAnalyzer::new(&wasm_data)?;
    let report = analyzer.analyze()?;
    
    // Access individual analyses
    println!("Total functions: {}", report.metrics.total_functions);
    println!("Average complexity: {:.2}", report.complexity.average_complexity);
    println!("Gas risk: {}", report.performance.gas_risk_level);
    
    Ok(())
}
```

### Custom Output Formatting

```rust
use stellar_scout_analyzer::{analyze_file, report::ReportFormat};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let report = analyze_file(Path::new("contract.wasm"))?;
    
    // Human-readable format
    let human = report.format(ReportFormat::Human)?;
    println!("{}", human);
    
    // JSON format
    let json = report.format(ReportFormat::Json)?;
    println!("{}", json);
    
    Ok(())
}
```

### Batch Analysis

```rust
use stellar_scout_analyzer::analyze_file;
use std::path::Path;
use std::fs;

fn analyze_contracts(dir: &str) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map(|e| e == "wasm").unwrap_or(false) {
            match analyze_file(&path) {
                Ok(report) => {
                    println!("✓ {}: {} functions, {} complexity avg",
                        path.display(),
                        report.metrics.total_functions,
                        report.complexity.average_complexity);
                }
                Err(e) => {
                    eprintln!("✗ {}: {}", path.display(), e);
                }
            }
        }
    }
    Ok(())
}
```

## Interpreting Results

### Metrics Section

- **Total Functions**: Count of all functions in the contract
- **Imported Functions**: Functions called from external modules
- **Exported Functions**: Functions accessible to contract callers
- **Code Section Size**: Compiled bytecode size (influences gas)
- **Memory Pages**: Initial memory allocation (64KB per page)

### Complexity Analysis

**Cyclomatic Complexity (CC):**
- Measures number of decision paths
- Lower is better (simpler to audit, less gas)
- Threshold warning: CC > 10 (high complexity)

**Nesting Depth:**
- Maximum block nesting level
- Higher nesting = harder to follow logic
- Threshold warning: depth > 5

**Recommendations:**
- Refactor functions with high complexity
- Flatten deeply nested logic
- Consider splitting complex functions

### Performance Analysis

**Gas Risk Levels:**
- **LOW** (< 10,000): Safe for most use cases
- **MEDIUM** (10k - 50k): Monitor expensive operations
- **HIGH** (> 50,000): Optimize before deployment

**Expensive Operations:**
- `memory.grow`: ~3000 gas (allocates new memory)
- `table.grow`: ~3000 gas (extends table)
- `memory.copy`: 1 gas per byte
- `memory.fill`: 1 gas per byte

**Optimization Tips:**
- Pre-allocate memory if pattern is known
- Batch memory operations when possible
- Avoid unnecessary table growth
- Use local variables instead of memory when feasible

## Integration Examples

### GitHub Actions CI

```yaml
name: Contract Analysis
on: [push, pull_request]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: rust-lang/rust-toolchain@v1
      - run: |
          cargo build --release
          ./target/release/stellar-scout contract.wasm --format json > analysis.json
          # Process analysis.json with custom script
      - uses: actions/upload-artifact@v3
        with:
          name: analysis-report
          path: analysis.json
```

### Local Pre-commit Hook

```bash
#!/bin/bash
# .githooks/pre-commit

cargo build --release 2>/dev/null || exit 1

for wasm in $(find . -name "*.wasm"); do
    ./target/release/stellar-scout "$wasm" > /tmp/analysis.txt
    if grep -q "HIGH" /tmp/analysis.txt; then
        echo "⚠️  High gas risk in $wasm"
        exit 1
    fi
done
```

## Troubleshooting

### "Invalid WASM module"
- Verify file is valid WASM binary
- Check file wasn't corrupted during transfer
- Try re-compiling contract

### "Unexpected EOF"
- File may be incomplete
- Ensure full contract binary was provided
- Check file size is reasonable

### Large analysis time
- Normal for very large contracts (> 1MB)
- Consider splitting contract into modules
- Verify system has adequate RAM
