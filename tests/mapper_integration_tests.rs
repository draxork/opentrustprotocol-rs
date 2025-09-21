//! Integration tests for OTP Mapper functionality

use opentrustprotocol::{
    get_global_registry, reset_global_registry, BaseMapperParams, BooleanMapper, BooleanParams,
    CategoricalMapper, CategoricalParams, JudgmentData, MapperType, MapperValidator,
    NumericalMapper, NumericalParams,
};
use std::collections::HashMap;

/// Helper function to create base mapper parameters
fn create_base_params(id: &str, mapper_type: MapperType) -> BaseMapperParams {
    BaseMapperParams {
        id: id.to_string(),
        version: "1.0.0".to_string(),
        mapper_type,
        description: Some(format!("Test mapper for {}", id)),
        metadata: None,
    }
}

#[test]
fn test_numerical_mapper_basic_functionality() {
    let params = NumericalParams {
        base: create_base_params("test-numerical", MapperType::Numerical),
        falsity_point: 1.0,
        indeterminacy_point: 1.5,
        truth_point: 3.0,
        clamp_to_range: Some(true),
    };

    let mapper = NumericalMapper::new(params).unwrap();

    // Test basic mapping
    let judgment = mapper.apply(2.0).unwrap();
    assert!(judgment.is_valid());
    assert!(judgment.total() <= 1.0);

    // Test edge cases
    let judgment_falsity = mapper.apply(1.0).unwrap();
    let judgment_indeterminacy = mapper.apply(1.5).unwrap();
    let judgment_truth = mapper.apply(3.0).unwrap();

    assert!(judgment_falsity.is_valid());
    assert!(judgment_indeterminacy.is_valid());
    assert!(judgment_truth.is_valid());

    // Test provenance chain
    assert_eq!(judgment.provenance_chain.len(), 1);
    let provenance = &judgment.provenance_chain[0];
    assert_eq!(provenance.source_id, "test-numerical");
}

#[test]
fn test_categorical_mapper_basic_functionality() {
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
    mappings.insert(
        "REJECTED".to_string(),
        JudgmentData {
            T: 0.0,
            I: 0.0,
            F: 1.0,
        },
    );

    let params = CategoricalParams {
        base: create_base_params("test-categorical", MapperType::Categorical),
        mappings,
        default_judgment: Some(JudgmentData {
            T: 0.0,
            I: 0.5,
            F: 0.5,
        }),
    };

    let mapper = CategoricalMapper::new(params).unwrap();

    // Test known categories
    let verified_judgment = mapper.apply("VERIFIED").unwrap();
    assert!(verified_judgment.is_valid());
    assert_eq!(verified_judgment.t, 0.9);
    assert_eq!(verified_judgment.i, 0.1);
    assert_eq!(verified_judgment.f, 0.0);

    let pending_judgment = mapper.apply("PENDING").unwrap();
    assert!(pending_judgment.is_valid());
    assert_eq!(pending_judgment.t, 0.0);
    assert_eq!(pending_judgment.i, 1.0);
    assert_eq!(pending_judgment.f, 0.0);

    // Test unknown category with default
    let unknown_judgment = mapper.apply("UNKNOWN").unwrap();
    assert!(unknown_judgment.is_valid());
    assert_eq!(unknown_judgment.t, 0.0);
    assert_eq!(unknown_judgment.i, 0.5);
    assert_eq!(unknown_judgment.f, 0.5);
}

