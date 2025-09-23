//! Judgment ID System for Circle of Trust
//! 
//! This module implements the Judgment ID system that enables the Performance Oracle
//! and Circle of Trust functionality. The Judgment ID is a SHA-256 hash of the
//! canonical representation of a Neutrosophic Judgment, used to link decisions
//! with their real-world outcomes.

use crate::judgment::{NeutrosophicJudgment, ProvenanceEntry};
use crate::error::{OpenTrustError, Result};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::fmt;

/// **NEW**: Outcome Judgment for Performance Oracle
/// 
/// An Outcome Judgment represents the real-world result of a decision
/// that was informed by a Neutrosophic Judgment. It links back to the
/// original decision through the `links_to_judgment_id` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutcomeJudgment {
    /// Unique identifier for this outcome judgment
    pub judgment_id: String,
    /// Links to the original decision judgment
    pub links_to_judgment_id: String,
    /// Truth degree (usually binary: 1.0 for success, 0.0 for failure)
    pub t: f64,
    /// Indeterminacy degree (usually 0.0 for outcomes)
    pub i: f64,
    /// Falsity degree (usually binary: 0.0 for success, 1.0 for failure)
    pub f: f64,
    /// Type of outcome
    pub outcome_type: OutcomeType,
    /// Source of the oracle that recorded this outcome
    pub oracle_source: String,
    /// Provenance chain for this outcome
    pub provenance_chain: Vec<ProvenanceEntry>,
}

/// Type of outcome for Performance Oracle
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OutcomeType {
    Success,
    Failure,
    Partial,
}

impl fmt::Display for OutcomeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutcomeType::Success => write!(f, "success"),
            OutcomeType::Failure => write!(f, "failure"),
            OutcomeType::Partial => write!(f, "partial"),
        }
    }
}

impl OutcomeJudgment {
    /// Creates a new Outcome Judgment
    pub fn new(
        links_to_judgment_id: String,
        t: f64,
        i: f64,
        f: f64,
        outcome_type: OutcomeType,
        oracle_source: String,
        provenance_chain: Vec<ProvenanceEntry>,
    ) -> Result<Self> {
        // Validate the outcome judgment
        Self::validate(t, i, f)?;
        
        let mut outcome = Self {
            judgment_id: String::new(), // Will be generated
            links_to_judgment_id,
            t,
            i,
            f,
            outcome_type,
            oracle_source,
            provenance_chain,
        };
        
        // Generate the judgment ID
        outcome.judgment_id = generate_judgment_id(&outcome.to_neutrosophic_judgment())?;
        
        Ok(outcome)
    }
    
    /// Converts this Outcome Judgment to a regular Neutrosophic Judgment
    /// (without the oracle-specific fields)
    pub fn to_neutrosophic_judgment(&self) -> NeutrosophicJudgment {
        NeutrosophicJudgment {
            judgment_id: Some(self.judgment_id.clone()),
            t: self.t,
            i: self.i,
            f: self.f,
            provenance_chain: self.provenance_chain.clone(),
        }
    }
    
