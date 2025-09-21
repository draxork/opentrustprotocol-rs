//! MapperRegistry implementation for centralized mapper management

use crate::mapper::types::{Mapper, MapperParams, MapperRegistry, MapperType};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

/// Default implementation of MapperRegistry
pub struct DefaultMapperRegistry {
    mappers: RwLock<HashMap<String, Arc<dyn Mapper>>>,
}

impl DefaultMapperRegistry {
    /// Create a new DefaultMapperRegistry
    pub fn new() -> Self {
        Self {
            mappers: RwLock::new(HashMap::new()),
        }
    }

    /// Register a mapper with the given ID
    fn register_internal(&self, id: String, mapper: Arc<dyn Mapper>) -> crate::Result<()> {
        let mut mappers = self.mappers.write().unwrap();

        if mappers.contains_key(&id) {
            return Err(crate::error::OpenTrustError::InvalidFusionInput {
                message: format!("Mapper with ID '{}' already exists", id),
            }
            .into());
        }

        // Validate the mapper before registering
        mapper.validate()?;

        mappers.insert(id, mapper);
        Ok(())
    }
}

impl MapperRegistry for DefaultMapperRegistry {
    fn register(&self, mapper: Box<dyn Mapper>) -> crate::Result<()> {
        // Get the ID from the mapper parameters
        let id = match mapper.get_type() {
            MapperType::Numerical => {
                if let Some(params) = mapper
                    .get_params()
                    .downcast_ref::<crate::mapper::types::NumericalParams>()
                {
                    params.base.id.clone()
                } else {
                    return Err(crate::error::OpenTrustError::InvalidFusionInput {
                        message: "Failed to extract ID from NumericalMapper parameters".to_string(),
                    }
                    .into());
                }
            }
            MapperType::Categorical => {
                if let Some(params) = mapper
                    .get_params()
                    .downcast_ref::<crate::mapper::types::CategoricalParams>()
                {
                    params.base.id.clone()
                } else {
                    return Err(crate::error::OpenTrustError::InvalidFusionInput {
                        message: "Failed to extract ID from CategoricalMapper parameters"
                            .to_string(),
                    }
                    .into());
                }
            }
            MapperType::Boolean => {
                if let Some(params) = mapper
                    .get_params()
                    .downcast_ref::<crate::mapper::types::BooleanParams>()
                {
                    params.base.id.clone()
                } else {
                    return Err(crate::error::OpenTrustError::InvalidFusionInput {
                        message: "Failed to extract ID from BooleanMapper parameters".to_string(),
                    }
                    .into());
                }
            }
        };

        self.register_internal(id, Arc::from(mapper))
    }

    fn get(&self, id: &str) -> Option<Arc<dyn Mapper>> {
        let mappers = self.mappers.read().unwrap();
        mappers.get(id).cloned()
    }

    fn get_by_type(&self, mapper_type: MapperType) -> Vec<Arc<dyn Mapper>> {
        let mappers = self.mappers.read().unwrap();
        mappers
            .values()
            .filter(|mapper| mapper.get_type() == mapper_type)
            .cloned()
            .collect()
    }

    fn list(&self) -> Vec<String> {
        let mappers = self.mappers.read().unwrap();
        mappers.keys().cloned().collect()
    }

    fn export(&self) -> Vec<MapperParams> {
        let mappers = self.mappers.read().unwrap();
        let mut configs = Vec::new();

        for mapper in mappers.values() {
            match mapper.get_type() {
                MapperType::Numerical => {
                    if let Some(params) = mapper
                        .get_params()
                        .downcast_ref::<crate::mapper::types::NumericalParams>()
                    {
                        configs.push(MapperParams::Numerical(params.clone()));
                    }
                }
                MapperType::Categorical => {
                    if let Some(params) = mapper
                        .get_params()
                        .downcast_ref::<crate::mapper::types::CategoricalParams>()
                    {
                        configs.push(MapperParams::Categorical(params.clone()));
                    }
                }
                MapperType::Boolean => {
                    if let Some(params) = mapper
                        .get_params()
                        .downcast_ref::<crate::mapper::types::BooleanParams>()
                    {
                        configs.push(MapperParams::Boolean(params.clone()));
                    }
                }
            }
        }

        configs
    }
}