#[test]
fn test_boolean_mapper_basic_functionality() {
    let params = BooleanParams {
        base: create_base_params("test-boolean", MapperType::Boolean),
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

    // Test boolean inputs
    let true_judgment = mapper.apply(&true).unwrap();
    assert!(true_judgment.is_valid());
    assert_eq!(true_judgment.t, 0.9);
    assert_eq!(true_judgment.i, 0.1);
    assert_eq!(true_judgment.f, 0.0);

    let false_judgment = mapper.apply(&false).unwrap();
    assert!(false_judgment.is_valid());
    assert_eq!(false_judgment.t, 0.0);
    assert_eq!(false_judgment.i, 0.1);
    assert_eq!(false_judgment.f, 0.9);

    // Test integer inputs
    let int_true_judgment = mapper.apply(&1i32).unwrap();
    assert!(int_true_judgment.is_valid());
    assert_eq!(int_true_judgment.t, 0.9);

    let int_false_judgment = mapper.apply(&0i32).unwrap();
    assert!(int_false_judgment.is_valid());
    assert_eq!(int_false_judgment.f, 0.9);

    // Test string inputs
    let string_true_judgment = mapper.apply(&"true".to_string()).unwrap();
    assert!(string_true_judgment.is_valid());
    assert_eq!(string_true_judgment.t, 0.9);

    let string_false_judgment = mapper.apply(&"false".to_string()).unwrap();
    assert!(string_false_judgment.is_valid());
    assert_eq!(string_false_judgment.f, 0.9);
}

#[test]
fn test_mapper_registry_functionality() {
    reset_global_registry();
    let registry = get_global_registry();

    // Create and register different types of mappers
    let numerical_params = NumericalParams {
        base: create_base_params("numerical-test", MapperType::Numerical),
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
        base: create_base_params("categorical-test", MapperType::Categorical),
        mappings,
        default_judgment: None,
    };

    let boolean_params = BooleanParams {
        base: create_base_params("boolean-test", MapperType::Boolean),
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

    let numerical_mapper = NumericalMapper::new(numerical_params).unwrap();
    let categorical_mapper = CategoricalMapper::new(categorical_params).unwrap();
    let boolean_mapper = BooleanMapper::new(boolean_params).unwrap();

    // Register mappers
    registry.register(Box::new(numerical_mapper)).unwrap();
    registry.register(Box::new(categorical_mapper)).unwrap();
    registry.register(Box::new(boolean_mapper)).unwrap();

    // Test retrieval
    assert!(registry.get("numerical-test").is_some());
    assert!(registry.get("categorical-test").is_some());
    assert!(registry.get("boolean-test").is_some());
    assert!(registry.get("nonexistent").is_none());

    // Test listing
    let mappers = registry.list();
    assert_eq!(mappers.len(), 3);
    assert!(mappers.contains(&"numerical-test".to_string()));
    assert!(mappers.contains(&"categorical-test".to_string()));
    assert!(mappers.contains(&"boolean-test".to_string()));

    // Test get by type
    let numerical_mappers = registry.get_by_type(MapperType::Numerical);
    let categorical_mappers = registry.get_by_type(MapperType::Categorical);
    let boolean_mappers = registry.get_by_type(MapperType::Boolean);

    assert_eq!(numerical_mappers.len(), 1);
    assert_eq!(categorical_mappers.len(), 1);
    assert_eq!(boolean_mappers.len(), 1);
}

#[test]
fn test_mapper_validator_functionality() {
    let validator = MapperValidator::new();

    // Test valid numerical mapper
    let valid_numerical = NumericalParams {
        base: create_base_params("valid-numerical", MapperType::Numerical),
        falsity_point: 1.0,
        indeterminacy_point: 1.5,
        truth_point: 3.0,
        clamp_to_range: Some(true),
    };

    let result = validator.validate(&opentrustprotocol::MapperParams::Numerical(valid_numerical));
    assert!(result.valid);
    assert!(result.errors.is_empty());

    // Test invalid numerical mapper (duplicate points)
    let invalid_numerical = NumericalParams {
        base: create_base_params("invalid-numerical", MapperType::Numerical),
        falsity_point: 1.0,
        indeterminacy_point: 1.0, // Same as falsity_point
        truth_point: 3.0,
        clamp_to_range: Some(true),
    };

    let result = validator.validate(&opentrustprotocol::MapperParams::Numerical(
        invalid_numerical,
    ));
    assert!(!result.valid);
    assert!(!result.errors.is_empty());

    // Test valid categorical mapper
    let mut mappings = HashMap::new();
    mappings.insert(
        "VERIFIED".to_string(),
        JudgmentData {
            T: 0.9,
            I: 0.1,
            F: 0.0,
        },
    );

    let valid_categorical = CategoricalParams {
        base: create_base_params("valid-categorical", MapperType::Categorical),
        mappings,
        default_judgment: None,
    };

    let result = validator.validate(&opentrustprotocol::MapperParams::Categorical(
        valid_categorical,
    ));
    assert!(result.valid);
    assert!(result.errors.is_empty());

    // Test valid boolean mapper
    let valid_boolean = BooleanParams {
        base: create_base_params("valid-boolean", MapperType::Boolean),
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

    let result = validator.validate(&opentrustprotocol::MapperParams::Boolean(valid_boolean));
    assert!(result.valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_real_world_scenarios() {
    // DeFi Health Factor Mapper
    let health_factor_params = NumericalParams {
        base: create_base_params("defi-health-factor", MapperType::Numerical),
        falsity_point: 1.0, // Liquidation threshold
        indeterminacy_point: 1.5,
        truth_point: 2.0, // Safe zone
        clamp_to_range: Some(true),
    };

    let health_mapper = NumericalMapper::new(health_factor_params).unwrap();

    // Test different health factor values
    let low_health = health_mapper.apply(1.1).unwrap();
    let medium_health = health_mapper.apply(1.7).unwrap();
    let high_health = health_mapper.apply(2.5).unwrap();

    assert!(low_health.is_valid());
    assert!(medium_health.is_valid());
    assert!(high_health.is_valid());

    // KYC Status Mapper
    let mut kyc_mappings = HashMap::new();
    kyc_mappings.insert(
        "VERIFIED".to_string(),
        JudgmentData {
            T: 0.9,
            I: 0.1,
            F: 0.0,
        },
    );
    kyc_mappings.insert(
        "PENDING".to_string(),
        JudgmentData {
            T: 0.0,
            I: 1.0,
            F: 0.0,
        },
    );
    kyc_mappings.insert(
        "REJECTED".to_string(),
        JudgmentData {
            T: 0.0,
            I: 0.0,
            F: 1.0,
        },
    );

    let kyc_params = CategoricalParams {
        base: create_base_params("kyc-status", MapperType::Categorical),
        mappings: kyc_mappings,
        default_judgment: Some(JudgmentData {
            T: 0.0,
            I: 0.5,
            F: 0.5,
        }),
    };

    let kyc_mapper = CategoricalMapper::new(kyc_params).unwrap();

    let verified_status = kyc_mapper.apply("VERIFIED").unwrap();
    let pending_status = kyc_mapper.apply("PENDING").unwrap();
    let rejected_status = kyc_mapper.apply("REJECTED").unwrap();

    assert!(verified_status.is_valid());
    assert!(pending_status.is_valid());
    assert!(rejected_status.is_valid());

    // SSL Certificate Mapper
    let ssl_params = BooleanParams {
        base: create_base_params("ssl-certificate", MapperType::Boolean),
        true_map: JudgmentData {
            T: 0.9,
            I: 0.1,
            F: 0.0,
        },
        false_map: JudgmentData {
            T: 0.0,
            I: 0.0,
            F: 1.0,
        },
    };

    let ssl_mapper = BooleanMapper::new(ssl_params).unwrap();

    let valid_ssl = ssl_mapper.apply(&true).unwrap();
    let invalid_ssl = ssl_mapper.apply(&false).unwrap();

    assert!(valid_ssl.is_valid());
    assert!(invalid_ssl.is_valid());
    assert_eq!(valid_ssl.t, 0.9);
    assert_eq!(invalid_ssl.f, 1.0);
}

#[test]
fn test_conservation_constraint() {
    // Test that all mappers maintain the conservation constraint T + I + F <= 1.0
    let numerical_params = NumericalParams {
        base: create_base_params("conservation-test", MapperType::Numerical),
        falsity_point: 1.0,
        indeterminacy_point: 1.5,
        truth_point: 3.0,
        clamp_to_range: Some(true),
    };

    let mapper = NumericalMapper::new(numerical_params).unwrap();

    // Test multiple values
    for i in 0..100 {
        let value = 1.0 + (i as f64 * 0.02);
        let judgment = mapper.apply(value).unwrap();
        assert!(judgment.is_valid());
        assert!(judgment.total() <= 1.0);
    }
}

#[test]
fn test_provenance_chain_integrity() {
    let params = NumericalParams {
        base: create_base_params("provenance-test", MapperType::Numerical),
        falsity_point: 1.0,
        indeterminacy_point: 1.5,
        truth_point: 3.0,
        clamp_to_range: Some(true),
    };

    let mapper = NumericalMapper::new(params).unwrap();
    let judgment = mapper.apply(2.0).unwrap();

    // Check provenance chain
    assert_eq!(judgment.provenance_chain.len(), 1);
    let provenance = &judgment.provenance_chain[0];
    assert_eq!(provenance.source_id, "provenance-test");
    assert!(!provenance.timestamp.is_empty()); // Timestamp should not be empty
}