    /// Validates the outcome judgment parameters
    fn validate(t: f64, i: f64, f: f64) -> Result<()> {
        // Range validation
        if !(0.0..=1.0).contains(&t) {
            return Err(OpenTrustError::InvalidValue {
                field: "T".to_string(),
                value: t,
                message: "T value must be between 0 and 1".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&i) {
            return Err(OpenTrustError::InvalidValue {
                field: "I".to_string(),
                value: i,
                message: "I value must be between 0 and 1".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&f) {
            return Err(OpenTrustError::InvalidValue {
                field: "F".to_string(),
                value: f,
                message: "F value must be between 0 and 1".to_string(),
            });
        }

        // Conservation constraint validation
        let sum = t + i + f;
        if sum > 1.0 {
            return Err(OpenTrustError::ConservationViolation { t, i, f, sum });
        }

        Ok(())
    }
}

/// Generates a Judgment ID for a Neutrosophic Judgment
/// 
/// The Judgment ID is a SHA-256 hash of the canonical representation
/// of the judgment, excluding the judgment_id field itself to avoid
/// recursive hashing.
/// 
/// # Arguments
/// 
/// * `judgment` - The Neutrosophic Judgment to generate an ID for
/// 
/// # Returns
/// 
/// Returns a SHA-256 hash as a hexadecimal string
/// 
/// # Example
/// 
/// ```rust
/// use opentrustprotocol::{NeutrosophicJudgment, generate_judgment_id};
/// 
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let judgment = NeutrosophicJudgment::new(
///     0.8, 0.2, 0.0,
///     vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
/// )?;
/// 
/// let judgment_id = generate_judgment_id(&judgment)?;
/// println!("Judgment ID: {}", judgment_id);
/// # Ok(())
/// # }
/// ```
pub fn generate_judgment_id(judgment: &NeutrosophicJudgment) -> Result<String> {
    // Create canonical representation without judgment_id
    let canonical = CanonicalJudgment {
        t: judgment.t,
        i: judgment.i,
        f: judgment.f,
        provenance_chain: judgment.provenance_chain.iter().map(|entry| CanonicalProvenanceEntry {
            source_id: entry.source_id.clone(),
            timestamp: entry.timestamp.clone(),
            description: entry.description.clone(),
            metadata: entry.metadata.clone(),
            // Exclude conformance_seal for consistency with existing system
        }).collect(),
    };
    
    // Serialize to canonical JSON
    let canonical_json = serde_json::to_string(&canonical)
        .map_err(|e| OpenTrustError::SerializationError {
            message: format!("Failed to serialize judgment for ID generation: {}", e),
        })?;
    
    // Generate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(canonical_json.as_bytes());
    let hash = hasher.finalize();
    
    Ok(format!("{:x}", hash))
}

/// Canonical representation of a Neutrosophic Judgment for ID generation
#[derive(Serialize, Deserialize)]
struct CanonicalJudgment {
    t: f64,
    i: f64,
    f: f64,
    provenance_chain: Vec<CanonicalProvenanceEntry>,
}

/// Canonical representation of a Provenance Entry for ID generation
#[derive(Serialize, Deserialize)]
struct CanonicalProvenanceEntry {
    source_id: String,
    timestamp: String,
    description: Option<String>,
    metadata: Option<serde_json::Value>,
}

/// Ensures a Neutrosophic Judgment has a Judgment ID
/// 
/// If the judgment already has a judgment_id, returns it unchanged.
/// If not, generates a new judgment_id and returns a new judgment with it.
/// 
/// # Arguments
/// 
/// * `judgment` - The Neutrosophic Judgment to ensure has an ID
/// 
/// # Returns
/// 
/// Returns a Neutrosophic Judgment with a judgment_id
pub fn ensure_judgment_id(mut judgment: NeutrosophicJudgment) -> Result<NeutrosophicJudgment> {
    if judgment.judgment_id.is_none() {
        judgment.judgment_id = Some(generate_judgment_id(&judgment)?);
    }
    Ok(judgment)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::judgment::ProvenanceEntry;
    
    #[test]
    fn test_generate_judgment_id_basic() {
        let judgment = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let judgment_id = generate_judgment_id(&judgment).unwrap();
        
        // Should be a valid SHA-256 hash (64 hex characters)
        assert_eq!(judgment_id.len(), 64);
        assert!(judgment_id.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_generate_judgment_id_deterministic() {
        let judgment1 = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let judgment2 = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let id1 = generate_judgment_id(&judgment1).unwrap();
        let id2 = generate_judgment_id(&judgment2).unwrap();
        
        // Should be identical for identical judgments
        assert_eq!(id1, id2);
    }
    
    #[test]
    fn test_generate_judgment_id_different_judgments() {
        let judgment1 = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let judgment2 = NeutrosophicJudgment::new(
            0.7, 0.3, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        let id1 = generate_judgment_id(&judgment1).unwrap();
        let id2 = generate_judgment_id(&judgment2).unwrap();
        
        // Should be different for different judgments
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_ensure_judgment_id_new() {
        let judgment = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        assert!(judgment.judgment_id.is_none());
        
        let judgment_with_id = ensure_judgment_id(judgment).unwrap();
        
        assert!(judgment_with_id.judgment_id.is_some());
        assert_eq!(judgment_with_id.judgment_id.as_ref().unwrap().len(), 64);
    }
    
    #[test]
    fn test_ensure_judgment_id_existing() {
        let mut judgment = NeutrosophicJudgment::new(
            0.8, 0.2, 0.0,
            vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
        ).unwrap();
        
        judgment.judgment_id = Some("existing_id".to_string());
        
        let judgment_with_id = ensure_judgment_id(judgment.clone()).unwrap();
        
        assert_eq!(judgment_with_id.judgment_id, Some("existing_id".to_string()));
    }
    
    #[test]
    fn test_outcome_judgment_creation() {
        let outcome = OutcomeJudgment::new(
            "original_judgment_id".to_string(),
            1.0, 0.0, 0.0,
            OutcomeType::Success,
            "test-oracle".to_string(),
            vec![ProvenanceEntry::new(
                "test-oracle".to_string(),
                "2023-01-01T00:00:00Z".to_string()
            )],
        ).unwrap();
        
        assert_eq!(outcome.links_to_judgment_id, "original_judgment_id");
        assert_eq!(outcome.t, 1.0);
        assert_eq!(outcome.i, 0.0);
        assert_eq!(outcome.f, 0.0);
        assert_eq!(outcome.outcome_type, OutcomeType::Success);
        assert_eq!(outcome.oracle_source, "test-oracle");
        assert!(!outcome.judgment_id.is_empty());
    }
    
    #[test]
    fn test_outcome_judgment_validation() {
        // Test invalid T value
        let result = OutcomeJudgment::new(
            "original_judgment_id".to_string(),
            1.5, 0.0, 0.0, // Invalid T > 1.0
            OutcomeType::Success,
            "test-oracle".to_string(),
            vec![],
        );
        
        assert!(result.is_err());
    }
}
