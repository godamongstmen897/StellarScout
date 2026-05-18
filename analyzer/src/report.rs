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
