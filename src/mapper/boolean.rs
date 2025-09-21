//! BooleanMapper implementation for transforming boolean data

use crate::judgment::NeutrosophicJudgment;
use crate::mapper::types::{
    create_judgment, create_timestamp, normalize_boolean_input, validate_judgment_values,
    BaseMapperParams, BooleanParams, JudgmentData, Mapper, MapperType, ProvenanceEntry,
    ValidationError,
};
use std::collections::HashMap;

/// BooleanMapper for transforming boolean data into Neutrosophic Judgments
pub struct BooleanMapper {
    params: BooleanParams,
}

impl BooleanMapper {
    /// Create a new BooleanMapper with the given parameters
    pub fn new(params: BooleanParams) -> Result<Self, ValidationError> {
        let mapper = Self { params };
        mapper.validate_parameters()?;
        Ok(mapper)
    }

    /// Validate the mapper parameters
    fn validate_parameters(&self) -> Result<(), ValidationError> {
        // Validate true_map
        validate_judgment_values(
            self.params.true_map.T,
            self.params.true_map.I,
            self.params.true_map.F,
        )
        .map_err(|e| ValidationError::InvalidJudgment {
            message: format!("Invalid true_map: {}", e),
        })?;

        // Validate false_map
        validate_judgment_values(
            self.params.false_map.T,
            self.params.false_map.I,
            self.params.false_map.F,
        )
        .map_err(|e| ValidationError::InvalidJudgment {
            message: format!("Invalid false_map: {}", e),
        })?;

        Ok(())
    }

    /// Create provenance entry for the transformation
    fn create_provenance_entry(
        &self,
        input_value: &dyn std::any::Any,
        normalized: bool,
    ) -> ProvenanceEntry {
        let mut metadata = HashMap::new();
        metadata.insert(
            "mapper_type".to_string(),
            serde_json::Value::String("boolean".to_string()),
        );
        metadata.insert(
            "normalized".to_string(),
            serde_json::Value::String(normalized.to_string()),
        );

        // Add input type information
        metadata.insert(
            "input_type".to_string(),
            serde_json::Value::String(std::any::type_name_of_val(input_value).to_string()),
        );

        // Add original input value if possible
        if let Some(val) = input_value.downcast_ref::<bool>() {
            metadata.insert("original_input".to_string(), serde_json::Value::Bool(*val));
        } else if let Some(val) = input_value.downcast_ref::<i32>() {
            metadata.insert(
                "original_input".to_string(),
                serde_json::Value::Number(serde_json::Number::from(*val)),
            );
        } else if let Some(val) = input_value.downcast_ref::<String>() {
            metadata.insert(
                "original_input".to_string(),
                serde_json::Value::String(val.clone()),
            );
        }

        ProvenanceEntry {
            source_id: self.params.base.id.clone(),
            timestamp: create_timestamp(),
            description: Some(format!("Boolean mapping of value")),
            metadata: Some(metadata),
        }
    }

    /// Apply the mapper to a boolean input
    pub fn apply(&self, input_value: &dyn std::any::Any) -> crate::Result<NeutrosophicJudgment> {
        let normalized_input = normalize_boolean_input(input_value).map_err(|e| {
            crate::error::OpenTrustError::InvalidFusionInput {
                message: e.to_string(),
            }
        })?;

        let judgment_data = if normalized_input {
            &self.params.true_map
        } else {
            &self.params.false_map
        };

        let provenance_entry = self.create_provenance_entry(input_value, normalized_input);

        create_judgment(
            judgment_data.T,
            judgment_data.I,
            judgment_data.F,
            vec![provenance_entry],
        )
    }
}

impl Mapper for BooleanMapper {
    fn apply(&self, input: &dyn std::any::Any) -> crate::Result<NeutrosophicJudgment> {
        self.apply(input)
    }

    fn get_params(&self) -> &dyn std::any::Any {
        &self.params
    }

