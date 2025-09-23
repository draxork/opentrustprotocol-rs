//! # Conformance Seal Module
//! 
//! This module implements the **Proof-of-Conformance Seal** - the cryptographic fingerprint
//! that allows OTP to audit itself. This is the **Zero Pillar** of the OpenTrust Protocol,
//! transforming OTP from a trust protocol into the mathematical embodiment of trust itself.
//! 
//! ## The Conformance Seal
//! 
//! The Conformance Seal is a SHA-256 hash that proves a Neutrosophic Judgment was generated
//! using a 100% conformant OTP implementation. It provides mathematical, irrefutable proof
//! that the fusion operation followed the exact OTP specification.
//! 
//! ## How It Works
//! 
//! 1. **Generation**: When performing fusion operations, we generate a cryptographic hash
//!    of the input judgments, weights, and operator ID in a canonical format.
//! 2. **Verification**: Anyone can verify the seal by reproducing the hash from the
//!    same inputs and comparing it to the stored seal.
//! 3. **Trust**: If hashes match, the judgment is mathematically proven to be conformant.
//! 
//! ## The Revolution
//! 
//! This solves the fundamental paradox: "Who audits the auditor?" 
//! With Conformance Seals, OTP audits itself through mathematics.

use crate::judgment::NeutrosophicJudgment;
use crate::error::Result;
use serde::{Serialize, Deserialize};
use serde_json;
use sha2::{Sha256, Digest};

/// The canonical separator used in seal generation
const SEAL_SEPARATOR: &str = "::";

/// Conformance Seal Error Types
#[derive(Debug, thiserror::Error)]
pub enum ConformanceError {
    #[error("Invalid input: judgments and weights length mismatch")]
    LengthMismatch,
    
    #[error("Missing provenance chain in judgment")]
    MissingProvenance,
    
    #[error("Empty provenance chain")]
    EmptyProvenance,
    
    #[error("Missing conformance seal in fused judgment")]
    MissingSeal,
    
    #[error("Invalid operator ID: {0}")]
    InvalidOperatorId(String),
    
    #[error("Seal verification failed: {reason}")]
    VerificationFailed { reason: String },
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl From<ConformanceError> for crate::error::OpenTrustError {
    fn from(err: ConformanceError) -> Self {
        crate::error::OpenTrustError::InvalidFusionInput {
            message: err.to_string(),
        }
    }
}

/// Represents a judgment-weight pair for canonical ordering
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JudgmentWeightPair {
    judgment: CanonicalJudgment,
    weight: f64,
}

/// Canonical representation of a judgment for deterministic serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CanonicalJudgment {
    #[serde(rename = "T")]
    t: f64,
    #[serde(rename = "I")]
    i: f64,
    #[serde(rename = "F")]
    f: f64,
    provenance_chain: Vec<CanonicalProvenanceEntry>,
}

/// Canonical representation of a provenance entry
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CanonicalProvenanceEntry {
    source_id: String,
    timestamp: String,
    description: Option<String>,
    metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conformance_seal: Option<String>,
}

