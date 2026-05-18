pub mod analysis;
pub mod metrics;
pub mod complexity;
pub mod performance;
pub mod report;

pub use analysis::WasmAnalyzer;
pub use report::{Report, ReportFormat};

use anyhow::Result;
use std::path::Path;

pub fn analyze_file(path: &Path) -> Result<Report> {
    let data = std::fs::read(path)?;
    let analyzer = WasmAnalyzer::new(&data)?;
    analyzer.analyze()
}
