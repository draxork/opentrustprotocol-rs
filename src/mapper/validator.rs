//! MapperValidator implementation for JSON Schema validation

use crate::mapper::types::MapperParams;
#[cfg(test)]
use crate::mapper::types::MapperType;
use serde_json::Value;

/// Validation result for mapper configurations
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the configuration is valid
    pub valid: bool,
    /// List of validation errors
    pub errors: Vec<String>,
}

/// MapperValidator for validating mapper configurations against JSON Schema
pub struct MapperValidator {
    // JSON Schema definitions for each mapper type (currently unused but kept for future schema validation)
    #[allow(dead_code)]
    numerical_schema: Value,
    #[allow(dead_code)]
    categorical_schema: Value,
    #[allow(dead_code)]
    boolean_schema: Value,
}

impl MapperValidator {
    /// Create a new MapperValidator
    pub fn new() -> Self {
        Self {
            numerical_schema: Self::create_numerical_schema(),
            categorical_schema: Self::create_categorical_schema(),
            boolean_schema: Self::create_boolean_schema(),
        }
    }

    /// Create JSON Schema for NumericalMapper
    fn create_numerical_schema() -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "version": {"type": "string"},
                "mapper_type": {"const": "numerical"},
                "falsity_point": {"type": "number"},
                "indeterminacy_point": {"type": "number"},
                "truth_point": {"type": "number"},
                "clamp_to_range": {"type": "boolean"}
            },
            "required": ["id", "version", "mapper_type", "falsity_point", "indeterminacy_point", "truth_point"]
        })
    }

    /// Create JSON Schema for CategoricalMapper
    fn create_categorical_schema() -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "version": {"type": "string"},
                "mapper_type": {"const": "categorical"},
                "mappings": {
                    "type": "object",
                    "additionalProperties": {
                        "type": "object",
                        "properties": {
                            "T": {"type": "number", "minimum": 0.0, "maximum": 1.0},
                            "I": {"type": "number", "minimum": 0.0, "maximum": 1.0},
                            "F": {"type": "number", "minimum": 0.0, "maximum": 1.0}
                        },
                        "required": ["T", "I", "F"]
                    }
                },
                "default_judgment": {
                    "type": "object",
                    "properties": {
                        "T": {"type": "number", "minimum": 0.0, "maximum": 1.0},
                        "I": {"type": "number", "minimum": 0.0, "maximum": 1.0},
                        "F": {"type": "number", "minimum": 0.0, "maximum": 1.0}
                    },
                    "required": ["T", "I", "F"]
                }
            },
            "required": ["id", "version", "mapper_type", "mappings"]
        })
    }

    /// Create JSON Schema for BooleanMapper
    fn create_boolean_schema() -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "version": {"type": "string"},
                "mapper_type": {"const": "boolean"},
                "true_map": {
                    "type": "object",
                    "properties": {
                        "T": {"type": "number", "minimum": 0.0, "maximum": 1.0},
                        "I": {"type": "number", "minimum": 0.0, "maximum": 1.0},
                        "F": {"type": "number", "minimum": 0.0, "maximum": 1.0}
                    },
                    "required": ["T", "I", "F"]
                },
                "false_map": {
                    "type": "object",
                    "properties": {
                        "T": {"type": "number", "minimum": 0.0, "maximum": 1.0},
                        "I": {"type": "number", "minimum": 0.0, "maximum": 1.0},
                        "F": {"type": "number", "minimum": 0.0, "maximum": 1.0}
                    },
                    "required": ["T", "I", "F"]
                }
            },
            "required": ["id", "version", "mapper_type", "true_map", "false_map"]
        })
    }

    /// Validate a mapper configuration
    pub fn validate(&self, config: &MapperParams) -> ValidationResult {
        let mut errors = Vec::new();

        match config {
            MapperParams::Numerical(params) => {
                self.validate_numerical(params, &mut errors);
            }
            MapperParams::Categorical(params) => {
                self.validate_categorical(params, &mut errors);
            }
            MapperParams::Boolean(params) => {
                self.validate_boolean(params, &mut errors);
            }
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
        }
    }

    /// Validate NumericalMapper parameters
    fn validate_numerical(
        &self,
        params: &crate::mapper::types::NumericalParams,
        errors: &mut Vec<String>,
    ) {
        // Check if points are distinct (using epsilon comparison for f64)
        let points = [
            params.falsity_point,
            params.indeterminacy_point,
            params.truth_point,
        ];
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
            errors.push(
                "falsity_point, indeterminacy_point, and truth_point must be distinct".to_string(),
            );
        }

        // Check if ID is not empty
        if params.base.id.is_empty() {
            errors.push("id cannot be empty".to_string());
        }

        // Check if version is not empty
        if params.base.version.is_empty() {
            errors.push("version cannot be empty".to_string());
        }
    }

    /// Validate CategoricalMapper parameters
    fn validate_categorical(
        &self,
        params: &crate::mapper::types::CategoricalParams,
        errors: &mut Vec<String>,
    ) {
        // Check if mappings exist
        if params.mappings.is_empty() {
            errors.push("mappings cannot be empty".to_string());
        }

        // Validate each mapping
        for (category, judgment_data) in &params.mappings {
            if let Err(e) = crate::mapper::types::validate_judgment_values(
                judgment_data.T,
                judgment_data.I,
                judgment_data.F,
            ) {
                errors.push(format!(
                    "Invalid judgment for category '{}': {}",
                    category, e
                ));
            }
        }

        // Validate default judgment if present
        if let Some(ref default_judgment) = params.default_judgment {
            if let Err(e) = crate::mapper::types::validate_judgment_values(
                default_judgment.T,
                default_judgment.I,
                default_judgment.F,
            ) {
                errors.push(format!("Invalid default_judgment: {}", e));
            }
        }

        // Check if ID is not empty
        if params.base.id.is_empty() {
            errors.push("id cannot be empty".to_string());
        }

        // Check if version is not empty
        if params.base.version.is_empty() {
            errors.push("version cannot be empty".to_string());
        }
    }

    /// Validate BooleanMapper parameters
    fn validate_boolean(
        &self,
        params: &crate::mapper::types::BooleanParams,
        errors: &mut Vec<String>,
    ) {
        // Validate true_map
        if let Err(e) = crate::mapper::types::validate_judgment_values(
            params.true_map.T,
            params.true_map.I,
            params.true_map.F,
        ) {
            errors.push(format!("Invalid true_map: {}", e));
        }

        // Validate false_map
        if let Err(e) = crate::mapper::types::validate_judgment_values(
            params.false_map.T,
            params.false_map.I,
            params.false_map.F,
        ) {
            errors.push(format!("Invalid false_map: {}", e));
        }

        // Check if ID is not empty
        if params.base.id.is_empty() {
            errors.push("id cannot be empty".to_string());
        }

        // Check if version is not empty
        if params.base.version.is_empty() {
            errors.push("version cannot be empty".to_string());
        }
    }

    /// Validate multiple configurations
    pub fn validate_multiple(
        &self,
        configs: &[MapperParams],
    ) -> Vec<(MapperParams, ValidationResult)> {
        configs
            .iter()
            .map(|config| (config.clone(), self.validate(config)))
            .collect()
    }

    /// Validate a JSON configuration
    pub fn validate_json(&self, json: &str) -> Result<ValidationResult, serde_json::Error> {
        let config: MapperParams = serde_json::from_str(json)?;
        Ok(self.validate(&config))
    }
}

