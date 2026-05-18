use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasmparser::Payload;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl ModuleMetrics {
    pub fn from_payloads(payloads: &[Payload]) -> Result<Self> {
        let mut metrics = ModuleMetrics {
            total_functions: 0,
            imported_functions: 0,
            exported_functions: 0,
            code_section_size: 0,
            total_module_size: 0,
            imports_count: 0,
            exports_count: 0,
            memory_pages: None,
            table_count: 0,
        };

        for payload in payloads {
            match payload {
                Payload::ImportSection(reader) => {
                    for import in reader.clone() {
                        let import = import?;
                        metrics.imports_count += 1;
                        if matches!(import.ty, wasmparser::TypeRef::Func(_)) {
                            metrics.imported_functions += 1;
                        }
                    }
                }
                Payload::CodeSectionEntry(_) => {
                    metrics.total_functions += 1;
                }
                Payload::ExportSection(reader) => {
                    for export in reader.clone() {
                        let export = export?;
                        metrics.exports_count += 1;
                        if matches!(export.kind, wasmparser::ExternalKind::Func) {
                            metrics.exported_functions += 1;
                        }
                    }
                }
                Payload::MemorySection(reader) => {
                    for mem in reader.clone() {
                        let mem = mem?;
                        metrics.memory_pages = Some(mem.initial as u32);
                    }
                }
                Payload::TableSection(reader) => {
                    for _ in reader.clone() {
                        metrics.table_count += 1;
                    }
                }
                _ => {}
            }
        }

        // Calculate code section size
        for payload in payloads {
            if let Payload::CodeSectionEntry(body) = payload {
                metrics.code_section_size += body.range().len();
            }
        }

        Ok(metrics)
    }
}
