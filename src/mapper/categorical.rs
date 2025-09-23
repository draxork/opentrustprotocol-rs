//! CategoricalMapper implementation for transforming categorical data

use crate::judgment::NeutrosophicJudgment;
use crate::mapper::types::{
    create_judgment, create_timestamp, validate_judgment_values, CategoricalParams, Mapper,
    MapperType, ProvenanceEntry, ValidationError,
};
#[cfg(test)]
use crate::mapper::types::{BaseMapperParams, JudgmentData};
use std::collections::HashMap;

/// CategoricalMapper for transforming categorical data into Neutrosophic Judgments
pub struct CategoricalMapper {
    params: CategoricalParams,
}

impl CategoricalMapper {
    /// Create a new CategoricalMapper with the given parameters
    pub fn new(params: CategoricalParams) -> Result<Self, ValidationError> {
        let mapper = Self { params };
        mapper.validate_parameters()?;
        Ok(mapper)
    }

    /// Validate the mapper parameters
    fn validate_parameters(&self) -> Result<(), ValidationError> {
        // Check if mappings exist
        if self.params.mappings.is_empty() {
            return Err(ValidationError::InvalidJudgment {
                message: "CategoricalMapper must have at least one mapping defined".to_string(),
            });
        }

        // Validate all mappings
        for (category, judgment_data) in &self.params.mappings {
            validate_judgment_values(judgment_data.T, judgment_data.I, judgment_data.F).map_err(
                |e| ValidationError::InvalidJudgment {
                    message: format!("Invalid judgment for category '{}': {}", category, e),
                },
            )?;
        }

        // Validate default judgment if present
        if let Some(ref default_judgment) = self.params.default_judgment {
            validate_judgment_values(default_judgment.T, default_judgment.I, default_judgment.F)
                .map_err(|e| ValidationError::InvalidJudgment {
                    message: format!("Invalid default_judgment: {}", e),
                })?;
        }

        Ok(())
    }

    /// Create provenance entry for the transformation
    fn create_provenance_entry(&self, input_category: &str) -> ProvenanceEntry {
        let mut metadata = HashMap::new();
        metadata.insert(
            "mapper_type".to_string(),
            serde_json::Value::String("categorical".to_string()),
        );
        metadata.insert(
            "input_category".to_string(),
            serde_json::Value::String(input_category.to_string()),
        );
        metadata.insert(
            "has_mapping".to_string(),
            serde_json::Value::Bool(self.params.mappings.contains_key(input_category)),
        );

        ProvenanceEntry {
            source_id: self.params.base.id.clone(),
            timestamp: create_timestamp(),
            description: Some(format!(
                "Categorical mapping of category '{}'",
                input_category
            )),
            metadata: Some(metadata),
            conformance_seal: None,
        }
    }

    /// Apply the mapper to a categorical input
    pub fn apply(&self, input_category: &str) -> crate::Result<NeutrosophicJudgment> {
        let provenance_entry = self.create_provenance_entry(input_category);

        // Check if category exists in mappings
        if let Some(judgment_data) = self.params.mappings.get(input_category) {
            return create_judgment(
                judgment_data.T,
                judgment_data.I,
                judgment_data.F,
                vec![provenance_entry],
            );
        }

        // Use default judgment if available
        if let Some(ref default_judgment) = self.params.default_judgment {
            return create_judgment(
                default_judgment.T,
                default_judgment.I,
                default_judgment.F,
                vec![provenance_entry],
            );
        }

        // No mapping and no default judgment
        Err(crate::error::OpenTrustError::InvalidFusionInput {
            message: format!(
                "Input category '{}' not found in mapper and no default_judgment is defined",
                input_category
            ),
        })
    }
}

impl Mapper for CategoricalMapper {
    fn apply(&self, input: &dyn std::any::Any) -> crate::Result<NeutrosophicJudgment> {
        if let Some(value) = input.downcast_ref::<String>() {
            self.apply(value)
        } else if let Some(value) = input.downcast_ref::<&str>() {
            self.apply(value)
        } else {
            Err(crate::error::OpenTrustError::InvalidFusionInput {
                message: format!(
                    "Input for CategoricalMapper must be a string, got {}",
                    std::any::type_name_of_val(input)
                ),
            })
        }
    }

    fn get_params(&self) -> &dyn std::any::Any {
        &self.params
    }

    fn get_type(&self) -> MapperType {
        MapperType::Categorical
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
    fn test_categorical_mapper_creation() {
        let mut mappings = HashMap::new();
        mappings.insert(
            "VERIFIED".to_string(),
            JudgmentData {
                T: 0.9,
                I: 0.1,
                F: 0.0,
            },
        );
        mappings.insert(
            "PENDING".to_string(),
            JudgmentData {
                T: 0.0,
                I: 1.0,
                F: 0.0,
            },
        );

        let params = CategoricalParams {
            base: BaseMapperParams {
                id: "test-categorical".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Categorical,
                description: None,
                metadata: None,
            },
            mappings,
            default_judgment: Some(JudgmentData {
                T: 0.0,
                I: 0.5,
                F: 0.5,
            }),
        };

        let mapper = CategoricalMapper::new(params).unwrap();
        assert_eq!(mapper.get_type(), MapperType::Categorical);
    }

    #[test]
    fn test_categorical_mapper_validation() {
        let params = CategoricalParams {
            base: BaseMapperParams {
                id: "test-categorical".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Categorical,
                description: None,
                metadata: None,
            },
            mappings: HashMap::new(), // Empty mappings - should fail
            default_judgment: None,
        };

        let result = CategoricalMapper::new(params);
        assert!(result.is_err());
    }

    #[test]
    fn test_categorical_mapper_apply() {
        let mut mappings = HashMap::new();
        mappings.insert(
            "VERIFIED".to_string(),
            JudgmentData {
                T: 0.9,
                I: 0.1,
                F: 0.0,
            },
        );
        mappings.insert(
            "PENDING".to_string(),
            JudgmentData {
                T: 0.0,
                I: 1.0,
                F: 0.0,
            },
        );

        let params = CategoricalParams {
            base: BaseMapperParams {
                id: "test-categorical".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Categorical,
                description: None,
                metadata: None,
            },
            mappings,
            default_judgment: Some(JudgmentData {
                T: 0.0,
                I: 0.5,
                F: 0.5,
            }),
        };

        let mapper = CategoricalMapper::new(params).unwrap();

        // Test known category
        let judgment1 = mapper.apply("VERIFIED").unwrap();
        assert!(judgment1.is_valid());
        assert_eq!(judgment1.t, 0.9);
        assert_eq!(judgment1.i, 0.1);
        assert_eq!(judgment1.f, 0.0);

        // Test unknown category with default
        let judgment2 = mapper.apply("UNKNOWN").unwrap();
        assert!(judgment2.is_valid());
        assert_eq!(judgment2.t, 0.0);
        assert_eq!(judgment2.i, 0.5);
        assert_eq!(judgment2.f, 0.5);
    }

    #[test]
    fn test_categorical_mapper_no_default() {
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
            base: BaseMapperParams {
                id: "test-categorical".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Categorical,
                description: None,
                metadata: None,
            },
            mappings,
            default_judgment: None,
        };

        let mapper = CategoricalMapper::new(params).unwrap();

        // Test unknown category without default - should fail
        let result = mapper.apply("UNKNOWN");
        assert!(result.is_err());
    }
}
