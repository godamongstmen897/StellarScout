use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasmparser::{Payload, Operator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityAnalysis {
    pub functions_analyzed: usize,
    pub high_complexity_functions: Vec<FunctionComplexity>,
    pub average_complexity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    pub function_index: usize,
    pub cyclomatic_complexity: usize,
    pub max_nesting_depth: usize,
}

impl ComplexityAnalysis {
    pub fn from_payloads(payloads: &[Payload]) -> Result<Self> {
        let mut complexities = Vec::new();
        let mut function_index = 0;

        for payload in payloads {
            if let Payload::CodeSectionEntry(body) = payload {
                let reader = body.get_operators_reader()?;
                let (cyclomatic, nesting) = analyze_function_body(reader)?;
                
                if cyclomatic > 5 || nesting > 5 {
                    complexities.push(FunctionComplexity {
                        function_index,
                        cyclomatic_complexity: cyclomatic,
                        max_nesting_depth: nesting,
                    });
                }
                function_index += 1;
            }
        }

        let avg_complexity = if !complexities.is_empty() {
            complexities.iter().map(|f| f.cyclomatic_complexity).sum::<usize>() as f64
                / complexities.len() as f64
        } else {
            0.0
        };

        let high_complexity = complexities.into_iter().filter(|f| f.cyclomatic_complexity > 10).collect();

        Ok(ComplexityAnalysis {
            functions_analyzed: function_index,
            high_complexity_functions: high_complexity,
            average_complexity: avg_complexity,
        })
    }
}

fn analyze_function_body(
    reader: wasmparser::OperatorsReader,
) -> Result<(usize, usize)> {
    let mut cyclomatic_complexity = 1;
    let mut current_nesting = 0;
    let mut max_nesting = 0;

    for op in reader {
        let op = op?;
        match op {
            Operator::If { .. } | Operator::Loop { .. } => {
                cyclomatic_complexity += 1;
                current_nesting += 1;
                max_nesting = max_nesting.max(current_nesting);
            }
            Operator::Block { .. } => {
                current_nesting += 1;
                max_nesting = max_nesting.max(current_nesting);
            }
            Operator::End => {
                if current_nesting > 0 {
                    current_nesting -= 1;
                }
            }
            _ => {}
        }
    }

    Ok((cyclomatic_complexity, max_nesting))
}
