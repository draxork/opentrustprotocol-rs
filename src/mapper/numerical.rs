//! NumericalMapper implementation for transforming continuous numerical data

use crate::judgment::NeutrosophicJudgment;
use crate::mapper::types::{
    create_judgment, create_timestamp, Mapper, MapperType, NumericalParams,
    ProvenanceEntry, ValidationError,
};
#[cfg(test)]
use crate::mapper::types::BaseMapperParams;
use std::collections::HashMap;

/// NumericalMapper for transforming continuous numerical data into Neutrosophic Judgments
pub struct NumericalMapper {
    params: NumericalParams,
}

impl NumericalMapper {
    /// Create a new NumericalMapper with the given parameters
    pub fn new(params: NumericalParams) -> Result<Self, ValidationError> {
        let mapper = Self { params };
        mapper.validate_parameters()?;
        Ok(mapper)
    }

    /// Validate the mapper parameters
    fn validate_parameters(&self) -> Result<(), ValidationError> {
        let points = [
            self.params.falsity_point,
            self.params.indeterminacy_point,
            self.params.truth_point,
        ];

        // Check if all points are distinct (using epsilon comparison for f64)
        let epsilon = 1e-10;
        let mut distinct_count = 0;
        for i in 0..3 {
            let mut is_unique = true;
            for j in (i + 1)..3 {
                if (points[i] - points[j]).abs() < epsilon {
                    is_unique = false;
                    break;
                }
            }
            if is_unique {
                distinct_count += 1;
            }
        }

        if distinct_count < 3 {
            return Err(ValidationError::InvalidJudgment {
                message: "falsity_point, indeterminacy_point, and truth_point must be distinct for NumericalMapper".to_string(),
            });
        }

        Ok(())
    }

    /// Calculate interpolation for the given input value
    fn calculate_interpolation(&self, input_value: f64) -> (f64, f64, f64) {
        #[allow(non_snake_case)]
        let mut T = 0.0;
        #[allow(non_snake_case)]
        let mut I = 0.0;
        #[allow(non_snake_case)]
        let mut F = 0.0;

        let falsity_point = self.params.falsity_point;
        let indeterminacy_point = self.params.indeterminacy_point;
        let truth_point = self.params.truth_point;

        // Sort points for easier calculation
        let mut points = [
            (falsity_point, 'F'),
            (indeterminacy_point, 'I'),
            (truth_point, 'T'),
        ];
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let (min_point, min_type) = points[0];
        let (max_point, max_type) = points[2];

        if input_value <= min_point {
            // Assign full value to the minimum point type
            match min_type {
                'F' => F = 1.0,
                'I' => I = 1.0,
                'T' => T = 1.0,
                _ => {}
            }
        } else if input_value >= max_point {
            // Assign full value to the maximum point type
            match max_type {
                'T' => T = 1.0,
                'I' => I = 1.0,
                'F' => F = 1.0,
                _ => {}
            }
        } else {
            // Interpolate between points
            if (falsity_point <= indeterminacy_point && indeterminacy_point <= truth_point)
                || (falsity_point >= indeterminacy_point && indeterminacy_point >= truth_point)
            {
                // Linear interpolation case
                if input_value >= falsity_point.min(indeterminacy_point)
                    && input_value <= falsity_point.max(indeterminacy_point)
                    && falsity_point != indeterminacy_point
                {
                    let ratio = (input_value - falsity_point).abs()
                        / (indeterminacy_point - falsity_point).abs();
                    I = ratio;
                    F = 1.0 - ratio;
                } else if input_value >= indeterminacy_point.min(truth_point)
                    && input_value <= indeterminacy_point.max(truth_point)
                    && indeterminacy_point != truth_point
                {
                    let ratio = (input_value - indeterminacy_point).abs()
                        / (truth_point - indeterminacy_point).abs();
                    T = ratio;
                    I = 1.0 - ratio;
                } else if (falsity_point == indeterminacy_point && input_value == falsity_point)
                    || (indeterminacy_point == truth_point && input_value == indeterminacy_point)
                    || (falsity_point == truth_point && input_value == falsity_point)
                {
                    I = 1.0;
                }
            } else {
                // Non-linear interpolation case
                if input_value >= falsity_point.min(indeterminacy_point)
                    && input_value <= falsity_point.max(indeterminacy_point)
                    && falsity_point != indeterminacy_point
                {
                    let ratio = (input_value - falsity_point).abs()
                        / (indeterminacy_point - falsity_point).abs();
                    I = ratio;
                    F = 1.0 - ratio;
                } else if input_value >= indeterminacy_point.min(truth_point)
                    && input_value <= indeterminacy_point.max(truth_point)
                    && indeterminacy_point != truth_point
                {
                    let ratio = (input_value - indeterminacy_point).abs()
                        / (truth_point - indeterminacy_point).abs();
                    T = ratio;
                    I = 1.0 - ratio;
                } else {
                    // Exact match cases
                    if input_value == falsity_point {
                        F = 1.0;
                    } else if input_value == indeterminacy_point {
                        I = 1.0;
                    } else if input_value == truth_point {
                        T = 1.0;
                    }
                }
            }
        }

        (T, I, F)
    }

