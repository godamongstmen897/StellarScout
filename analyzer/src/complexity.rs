use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasmparser::{Operator, Payload};

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
            complexities
                .iter()
                .map(|f| f.cyclomatic_complexity)
                .sum::<usize>() as f64
                / complexities.len() as f64
        } else {
            0.0
        };

        let high_complexity = complexities
            .into_iter()
            .filter(|f| f.cyclomatic_complexity > 10)
            .collect();

        Ok(ComplexityAnalysis {
            functions_analyzed: function_index,
            high_complexity_functions: high_complexity,
            average_complexity: avg_complexity,
        })
    }
}

fn analyze_function_body(reader: wasmparser::OperatorsReader) -> Result<(usize, usize)> {
    let mut cyclomatic_complexity = 1;
    let mut current_nesting: usize = 0;
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
                current_nesting = current_nesting.saturating_sub(1);
            }
            _ => {}
        }
    }

    Ok((cyclomatic_complexity, max_nesting))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complexity_analysis_creation() {
        let analysis = ComplexityAnalysis {
            functions_analyzed: 10,
            high_complexity_functions: vec![],
            average_complexity: 3.5,
        };

        assert_eq!(analysis.functions_analyzed, 10);
        assert_eq!(analysis.high_complexity_functions.len(), 0);
        assert_eq!(analysis.average_complexity, 3.5);
    }

    #[test]
    fn test_function_complexity_creation() {
        let func = FunctionComplexity {
            function_index: 0,
            cyclomatic_complexity: 8,
            max_nesting_depth: 4,
        };

        assert_eq!(func.function_index, 0);
        assert_eq!(func.cyclomatic_complexity, 8);
        assert_eq!(func.max_nesting_depth, 4);
    }

    #[test]
    fn test_complexity_serialization() {
        let analysis = ComplexityAnalysis {
            functions_analyzed: 5,
            high_complexity_functions: vec![FunctionComplexity {
                function_index: 0,
                cyclomatic_complexity: 12,
                max_nesting_depth: 3,
            }],
            average_complexity: 7.2,
        };

        let json = serde_json::to_string(&analysis).expect("serialization should succeed");
        assert!(json.contains("\"functions_analyzed\":5"));
        assert!(json.contains("\"cyclomatic_complexity\":12"));

        let deserialized: ComplexityAnalysis =
            serde_json::from_str(&json).expect("deserialization should succeed");
        assert_eq!(deserialized.functions_analyzed, analysis.functions_analyzed);
        assert_eq!(
            deserialized.high_complexity_functions.len(),
            analysis.high_complexity_functions.len()
        );
    }
}