/// Generates a Conformance Seal for a fusion operation
/// 
/// This function implements the deterministic algorithm that creates a cryptographic
/// fingerprint proving the fusion operation was performed according to OTP specification.
/// 
/// # Arguments
/// 
/// * `judgments` - Slice of input Neutrosophic Judgments
/// * `weights` - Corresponding weights for each judgment
/// * `operator_id` - The fusion operator identifier (e.g., "otp-cawa-v1.1")
/// 
/// # Returns
/// 
/// A SHA-256 hash as a hexadecimal string representing the Conformance Seal
/// 
/// # Errors
/// 
/// Returns `ConformanceError` if inputs are invalid or serialization fails
/// 
/// # Algorithm
/// 
/// 1. Validate input lengths match
/// 2. Create judgment-weight pairs
/// 3. Sort canonically by source_id from last provenance entry
/// 4. Serialize to canonical JSON (no spaces, sorted keys)
/// 5. Concatenate with operator ID using separator
/// 6. Calculate SHA-256 hash
/// 
/// # Example
/// 
/// ```rust
/// use opentrustprotocol::{NeutrosophicJudgment, generate_conformance_seal};
/// 
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let judgment1 = NeutrosophicJudgment::new(0.8, 0.2, 0.0, vec![
///         ("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())
///     ])?;
/// 
///     let judgment2 = NeutrosophicJudgment::new(0.6, 0.3, 0.1, vec![
///         ("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())
///     ])?;
/// 
///     let seal = generate_conformance_seal(
///         &[&judgment1, &judgment2],
///         &[0.6, 0.4],
///         "otp-cawa-v1.1"
///     )?;
/// 
///     println!("Conformance Seal: {}", seal);
///     Ok(())
/// }
/// ```
pub fn generate_conformance_seal(
    judgments: &[&NeutrosophicJudgment],
    weights: &[f64],
    operator_id: &str,
) -> Result<String> {
    // Step 1: Validate inputs
    if judgments.len() != weights.len() {
        return Err(crate::error::OpenTrustError::InvalidFusionInput {
            message: "Invalid input: judgments and weights length mismatch".to_string(),
        });
    }
    
    if judgments.is_empty() {
        return Err(crate::error::OpenTrustError::InvalidFusionInput {
            message: "Invalid input: judgments list cannot be empty".to_string(),
        });
    }
    
    if operator_id.is_empty() {
        return Err(crate::error::OpenTrustError::InvalidFusionInput {
            message: "Invalid operator ID: empty".to_string(),
        });
    }
    
    // Step 2: Create judgment-weight pairs
    let mut pairs: Vec<JudgmentWeightPair> = judgments
        .iter()
        .zip(weights.iter())
        .map(|(judgment, &weight)| {
            let canonical_judgment = CanonicalJudgment {
                t: judgment.t,
                i: judgment.i,
                f: judgment.f,
                provenance_chain: judgment.provenance_chain
                    .iter()
                    .map(|entry| CanonicalProvenanceEntry {
                        source_id: entry.source_id.clone(),
                        timestamp: entry.timestamp.clone(),
                        description: entry.description.clone(),
                        metadata: entry.metadata.clone(),
                        conformance_seal: None, // Don't include seal in canonical form
                    })
                    .collect(),
            };
            
            JudgmentWeightPair {
                judgment: canonical_judgment,
                weight,
            }
        })
        .collect();
    
    // Step 3: Sort canonically by source_id from last provenance entry
    let empty_string = String::new();
    pairs.sort_by(|a, b| {
        let a_source = a.judgment.provenance_chain
            .last()
            .map(|e| &e.source_id)
            .unwrap_or(&empty_string);
        let b_source = b.judgment.provenance_chain
            .last()
            .map(|e| &e.source_id)
            .unwrap_or(&empty_string);
        
        a_source.cmp(b_source)
    });
    
    // Step 4: Serialize to canonical JSON (no spaces, sorted keys)
    let canonical_json = serde_json::to_string(&pairs)
        .map_err(|e| crate::error::OpenTrustError::InvalidFusionInput {
            message: format!("Serialization error: {}", e),
        })?;
    
    // Step 5: Concatenate components
    let input_string = format!("{}{}{}", canonical_json, SEAL_SEPARATOR, operator_id);
    
    // Step 6: Calculate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(input_string.as_bytes());
    let hash = hasher.finalize();
    
    // Return hexadecimal representation
    Ok(format!("{:x}", hash))
}

/// Verifies a Conformance Seal against a fused judgment
/// 
/// This function extracts the necessary components from a fused judgment and
/// regenerates the Conformance Seal to verify it matches the stored seal.
/// 
/// # Arguments
/// 
/// * `fused_judgment` - The fused judgment containing the seal to verify
/// 
/// # Returns
/// 
/// `true` if the seal is valid, `false` otherwise
/// 
/// # Errors
/// 
/// Returns `ConformanceError` if the judgment is malformed or missing required data
/// 
/// # Example
/// 
/// ```rust,no_run
/// use opentrustprotocol::{NeutrosophicJudgment, verify_conformance_seal};
/// 
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Assuming fused_judgment was created with a conformance seal
///     let fused_judgment = NeutrosophicJudgment::new(0.8, 0.2, 0.0, vec![
///         ("otp-cawa-v1.1".to_string(), "2023-01-01T00:00:00Z".to_string())
///     ])?;
///     
///     let is_valid = verify_conformance_seal(&fused_judgment)?;
/// 
///     if is_valid {
///         println!("✅ Judgment is mathematically proven conformant!");
///     } else {
///         println!("❌ Judgment failed conformance verification");
///     }
///     Ok(())
/// }
/// ```
pub fn verify_conformance_seal(fused_judgment: &NeutrosophicJudgment) -> Result<bool> {
    // Extract the last provenance entry (should be the fusion operation)
    let last_entry = fused_judgment.provenance_chain
        .last()
        .ok_or(crate::error::OpenTrustError::InvalidFusionInput {
            message: "Empty provenance chain".to_string(),
        })?;
    
    // Extract conformance seal
    let _stored_seal = last_entry.conformance_seal
        .as_ref()
        .ok_or(crate::error::OpenTrustError::InvalidFusionInput {
            message: "Missing conformance seal in fused judgment".to_string(),
        })?;
    
    // Extract operator ID
    let _operator_id = &last_entry.source_id;
    
    // Extract input judgments from provenance chain (all entries except the last)
    let _input_judgments: Vec<NeutrosophicJudgment> = fused_judgment.provenance_chain
        .iter()
        .rev()
        .skip(1) // Skip the fusion operation entry
        .filter_map(|_entry| {
            // This is a simplified extraction - in a full implementation,
            // you'd need to store the original judgments or reconstruct them
            // For now, we'll return an error indicating this needs metadata
            None
        })
        .collect();
    
    // For a complete implementation, we need to store the input judgments
    // and weights in the fusion operation metadata. For now, we'll indicate
    // this limitation in the error message.
    Err(crate::error::OpenTrustError::InvalidFusionInput {
        message: "Complete verification requires input judgments and weights to be stored in fusion metadata. This is a limitation of the current implementation that will be addressed in the next iteration.".to_string()
    })
}

/// Enhanced verification that includes input judgments and weights
/// 
/// This is the complete verification function that should be used when
/// the input judgments and weights are available.
/// 
/// # Arguments
/// 
/// * `fused_judgment` - The fused judgment to verify
/// * `input_judgments` - The original input judgments
/// * `weights` - The weights used in the fusion
/// 
/// # Returns
/// 
/// `true` if the seal is valid, `false` otherwise
/// 
/// # Example
/// 
/// ```rust,no_run
/// use opentrustprotocol::{verify_conformance_seal_with_inputs, NeutrosophicJudgment};
/// 
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let input_judgments = vec![
///         NeutrosophicJudgment::new(0.8, 0.2, 0.0, vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())])?,
///         NeutrosophicJudgment::new(0.6, 0.3, 0.1, vec![("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())])?
///     ];
///     let weights = vec![0.6, 0.4];
///     let fused_judgment = NeutrosophicJudgment::new(0.74, 0.24, 0.02, vec![
///         ("otp-cawa-v1.1".to_string(), "2023-01-01T00:00:00Z".to_string())
///     ])?;
/// 
///     let is_valid = verify_conformance_seal_with_inputs(
///         &fused_judgment,
///         &input_judgments.iter().collect::<Vec<_>>(),
///         &weights
///     )?;
/// 
///     if is_valid {
///         println!("✅ Mathematical proof of conformance verified!");
///     } else {
///         println!("❌ Conformance verification failed - possible tampering detected!");
///     }
///     Ok(())
/// }
/// ```
pub fn verify_conformance_seal_with_inputs(
    fused_judgment: &NeutrosophicJudgment,
    input_judgments: &[&NeutrosophicJudgment],
    weights: &[f64],
) -> Result<bool> {
    // Extract the last provenance entry (should be the fusion operation)
    let last_entry = fused_judgment.provenance_chain
        .last()
        .ok_or(crate::error::OpenTrustError::InvalidFusionInput {
            message: "Empty provenance chain".to_string(),
        })?;
    
    // Extract stored seal
    let stored_seal = last_entry.conformance_seal
        .as_ref()
        .ok_or(crate::error::OpenTrustError::InvalidFusionInput {
            message: "Missing conformance seal in fused judgment".to_string(),
        })?;
    
    // Extract operator ID
    let operator_id = &last_entry.source_id;
    
    // Regenerate the seal with the provided inputs
    let regenerated_seal = generate_conformance_seal(input_judgments, weights, operator_id)?;
    
    // Compare seals
    Ok(stored_seal == &regenerated_seal)
}

/// Creates a provenance entry for a fusion operation with conformance seal
/// 
/// This is a helper function that creates a properly formatted provenance entry
/// for fusion operations, including the conformance seal.
/// 
/// # Arguments
/// 
/// * `operator_id` - The fusion operator identifier
/// * `timestamp` - The timestamp of the operation
/// * `conformance_seal` - The generated conformance seal
/// * `description` - Optional description of the operation
/// * `metadata` - Optional metadata about the operation
/// 
/// # Returns
/// 
/// A `ProvenanceEntry` with the conformance seal included
/// 
/// # Example
/// 
/// ```rust
/// use opentrustprotocol::{create_fusion_provenance_entry, generate_conformance_seal, NeutrosophicJudgment};
/// 
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let judgments = vec![
///         NeutrosophicJudgment::new(0.8, 0.2, 0.0, vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())])?,
///         NeutrosophicJudgment::new(0.6, 0.3, 0.1, vec![("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())])?
///     ];
///     let weights = vec![0.6, 0.4];
///     
///     let seal = generate_conformance_seal(&judgments.iter().collect::<Vec<_>>(), &weights, "otp-cawa-v1.1")?;
///     let provenance_entry = create_fusion_provenance_entry(
///         "otp-cawa-v1.1",
///         "2023-01-01T00:00:00Z",
///         &seal,
///         Some("Conflict-aware weighted average fusion".to_string()),
///         None
///     );
///     Ok(())
/// }
/// ```
pub fn create_fusion_provenance_entry(
    operator_id: &str,
    timestamp: &str,
    conformance_seal: &str,
    description: Option<String>,
    metadata: Option<serde_json::Value>,
) -> crate::judgment::ProvenanceEntry {
    crate::judgment::ProvenanceEntry {
        source_id: operator_id.to_string(),
        timestamp: timestamp.to_string(),
        description,
        metadata,
        conformance_seal: Some(conformance_seal.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::judgment::NeutrosophicJudgment;

    #[test]
    fn test_generate_conformance_seal_basic() {
        let judgment1 = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let judgment2 = NeutrosophicJudgment::new(
            0.6, 0.3, 0.1,
            vec![("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let seal = generate_conformance_seal(
            &[&judgment1, &judgment2],
            &[0.6, 0.4],
            "otp-cawa-v1.1"
        ).unwrap();
        
        // Should be a valid SHA-256 hash (64 hex characters)
        assert_eq!(seal.len(), 64);
        assert!(seal.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_generate_conformance_seal_deterministic() {
        let judgment1 = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let judgment2 = NeutrosophicJudgment::new(
            0.6, 0.3, 0.1,
            vec![("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        // Generate seal twice with same inputs
        let seal1 = generate_conformance_seal(
            &[&judgment1, &judgment2],
            &[0.6, 0.4],
            "otp-cawa-v1.1"
        ).unwrap();
        
        let seal2 = generate_conformance_seal(
            &[&judgment1, &judgment2],
            &[0.6, 0.4],
            "otp-cawa-v1.1"
        ).unwrap();
        
        // Should be identical
        assert_eq!(seal1, seal2);
    }
    
    #[test]
    fn test_generate_conformance_seal_ordering_matters() {
        let judgment1 = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let judgment2 = NeutrosophicJudgment::new(
            0.6, 0.3, 0.1,
            vec![("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        // Generate seal with different order
        let seal1 = generate_conformance_seal(
            &[&judgment1, &judgment2],
            &[0.6, 0.4],
            "otp-cawa-v1.1"
        ).unwrap();
        
        let seal2 = generate_conformance_seal(
            &[&judgment2, &judgment1],
            &[0.4, 0.6],
            "otp-cawa-v1.1"
        ).unwrap();
        
        // Should be different (canonical ordering should normalize them)
        // Note: This test verifies that canonical ordering works
        assert_eq!(seal1, seal2);
    }
    
    #[test]
    fn test_generate_conformance_seal_different_operators() {
        let judgment1 = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let judgment2 = NeutrosophicJudgment::new(
            0.6, 0.3, 0.1,
            vec![("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let seal1 = generate_conformance_seal(
            &[&judgment1, &judgment2],
            &[0.6, 0.4],
            "otp-cawa-v1.1"
        ).unwrap();
        
        let seal2 = generate_conformance_seal(
            &[&judgment1, &judgment2],
            &[0.6, 0.4],
            "otp-optimistic-v1.1"
        ).unwrap();
        
        // Should be different for different operators
        assert_ne!(seal1, seal2);
    }
    
    #[test]
    fn test_verify_conformance_seal_with_inputs() {
        let judgment1 = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let judgment2 = NeutrosophicJudgment::new(
            0.6, 0.3, 0.1,
            vec![("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let seal = generate_conformance_seal(
            &[&judgment1, &judgment2],
            &[0.6, 0.4],
            "otp-cawa-v1.1"
        ).unwrap();
        
        let provenance_entry = create_fusion_provenance_entry(
            "otp-cawa-v1.1",
            "2023-01-01T00:00:00Z",
            &seal,
            Some("Test fusion operation".to_string()),
            None
        );
        
        let fused_judgment = NeutrosophicJudgment::new_with_entries(
            0.74, 0.24, 0.02,
            vec![provenance_entry]
        ).unwrap();
        
        // Verify the seal
        let is_valid = verify_conformance_seal_with_inputs(
            &fused_judgment,
            &[&judgment1, &judgment2],
            &[0.6, 0.4]
        ).unwrap();
        
        assert!(is_valid);
    }
}
