use crate::complexity::ComplexityAnalysis;
use crate::metrics::ModuleMetrics;
use crate::performance::PerformanceAnalysis;
use crate::report::Report;
use anyhow::Result;
use wasmparser::Parser as WpParser;

pub struct WasmAnalyzer {
    data: Vec<u8>,
}

impl WasmAnalyzer {
    pub fn new(data: &[u8]) -> Result<Self> {
        Ok(WasmAnalyzer {
            data: data.to_vec(),
        })
    }

    pub fn analyze(&self) -> Result<Report> {
        let parser = WpParser::new(0);
        let payloads: Result<Vec<_>, _> = parser.parse_all(&self.data).collect();
        let payloads = payloads?;

        let metrics = ModuleMetrics::from_payloads(&payloads)?;
        let complexity = ComplexityAnalysis::from_payloads(&payloads)?;
        let performance = PerformanceAnalysis::from_payloads(&payloads)?;

        Ok(Report {
            metrics,
            complexity,
            performance,
        })
    }
}
