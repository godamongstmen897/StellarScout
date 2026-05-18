use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasmparser::{Operator, Payload};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub expensive_operations: Vec<ExpensiveOp>,
    pub memory_operations_count: usize,
    pub call_count: usize,
    pub gas_risk_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpensiveOp {
    pub operation: String,
    pub count: usize,
    pub estimated_gas_cost: usize,
}

impl PerformanceAnalysis {
    pub fn from_payloads(payloads: &[Payload]) -> Result<Self> {
        let mut expensive_ops = std::collections::HashMap::new();
        let mut memory_ops = 0;
        let mut call_count = 0;

        for payload in payloads {
            if let Payload::CodeSectionEntry(body) = payload {
                let reader = body.get_operators_reader()?;
                for op in reader {
                    let op = op?;
                    match op {
                        Operator::MemoryGrow { .. } => {
                            *expensive_ops.entry("memory.grow").or_insert(0) += 1;
                            memory_ops += 1;
                        }
                        Operator::TableGrow { .. } => {
                            *expensive_ops.entry("table.grow").or_insert(0) += 1;
                        }
                        Operator::Call { .. } | Operator::CallIndirect { .. } => {
                            call_count += 1;
                        }
                        Operator::MemoryCopy { .. } => {
                            *expensive_ops.entry("memory.copy").or_insert(0) += 1;
                            memory_ops += 1;
                        }
                        Operator::MemoryFill { .. } => {
                            *expensive_ops.entry("memory.fill").or_insert(0) += 1;
                            memory_ops += 1;
                        }
                        _ => {}
                    }
                }
            }
        }

        let mut ops = Vec::new();
        for (op_name, count) in expensive_ops.iter() {
            let estimated_gas = match *op_name {
                "memory.grow" => 3000 * count,
                "table.grow" => 3000 * count,
                "memory.copy" => *count,
                "memory.fill" => *count,
                _ => 100 * count,
            };
            ops.push(ExpensiveOp {
                operation: op_name.to_string(),
                count: *count,
                estimated_gas_cost: estimated_gas,
            });
        }

        ops.sort_by_key(|o| std::cmp::Reverse(o.estimated_gas_cost));

        let total_gas_risk: usize = ops.iter().map(|o| o.estimated_gas_cost).sum();
        let gas_risk_level = if total_gas_risk > 50000 {
            "HIGH".to_string()
        } else if total_gas_risk > 10000 {
            "MEDIUM".to_string()
        } else {
            "LOW".to_string()
        };

        Ok(PerformanceAnalysis {
            expensive_operations: ops,
            memory_operations_count: memory_ops,
            call_count,
            gas_risk_level,
        })
    }
}