    fn get_type(&self) -> MapperType {
        MapperType::Boolean
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
    fn test_boolean_mapper_creation() {
        let params = BooleanParams {
            base: BaseMapperParams {
                id: "test-boolean".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Boolean,
                description: None,
                metadata: None,
            },
            true_map: JudgmentData {
                T: 0.9,
                I: 0.1,
                F: 0.0,
            },
            false_map: JudgmentData {
                T: 0.0,
                I: 0.1,
                F: 0.9,
            },
        };

        let mapper = BooleanMapper::new(params).unwrap();
        assert_eq!(mapper.get_type(), MapperType::Boolean);
    }

    #[test]
    fn test_boolean_mapper_validation() {
        let params = BooleanParams {
            base: BaseMapperParams {
                id: "test-boolean".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Boolean,
                description: None,
                metadata: None,
            },
            true_map: JudgmentData {
                T: 1.5, // Invalid - should fail
                I: 0.1,
                F: 0.0,
            },
            false_map: JudgmentData {
                T: 0.0,
                I: 0.1,
                F: 0.9,
            },
        };

        let result = BooleanMapper::new(params);
        assert!(result.is_err());
    }

    #[test]
    fn test_boolean_mapper_apply_bool() {
        let params = BooleanParams {
            base: BaseMapperParams {
                id: "test-boolean".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Boolean,
                description: None,
                metadata: None,
            },
            true_map: JudgmentData {
                T: 0.9,
                I: 0.1,
                F: 0.0,
            },
            false_map: JudgmentData {
                T: 0.0,
                I: 0.1,
                F: 0.9,
            },
        };

        let mapper = BooleanMapper::new(params).unwrap();

        // Test true value
        let judgment1 = mapper.apply(&true).unwrap();
        assert!(judgment1.is_valid());
        assert_eq!(judgment1.t, 0.9);
        assert_eq!(judgment1.i, 0.1);
        assert_eq!(judgment1.f, 0.0);

        // Test false value
        let judgment2 = mapper.apply(&false).unwrap();
        assert!(judgment2.is_valid());
        assert_eq!(judgment2.t, 0.0);
        assert_eq!(judgment2.i, 0.1);
        assert_eq!(judgment2.f, 0.9);
    }

    #[test]
    fn test_boolean_mapper_apply_int() {
        let params = BooleanParams {
            base: BaseMapperParams {
                id: "test-boolean".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Boolean,
                description: None,
                metadata: None,
            },
            true_map: JudgmentData {
                T: 0.9,
                I: 0.1,
                F: 0.0,
            },
            false_map: JudgmentData {
                T: 0.0,
                I: 0.1,
                F: 0.9,
            },
        };

        let mapper = BooleanMapper::new(params).unwrap();

        // Test integer 1 (true)
        let judgment1 = mapper.apply(&1i32).unwrap();
        assert!(judgment1.is_valid());
        assert_eq!(judgment1.t, 0.9);

        // Test integer 0 (false)
        let judgment2 = mapper.apply(&0i32).unwrap();
        assert!(judgment2.is_valid());
        assert_eq!(judgment2.f, 0.9);
    }

    #[test]
    fn test_boolean_mapper_apply_string() {
        let params = BooleanParams {
            base: BaseMapperParams {
                id: "test-boolean".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Boolean,
                description: None,
                metadata: None,
            },
            true_map: JudgmentData {
                T: 0.9,
                I: 0.1,
                F: 0.0,
            },
            false_map: JudgmentData {
                T: 0.0,
                I: 0.1,
                F: 0.9,
            },
        };

        let mapper = BooleanMapper::new(params).unwrap();

        // Test string "true"
        let judgment1 = mapper.apply(&"true".to_string()).unwrap();
        assert!(judgment1.is_valid());
        assert_eq!(judgment1.t, 0.9);

        // Test string "false"
        let judgment2 = mapper.apply(&"false".to_string()).unwrap();
        assert!(judgment2.is_valid());
        assert_eq!(judgment2.f, 0.9);
    }

    #[test]
    fn test_boolean_mapper_invalid_input() {
        let params = BooleanParams {
            base: BaseMapperParams {
                id: "test-boolean".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Boolean,
                description: None,
                metadata: None,
            },
            true_map: JudgmentData {
                T: 0.9,
                I: 0.1,
                F: 0.0,
            },
            false_map: JudgmentData {
                T: 0.0,
                I: 0.1,
                F: 0.9,
            },
        };

        let mapper = BooleanMapper::new(params).unwrap();

        // Test invalid input
        let result = mapper.apply(&"invalid".to_string());
        assert!(result.is_err());
    }
}
