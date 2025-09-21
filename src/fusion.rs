//! Fusion operators for OpenTrust Protocol

use crate::judgment::{NeutrosophicJudgment, ProvenanceEntry};
use crate::error::{OpenTrustError, Result};

/// Validates inputs for fusion functions
fn validate_inputs(judgments: &[&NeutrosophicJudgment], weights: Option<&[f64]>) -> Result<()> {
    if judgments.is_empty() {
        return Err(OpenTrustError::InvalidFusionInput {
            message: "Judgments list cannot be empty".to_string(),
        });
    }

    if let Some(weights) = weights {
        if judgments.len() != weights.len() {
            return Err(OpenTrustError::WeightsLengthMismatch {
                judgments_len: judgments.len(),
                weights_len: weights.len(),
            });
        }

        if !weights.iter().all(|&w| w.is_finite() && w >= 0.0) {
            return Err(OpenTrustError::InvalidFusionInput {
                message: "All weights must be finite and non-negative".to_string(),
            });
        }

        if weights.iter().sum::<f64>() == 0.0 {
            return Err(OpenTrustError::AllWeightsZero);
        }
    }

    Ok(())
}

/// Creates a new provenance entry for fusion operations
fn create_fusion_provenance(
    operator: &str,
    judgments: &[&NeutrosophicJudgment],
    weights: Option<&[f64]>,
) -> ProvenanceEntry {
    let mut metadata = serde_json::Map::new();
    metadata.insert("operator".to_string(), operator.into());
    metadata.insert("input_count".to_string(), judgments.len().into());
    
    if let Some(weights) = weights {
        metadata.insert("weights".to_string(), 
            serde_json::Value::Array(
                weights.iter().map(|&w| w.into()).collect()
            )
        );
    } else {
        metadata.insert("weights".to_string(), serde_json::Value::Null);
    }
    
    metadata.insert("version".to_string(), "0.1.0".into());

    ProvenanceEntry {
        source_id: operator.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        description: Some(format!("Fusion operation using {}", operator)),
        metadata: Some(serde_json::Value::Object(metadata)),
    }
}

/// Fuses a list of judgments using the conflict-aware weighted average.
/// This is the primary and recommended operator in OTP.
///
/// # Arguments
///
/// * `judgments` - A slice of references to NeutrosophicJudgment objects to fuse
/// * `weights` - A slice of numeric weights corresponding to each judgment
///
/// # Returns
///
/// A new NeutrosophicJudgment object representing the fused judgment
///
/// # Errors
///
/// Returns an error if validation fails
pub fn conflict_aware_weighted_average(
    judgments: &[&NeutrosophicJudgment],
    weights: &[f64],
) -> Result<NeutrosophicJudgment> {
    validate_inputs(judgments, Some(weights))?;

    // Calculate adjusted weights based on conflicts
    let adjusted_weights: Vec<f64> = judgments
        .iter()
        .zip(weights.iter())
        .map(|(&judgment, &weight)| {
            let conflict_score = judgment.t * judgment.f;
            weight * (1.0 - conflict_score)
        })
        .collect();

    let total_adjusted_weight: f64 = adjusted_weights.iter().sum();

    let (final_t, final_i, final_f) = if total_adjusted_weight == 0.0 {
        // Edge case: all adjusted weights are zero, fallback to unweighted average
        let num_judgments = judgments.len() as f64;
        let t = judgments.iter().map(|j| j.t).sum::<f64>() / num_judgments;
        let i = judgments.iter().map(|j| j.i).sum::<f64>() / num_judgments;
        let f = judgments.iter().map(|j| j.f).sum::<f64>() / num_judgments;
        (t, i, f)
    } else {
        // Normal case: use adjusted weights
        let t = judgments
            .iter()
            .zip(adjusted_weights.iter())
            .map(|(&judgment, &weight)| judgment.t * weight)
            .sum::<f64>()
            / total_adjusted_weight;

        let i = judgments
            .iter()
            .zip(adjusted_weights.iter())
            .map(|(&judgment, &weight)| judgment.i * weight)
            .sum::<f64>()
            / total_adjusted_weight;

        let f = judgments
            .iter()
            .zip(adjusted_weights.iter())
            .map(|(&judgment, &weight)| judgment.f * weight)
            .sum::<f64>()
            / total_adjusted_weight;

        (t, i, f)
    };

    // Build the new provenance chain
    let mut new_provenance = Vec::new();
    for judgment in judgments {
        new_provenance.extend(judgment.provenance_chain.clone());
    }
    new_provenance.push(create_fusion_provenance("otp-cawa-v0.1.0", judgments, Some(weights)));

    NeutrosophicJudgment::new_with_entries(final_t, final_i, final_f, new_provenance)
}

