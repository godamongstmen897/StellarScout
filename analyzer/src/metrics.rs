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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_initialization() {
        let metrics = ModuleMetrics {
            total_functions: 5,
            imported_functions: 2,
            exported_functions: 1,
            code_section_size: 1024,
            total_module_size: 2048,
            imports_count: 3,
            exports_count: 1,
            memory_pages: Some(1),
            table_count: 0,
        };

        assert_eq!(metrics.total_functions, 5);
        assert_eq!(metrics.imported_functions, 2);
        assert_eq!(metrics.exported_functions, 1);
        assert_eq!(metrics.code_section_size, 1024);
        assert_eq!(metrics.memory_pages, Some(1));
    }

    #[test]
    fn test_metrics_serialization() {
        let metrics = ModuleMetrics {
            total_functions: 3,
            imported_functions: 1,
            exported_functions: 2,
            code_section_size: 512,
            total_module_size: 1024,
            imports_count: 1,
            exports_count: 2,
            memory_pages: Some(2),
            table_count: 1,
        };

        let json = serde_json::to_string(&metrics).expect("serialization should succeed");
        assert!(json.contains("\"total_functions\":3"));
        assert!(json.contains("\"memory_pages\":2"));

        let deserialized: ModuleMetrics =
            serde_json::from_str(&json).expect("deserialization should succeed");
        assert_eq!(deserialized.total_functions, metrics.total_functions);
        assert_eq!(deserialized.memory_pages, metrics.memory_pages);
    }
}