    /// Create provenance entry for the transformation
    fn create_provenance_entry(&self, input_value: f64) -> ProvenanceEntry {
        let mut metadata = HashMap::new();
        metadata.insert(
            "mapper_type".to_string(),
            serde_json::Value::String("numerical".to_string()),
        );
        metadata.insert(
            "input_value".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(input_value).unwrap()),
        );
        metadata.insert(
            "falsity_point".to_string(),
            serde_json::Value::Number(
                serde_json::Number::from_f64(self.params.falsity_point).unwrap(),
            ),
        );
        metadata.insert(
            "indeterminacy_point".to_string(),
            serde_json::Value::Number(
                serde_json::Number::from_f64(self.params.indeterminacy_point).unwrap(),
            ),
        );
        metadata.insert(
            "truth_point".to_string(),
            serde_json::Value::Number(
                serde_json::Number::from_f64(self.params.truth_point).unwrap(),
            ),
        );

        ProvenanceEntry {
            source_id: self.params.base.id.clone(),
            timestamp: create_timestamp(),
            description: Some(format!("Numerical mapping of value {}", input_value)),
            metadata: Some(metadata),
        }
    }

    /// Apply the mapper to a numerical input
    pub fn apply(&self, input_value: f64) -> crate::Result<NeutrosophicJudgment> {
        let min_point = self.params.falsity_point.min(self.params.truth_point);
        let max_point = self.params.falsity_point.max(self.params.truth_point);

        let clamped_value = if self.params.clamp_to_range.unwrap_or(true) {
            input_value.max(min_point).min(max_point)
        } else {
            if input_value < min_point || input_value > max_point {
                return Err(crate::error::OpenTrustError::InvalidFusionInput {
                    message: format!(
                        "Input value {} is out of the defined mapper range [{}, {}] and clamp_to_range is false",
                        input_value, min_point, max_point
                    ),
                });
            }
            input_value
        };

        #[allow(non_snake_case)]
        let (T, I, F) = self.calculate_interpolation(clamped_value);
        let provenance_entry = self.create_provenance_entry(input_value);

        create_judgment(T, I, F, vec![provenance_entry])
    }
}

impl Mapper for NumericalMapper {
    fn apply(&self, input: &dyn std::any::Any) -> crate::Result<NeutrosophicJudgment> {
        if let Some(value) = input.downcast_ref::<f64>() {
            self.apply(*value)
        } else if let Some(value) = input.downcast_ref::<i32>() {
            self.apply(*value as f64)
        } else if let Some(value) = input.downcast_ref::<i64>() {
            self.apply(*value as f64)
        } else {
            Err(crate::error::OpenTrustError::InvalidFusionInput {
                message: format!(
                    "Input for NumericalMapper must be a number, got {}",
                    std::any::type_name_of_val(input)
                ),
            })
        }
    }

    fn get_params(&self) -> &dyn std::any::Any {
        &self.params
    }

    fn get_type(&self) -> MapperType {
        MapperType::Numerical
    }

    fn validate(&self) -> crate::Result<()> {
        self.validate_parameters()
            .map_err(|e| crate::error::OpenTrustError::InvalidFusionInput {
                message: e.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numerical_mapper_creation() {
        let params = NumericalParams {
            base: BaseMapperParams {
                id: "test-numerical".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Numerical,
                description: None,
                metadata: None,
            },
            falsity_point: 1.0,
            indeterminacy_point: 1.5,
            truth_point: 3.0,
            clamp_to_range: Some(true),
        };

        let mapper = NumericalMapper::new(params).unwrap();
        assert_eq!(mapper.get_type(), MapperType::Numerical);
    }

    #[test]
    fn test_numerical_mapper_validation() {
        let params = NumericalParams {
            base: BaseMapperParams {
                id: "test-numerical".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Numerical,
                description: None,
                metadata: None,
            },
            falsity_point: 1.0,
            indeterminacy_point: 1.0, // Same as falsity_point - should fail
            truth_point: 3.0,
            clamp_to_range: Some(true),
        };

        let result = NumericalMapper::new(params);
        assert!(result.is_err());
    }

    #[test]
    fn test_numerical_mapper_apply() {
        let params = NumericalParams {
            base: BaseMapperParams {
                id: "test-numerical".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Numerical,
                description: None,
                metadata: None,
            },
            falsity_point: 1.0,
            indeterminacy_point: 1.5,
            truth_point: 3.0,
            clamp_to_range: Some(true),
        };

        let mapper = NumericalMapper::new(params).unwrap();
        let judgment = mapper.apply(2.0).unwrap();

        assert!(judgment.is_valid());
        assert!(judgment.total() <= 1.0);
        assert_eq!(judgment.provenance_chain.len(), 1);
    }

    #[test]
    fn test_numerical_mapper_edge_cases() {
        let params = NumericalParams {
            base: BaseMapperParams {
                id: "test-numerical".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Numerical,
                description: None,
                metadata: None,
            },
            falsity_point: 1.0,
            indeterminacy_point: 1.5,
            truth_point: 3.0,
            clamp_to_range: Some(true),
        };

        let mapper = NumericalMapper::new(params).unwrap();

        // Test edge cases
        let judgment1 = mapper.apply(1.0).unwrap(); // Falsity point
        let judgment2 = mapper.apply(1.5).unwrap(); // Indeterminacy point
        let judgment3 = mapper.apply(3.0).unwrap(); // Truth point

        assert!(judgment1.is_valid());
        assert!(judgment2.is_valid());
        assert!(judgment3.is_valid());
    }
}
