//! Integration tests for OpenTrust Protocol Rust SDK

use opentrustprotocol::{
    conflict_aware_weighted_average, optimistic_fusion, pessimistic_fusion, NeutrosophicJudgment,
    OpenTrustError,
};

/// Creates a test judgment with the given T, I, F values
fn create_test_judgment(t: f64, i: f64, f: f64, source_id: &str) -> NeutrosophicJudgment {
    NeutrosophicJudgment::new(
        t,
        i,
        f,
        vec![(source_id.to_string(), "2023-01-01T00:00:00Z".to_string())],
    )
    .unwrap()
}

#[test]
fn test_basic_judgment_creation() {
    let judgment = create_test_judgment(0.8, 0.2, 0.0, "test");

    assert_eq!(judgment.t, 0.8);
    assert_eq!(judgment.i, 0.2);
    assert_eq!(judgment.f, 0.0);
    assert!(judgment.is_valid());
    assert_eq!(judgment.total(), 1.0);
}

#[test]
fn test_conservation_constraint_validation() {
    // Valid judgment
    let valid = create_test_judgment(0.8, 0.2, 0.0, "valid");
    assert!(valid.is_valid());

    // Invalid judgment (T + I + F > 1.0)
    let result = NeutrosophicJudgment::new(
        0.5,
        0.5,
        0.3,
        vec![("test".to_string(), "2023-01-01T00:00:00Z".to_string())],
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        OpenTrustError::ConservationViolation { t, i, f, sum } => {
            assert_eq!(t, 0.5);
            assert_eq!(i, 0.5);
            assert_eq!(f, 0.3);
            assert_eq!(sum, 1.3);
        }
        _ => panic!("Expected ConservationViolation error"),
    }
}

#[test]
fn test_fusion_operations() {
    let judgment1 = create_test_judgment(0.8, 0.2, 0.0, "source1");
    let judgment2 = create_test_judgment(0.6, 0.3, 0.1, "source2");
    let judgment3 = create_test_judgment(0.7, 0.1, 0.2, "source3");

    // Conflict-Aware Weighted Average
    let cawa_result =
        conflict_aware_weighted_average(&[&judgment1, &judgment2, &judgment3], &[0.5, 0.3, 0.2])
            .unwrap();

    assert!(cawa_result.is_valid());
    assert!(cawa_result.total() <= 1.0);
    assert_eq!(cawa_result.provenance_chain.len(), 4); // 3 original + 1 fusion

    // Optimistic Fusion
    let optimistic_result = optimistic_fusion(&[&judgment1, &judgment2, &judgment3]).unwrap();
    assert!(optimistic_result.is_valid());
    assert_eq!(optimistic_result.t, 0.8); // max T
    assert_eq!(optimistic_result.f, 0.0); // min F

    // Pessimistic Fusion
    let pessimistic_result = pessimistic_fusion(&[&judgment1, &judgment2, &judgment3]).unwrap();
    assert!(pessimistic_result.is_valid());
    assert_eq!(pessimistic_result.t, 0.6); // min T
    assert_eq!(pessimistic_result.f, 0.2); // max F
}

#[test]
fn test_json_serialization_roundtrip() {
    let original = create_test_judgment(0.8, 0.2, 0.0, "json_test");
    let json = original.to_json().unwrap();
    let deserialized = NeutrosophicJudgment::from_json(&json).unwrap();

    assert!(original.equals(&deserialized, 1e-10));
    assert_eq!(
        original.provenance_chain.len(),
        deserialized.provenance_chain.len()
    );
}

#[test]
fn test_error_handling() {
    // Empty judgments list
    let result = conflict_aware_weighted_average(&[], &[]);
    assert!(result.is_err());

    // Weights length mismatch
    let judgment = create_test_judgment(0.8, 0.2, 0.0, "test");
    let result = conflict_aware_weighted_average(&[&judgment], &[0.5, 0.5]);
    assert!(result.is_err());

    // Invalid T value
    let result = NeutrosophicJudgment::new(
        1.5,
        0.0,
        0.0,
        vec![("test".to_string(), "2023-01-01T00:00:00Z".to_string())],
    );
    assert!(result.is_err());

    // Empty provenance chain
    let result = NeutrosophicJudgment::new(0.8, 0.2, 0.0, vec![]);
    assert!(result.is_err());
}

#[test]
fn test_provenance_chain_integrity() {
    let judgment1 = create_test_judgment(0.8, 0.2, 0.0, "source1");
    let judgment2 = create_test_judgment(0.6, 0.3, 0.1, "source2");

    let fused = conflict_aware_weighted_average(&[&judgment1, &judgment2], &[0.6, 0.4]).unwrap();

    // Should have 2 original entries + 1 fusion entry
    assert_eq!(fused.provenance_chain.len(), 3);

    // Check that original provenance is preserved
    assert_eq!(fused.provenance_chain[0].source_id, "source1");
    assert_eq!(fused.provenance_chain[1].source_id, "source2");
    assert_eq!(fused.provenance_chain[2].source_id, "otp-cawa-v1.1");
}

#[test]
fn test_edge_cases() {
    // Boundary values
    let judgment = create_test_judgment(1.0, 0.0, 0.0, "boundary");
    assert!(judgment.is_valid());

    // All zeros
    let judgment = create_test_judgment(0.0, 0.0, 0.0, "zeros");
    assert!(judgment.is_valid());

    // Maximum conservation (T + I + F = 1.0)
    let judgment = create_test_judgment(0.5, 0.3, 0.2, "max_conservation");
    assert!(judgment.is_valid());
    assert_eq!(judgment.total(), 1.0);
}

#[test]
fn test_performance_with_many_judgments() {
    // Create many judgments for performance testing
    let judgments: Vec<NeutrosophicJudgment> = (0..100)
        .map(|i| {
            create_test_judgment(
                0.5 + (i as f64 * 0.001),
                0.3 - (i as f64 * 0.001),
                0.2,
                &format!("source_{}", i),
            )
        })
        .collect();

    let judgment_refs: Vec<&NeutrosophicJudgment> = judgments.iter().collect();
    let weights: Vec<f64> = (0..100).map(|i| 1.0 + (i as f64 * 0.01)).collect();

    let fused = conflict_aware_weighted_average(&judgment_refs, &weights).unwrap();

    assert!(fused.is_valid());
    assert!(fused.provenance_chain.len() > 100); // Should have all original + fusion entry
}

