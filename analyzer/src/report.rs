use crate::complexity::ComplexityAnalysis;
use crate::metrics::ModuleMetrics;
use crate::performance::PerformanceAnalysis;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub metrics: ModuleMetrics,
    pub complexity: ComplexityAnalysis,
    pub performance: PerformanceAnalysis,
}

#[derive(Debug, Clone, Copy)]
pub enum ReportFormat {
    Json,
    Human,
}

impl Report {
    pub fn format(&self, format: ReportFormat) -> Result<String> {
        match format {
            ReportFormat::Json => self.to_json(),
            ReportFormat::Human => self.to_human_readable(),
        }
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    fn to_human_readable(&self) -> Result<String> {
        let mut output = String::new();
        output.push_str("╔══════════════════════════════════════════════╗\n");
        output.push_str("║         STELLAR SCOUT WASM ANALYSIS          ║\n");
        output.push_str("╚══════════════════════════════════════════════╝\n\n");

        output.push_str("📊 MODULE METRICS\n");
        output.push_str("─────────────────────────────────────\n");
        output.push_str(&format!(
            "Total Functions:        {}\n",
            self.metrics.total_functions
        ));
        output.push_str(&format!(
            "Imported Functions:     {}\n",
            self.metrics.imported_functions
        ));
        output.push_str(&format!(
            "Exported Functions:     {}\n",
            self.metrics.exported_functions
        ));
        output.push_str(&format!(
            "Code Section Size:      {} bytes\n",
            self.metrics.code_section_size
        ));
        output.push_str(&format!(
            "Total Module Size:      {} bytes\n",
            self.metrics.total_module_size
        ));
        output.push_str(&format!(
            "Total Imports:          {}\n",
            self.metrics.imports_count
        ));
        output.push_str(&format!(
            "Total Exports:          {}\n",
            self.metrics.exports_count
        ));
        if let Some(pages) = self.metrics.memory_pages {
            output.push_str(&format!("Memory Pages:           {} (64KB each)\n", pages));
        }
        output.push_str(&format!(
            "Table Count:            {}\n\n",
            self.metrics.table_count
        ));

        output.push_str("🔍 COMPLEXITY ANALYSIS\n");
        output.push_str("─────────────────────────────────────\n");
        output.push_str(&format!(
            "Functions Analyzed:     {}\n",
            self.complexity.functions_analyzed
        ));
        output.push_str(&format!(
            "Average Complexity:     {:.2}\n",
            self.complexity.average_complexity
        ));
        output.push_str(&format!(
            "High Complexity Count:  {}\n",
            self.complexity.high_complexity_functions.len()
        ));

        if !self.complexity.high_complexity_functions.is_empty() {
            output.push_str("\nHigh Complexity Functions:\n");
            for func in &self.complexity.high_complexity_functions {
                output.push_str(&format!(
                    "  • Func {} - CC: {}, Max Nesting: {}\n",
                    func.function_index, func.cyclomatic_complexity, func.max_nesting_depth
                ));
            }
        }
        output.push('\n');

        output.push_str("⚡ PERFORMANCE ANALYSIS\n");
        output.push_str("─────────────────────────────────────\n");
        output.push_str(&format!(
            "Memory Operations:      {}\n",
            self.performance.memory_operations_count
        ));
        output.push_str(&format!(
            "Function Calls:         {}\n",
            self.performance.call_count
        ));
        output.push_str(&format!(
            "Gas Risk Level:         {}\n",
            self.performance.gas_risk_level
        ));

        if !self.performance.expensive_operations.is_empty() {
            output.push_str("\nExpensive Operations:\n");
            for op in &self.performance.expensive_operations {
                output.push_str(&format!(
                    "  • {} - Count: {}, Est. Gas: {}\n",
                    op.operation, op.count, op.estimated_gas_cost
                ));
            }
        }
        output.push('\n');

        output.push_str("✅ Analysis complete!\n");

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::complexity::{ComplexityAnalysis, FunctionComplexity};
    use crate::metrics::ModuleMetrics;
    use crate::performance::{ExpensiveOp, PerformanceAnalysis};

    fn create_sample_report() -> Report {
        Report {
            metrics: ModuleMetrics {
                total_functions: 10,
                imported_functions: 2,
                exported_functions: 3,
                code_section_size: 2048,
                total_module_size: 4096,
                imports_count: 2,
                exports_count: 3,
                memory_pages: Some(1),
                table_count: 1,
            },
            complexity: ComplexityAnalysis {
                functions_analyzed: 10,
                high_complexity_functions: vec![FunctionComplexity {
                    function_index: 5,
                    cyclomatic_complexity: 12,
                    max_nesting_depth: 4,
                }],
                average_complexity: 5.5,
            },
            performance: PerformanceAnalysis {
                expensive_operations: vec![ExpensiveOp {
                    operation: "memory.grow".to_string(),
                    count: 2,
                    estimated_gas_cost: 6000,
                }],
                memory_operations_count: 5,
                call_count: 15,
                gas_risk_level: "LOW".to_string(),
            },
        }
    }

    #[test]
    fn test_report_creation() {
        let report = create_sample_report();
        assert_eq!(report.metrics.total_functions, 10);
        assert_eq!(report.complexity.functions_analyzed, 10);
        assert_eq!(report.performance.memory_operations_count, 5);
    }

    #[test]
    fn test_report_to_json() {
        let report = create_sample_report();
        let json = report
            .format(ReportFormat::Json)
            .expect("JSON formatting should succeed");
        assert!(json.contains("total_functions"));
        assert!(json.contains("10"));
        assert!(json.contains("functions_analyzed"));
        assert!(json.contains("gas_risk_level"));
    }

    #[test]
    fn test_report_to_human_readable() {
        let report = create_sample_report();
        let human = report
            .format(ReportFormat::Human)
            .expect("Human formatting should succeed");
        assert!(human.contains("STELLAR SCOUT WASM ANALYSIS"));
        assert!(human.contains("MODULE METRICS"));
        assert!(human.contains("COMPLEXITY ANALYSIS"));
        assert!(human.contains("PERFORMANCE ANALYSIS"));
        assert!(human.contains("10"));
    }

    #[test]
    fn test_report_serialization() {
        let report = create_sample_report();
        let json_str = serde_json::to_string(&report).expect("serialization should succeed");
        let deserialized: Report =
            serde_json::from_str(&json_str).expect("deserialization should succeed");
        assert_eq!(
            deserialized.metrics.total_functions,
            report.metrics.total_functions
        );
        assert_eq!(
            deserialized.complexity.functions_analyzed,
            report.complexity.functions_analyzed
        );
    }
}