/// Fuses judgments by prioritizing the maximum T value and the minimum F value.
/// Useful for opportunity analysis or "best-case" scenarios.
///
/// # Arguments
///
/// * `judgments` - A slice of references to NeutrosophicJudgment objects
///
/// # Returns
///
/// A new NeutrosophicJudgment with the max T, min F, and average I
///
/// # Errors
///
/// Returns an error if validation fails
pub fn optimistic_fusion(judgments: &[&NeutrosophicJudgment]) -> Result<NeutrosophicJudgment> {
    validate_inputs(judgments, None)?;

    let final_t = judgments.iter().map(|j| j.t).fold(0.0, f64::max);
    let final_f = judgments.iter().map(|j| j.f).fold(1.0, f64::min);
    let final_i = judgments.iter().map(|j| j.i).sum::<f64>() / judgments.len() as f64;

    // Ensure conservation constraint is satisfied
    let total = final_t + final_i + final_f;
    let (scaled_t, scaled_i, scaled_f) = if total > 1.0 {
        // Scale down proportionally to maintain relative relationships
        (final_t / total, final_i / total, final_f / total)
    } else {
        (final_t, final_i, final_f)
    };

    // Build the new provenance chain
    let mut new_provenance = Vec::new();
    for judgment in judgments {
        new_provenance.extend(judgment.provenance_chain.clone());
    }
    new_provenance.push(create_fusion_provenance("otp-optimistic-v0.1.0", judgments, None));

    NeutrosophicJudgment::new_with_entries(scaled_t, scaled_i, scaled_f, new_provenance)
}

/// Fuses judgments by prioritizing the maximum F value and the minimum T value.
/// Indispensable for risk analysis or "worst-case" scenarios.
///
/// # Arguments
///
/// * `judgments` - A slice of references to NeutrosophicJudgment objects
///
/// # Returns
///
/// A new NeutrosophicJudgment with the max F, min T, and average I
///
/// # Errors
///
/// Returns an error if validation fails
pub fn pessimistic_fusion(judgments: &[&NeutrosophicJudgment]) -> Result<NeutrosophicJudgment> {
    validate_inputs(judgments, None)?;

    let final_t = judgments.iter().map(|j| j.t).fold(1.0, f64::min);
    let final_f = judgments.iter().map(|j| j.f).fold(0.0, f64::max);
    let final_i = judgments.iter().map(|j| j.i).sum::<f64>() / judgments.len() as f64;

    // Ensure conservation constraint is satisfied
    let total = final_t + final_i + final_f;
    let (scaled_t, scaled_i, scaled_f) = if total > 1.0 {
        // Scale down proportionally to maintain relative relationships
        (final_t / total, final_i / total, final_f / total)
    } else {
        (final_t, final_i, final_f)
    };

    // Build the new provenance chain
    let mut new_provenance = Vec::new();
    for judgment in judgments {
        new_provenance.extend(judgment.provenance_chain.clone());
    }
    new_provenance.push(create_fusion_provenance("otp-pessimistic-v0.1.0", judgments, None));

    NeutrosophicJudgment::new_with_entries(scaled_t, scaled_i, scaled_f, new_provenance)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_judgment(t: f64, i: f64, f: f64) -> NeutrosophicJudgment {
        NeutrosophicJudgment::new(
            t, i, f,
            vec![("test".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap()
    }

    #[test]
    fn test_conflict_aware_weighted_average() {
        let judgment1 = create_test_judgment(0.8, 0.2, 0.0);
        let judgment2 = create_test_judgment(0.6, 0.3, 0.1);

        let fused = conflict_aware_weighted_average(
            &[&judgment1, &judgment2],
            &[0.6, 0.4]
        ).unwrap();

        assert!(fused.is_valid());
        assert!(fused.total() <= 1.0);
        assert_eq!(fused.provenance_chain.len(), 3); // 2 original + 1 fusion
    }

    #[test]
    fn test_optimistic_fusion() {
        let judgment1 = create_test_judgment(0.8, 0.2, 0.0);
        let judgment2 = create_test_judgment(0.6, 0.3, 0.1);

        let fused = optimistic_fusion(&[&judgment1, &judgment2]).unwrap();

        assert!(fused.is_valid());
        assert!(fused.total() <= 1.0);
        assert_eq!(fused.t, 0.8); // max T
        assert_eq!(fused.f, 0.0); // min F
    }

    #[test]
    fn test_pessimistic_fusion() {
        let judgment1 = create_test_judgment(0.8, 0.2, 0.0);
        let judgment2 = create_test_judgment(0.6, 0.3, 0.1);

        let fused = pessimistic_fusion(&[&judgment1, &judgment2]).unwrap();

        assert!(fused.is_valid());
        assert!(fused.total() <= 1.0);
        assert_eq!(fused.t, 0.6); // min T
        assert_eq!(fused.f, 0.1); // max F
    }

    #[test]
    fn test_empty_judgments_error() {
        let result = conflict_aware_weighted_average(&[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_weights_length_mismatch() {
        let judgment = create_test_judgment(0.8, 0.2, 0.0);
        let result = conflict_aware_weighted_average(&[&judgment], &[0.5, 0.5]);
        assert!(result.is_err());
    }
}