impl Default for MapperValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapper::types::{BooleanParams, CategoricalParams, JudgmentData, NumericalParams};
    use std::collections::HashMap;

    #[test]
    fn test_validator_creation() {
        let validator = MapperValidator::new();
        assert!(validator.numerical_schema.is_object());
        assert!(validator.categorical_schema.is_object());
        assert!(validator.boolean_schema.is_object());
    }

    #[test]
    fn test_validate_valid_numerical() {
        let validator = MapperValidator::new();

        let params = NumericalParams {
            base: crate::mapper::types::BaseMapperParams {
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

        let result = validator.validate(&MapperParams::Numerical(params));
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_invalid_numerical() {
        let validator = MapperValidator::new();

        let params = NumericalParams {
            base: crate::mapper::types::BaseMapperParams {
                id: "".to_string(), // Empty ID
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Numerical,
                description: None,
                metadata: None,
            },
            falsity_point: 1.0,
            indeterminacy_point: 1.0, // Same as falsity_point
            truth_point: 3.0,
            clamp_to_range: Some(true),
        };

        let result = validator.validate(&MapperParams::Numerical(params));
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_validate_valid_categorical() {
        let validator = MapperValidator::new();

        let mut mappings = HashMap::new();
        mappings.insert(
            "VERIFIED".to_string(),
            JudgmentData {
                T: 0.9,
                I: 0.1,
                F: 0.0,
            },
        );

        let params = CategoricalParams {
            base: crate::mapper::types::BaseMapperParams {
                id: "test-categorical".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Categorical,
                description: None,
                metadata: None,
            },
            mappings,
            default_judgment: None,
        };

        let result = validator.validate(&MapperParams::Categorical(params));
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_valid_boolean() {
        let validator = MapperValidator::new();

        let params = BooleanParams {
            base: crate::mapper::types::BaseMapperParams {
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

        let result = validator.validate(&MapperParams::Boolean(params));
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_multiple() {
        let validator = MapperValidator::new();

        let configs = vec![
            MapperParams::Numerical(NumericalParams {
                base: crate::mapper::types::BaseMapperParams {
                    id: "test1".to_string(),
                    version: "1.0.0".to_string(),
                    mapper_type: MapperType::Numerical,
                    description: None,
                    metadata: None,
                },
                falsity_point: 1.0,
                indeterminacy_point: 1.5,
                truth_point: 3.0,
                clamp_to_range: Some(true),
            }),
            MapperParams::Numerical(NumericalParams {
                base: crate::mapper::types::BaseMapperParams {
                    id: "".to_string(), // Invalid
                    version: "1.0.0".to_string(),
                    mapper_type: MapperType::Numerical,
                    description: None,
                    metadata: None,
                },
                falsity_point: 1.0,
                indeterminacy_point: 1.0, // Invalid
                truth_point: 3.0,
                clamp_to_range: Some(true),
            }),
        ];

        let results = validator.validate_multiple(&configs);
        assert_eq!(results.len(), 2);
        assert!(results[0].1.valid);
        assert!(!results[1].1.valid);
    }
}