impl Default for DefaultMapperRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the global mapper registry
pub fn get_global_registry() -> Arc<dyn MapperRegistry> {
    use crate::mapper::types::get_global_registry;
    get_global_registry()
}

/// Reset the global mapper registry
pub fn reset_global_registry() {
    use crate::mapper::types::reset_global_registry;
    reset_global_registry();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapper::types::{BooleanParams, CategoricalParams, JudgmentData, NumericalParams};
    use crate::mapper::{BooleanMapper, CategoricalMapper, NumericalMapper};
    use std::collections::HashMap;

    #[test]
    fn test_registry_creation() {
        let registry = DefaultMapperRegistry::new();
        assert!(registry.list().is_empty());
    }

    #[test]
    fn test_registry_register_numerical() {
        let registry = DefaultMapperRegistry::new();

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

        let mapper = NumericalMapper::new(params).unwrap();
        registry.register(Box::new(mapper)).unwrap();

        assert_eq!(registry.list().len(), 1);
        assert!(registry.get("test-numerical").is_some());
    }

    #[test]
    fn test_registry_register_categorical() {
        let registry = DefaultMapperRegistry::new();

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

        let mapper = CategoricalMapper::new(params).unwrap();
        registry.register(Box::new(mapper)).unwrap();

        assert_eq!(registry.list().len(), 1);
        assert!(registry.get("test-categorical").is_some());
    }

    #[test]
    fn test_registry_register_boolean() {
        let registry = DefaultMapperRegistry::new();

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

        let mapper = BooleanMapper::new(params).unwrap();
        registry.register(Box::new(mapper)).unwrap();

        assert_eq!(registry.list().len(), 1);
        assert!(registry.get("test-boolean").is_some());
    }

    #[test]
    fn test_registry_duplicate_id() {
        let registry = DefaultMapperRegistry::new();

        let params1 = NumericalParams {
            base: crate::mapper::types::BaseMapperParams {
                id: "test".to_string(),
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

        let params2 = NumericalParams {
            base: crate::mapper::types::BaseMapperParams {
                id: "test".to_string(), // Same ID
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Numerical,
                description: None,
                metadata: None,
            },
            falsity_point: 2.0,
            indeterminacy_point: 2.5,
            truth_point: 4.0,
            clamp_to_range: Some(true),
        };

        let mapper1 = NumericalMapper::new(params1).unwrap();
        let mapper2 = NumericalMapper::new(params2).unwrap();

        registry.register(Box::new(mapper1)).unwrap();
        let result = registry.register(Box::new(mapper2));
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_get_by_type() {
        let registry = DefaultMapperRegistry::new();

        // Register different types of mappers
        let numerical_params = NumericalParams {
            base: crate::mapper::types::BaseMapperParams {
                id: "numerical-test".to_string(),
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

        let mut mappings = HashMap::new();
        mappings.insert(
            "VERIFIED".to_string(),
            JudgmentData {
                T: 0.9,
                I: 0.1,
                F: 0.0,
            },
        );

        let categorical_params = CategoricalParams {
            base: crate::mapper::types::BaseMapperParams {
                id: "categorical-test".to_string(),
                version: "1.0.0".to_string(),
                mapper_type: MapperType::Categorical,
                description: None,
                metadata: None,
            },
            mappings,
            default_judgment: None,
        };

        let numerical_mapper = NumericalMapper::new(numerical_params).unwrap();
        let categorical_mapper = CategoricalMapper::new(categorical_params).unwrap();

        registry.register(Box::new(numerical_mapper)).unwrap();
        registry.register(Box::new(categorical_mapper)).unwrap();

        let numerical_mappers = registry.get_by_type(MapperType::Numerical);
        let categorical_mappers = registry.get_by_type(MapperType::Categorical);

        assert_eq!(numerical_mappers.len(), 1);
        assert_eq!(categorical_mappers.len(), 1);
    }

    #[test]
    fn test_registry_export() {
        let registry = DefaultMapperRegistry::new();

        let params = NumericalParams {
            base: crate::mapper::types::BaseMapperParams {
                id: "test-export".to_string(),
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
        registry.register(Box::new(mapper)).unwrap();

        let exported = registry.export();
        assert_eq!(exported.len(), 1);

        match &exported[0] {
            MapperParams::Numerical(params) => {
                assert_eq!(params.base.id, "test-export");
            }
            _ => panic!("Expected Numerical mapper"),
        }
    }
}
