//! Neutrosophic Judgment implementation

use crate::error::{OpenTrustError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a provenance entry in the audit chain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceEntry {
    /// Unique identifier of the source
    pub source_id: String,
    /// Timestamp of the entry (ISO 8601 format)
    pub timestamp: String,
    /// Optional description of the entry
    pub description: Option<String>,
    /// Optional metadata
    pub metadata: Option<serde_json::Value>,
    /// **NEW**: Conformance Seal for fusion operations
    /// 
    /// This field contains the SHA-256 hash that proves the operation
    /// was performed according to OTP specification. Only present for
    /// fusion operations that generate Conformance Seals.
    pub conformance_seal: Option<String>,
}

impl ProvenanceEntry {
    /// Creates a new provenance entry
    pub fn new(source_id: String, timestamp: String) -> Self {
        Self {
            source_id,
            timestamp,
            description: None,
            metadata: None,
            conformance_seal: None,
        }
    }

    /// Creates a new provenance entry with description
    pub fn with_description(source_id: String, timestamp: String, description: String) -> Self {
        Self {
            source_id,
            timestamp,
            description: Some(description),
            metadata: None,
            conformance_seal: None,
        }
    }
}

/// A Neutrosophic Judgment represents evidence with Truth (T), Indeterminacy (I), and Falsity (F) components,
/// along with an immutable provenance chain for auditability and a unique judgment ID for the Circle of Trust.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeutrosophicJudgment {
    /// **NEW**: Unique identifier for the Circle of Trust
    /// 
    /// This field contains the SHA-256 hash of the canonical representation
    /// of this judgment, used to link decisions with their real-world outcomes
    /// in the Performance Oracle system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub judgment_id: Option<String>,
    /// Truth degree [0.0, 1.0]
    pub t: f64,
    /// Indeterminacy degree [0.0, 1.0]
    pub i: f64,
    /// Falsity degree [0.0, 1.0]
    pub f: f64,
    /// Immutable audit trail
    pub provenance_chain: Vec<ProvenanceEntry>,
}

impl NeutrosophicJudgment {
    /// Creates a new NeutrosophicJudgment
    ///
    /// # Arguments
    ///
    /// * `t` - Truth degree [0.0, 1.0]
    /// * `i` - Indeterminacy degree [0.0, 1.0]
    /// * `f` - Falsity degree [0.0, 1.0]
    /// * `provenance_chain` - List of provenance entries
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails (invalid ranges or conservation constraint)
    pub fn new(t: f64, i: f64, f: f64, provenance_chain: Vec<(String, String)>) -> Result<Self> {
        let provenance_entries: Vec<ProvenanceEntry> = provenance_chain
            .into_iter()
            .map(|(source_id, timestamp)| ProvenanceEntry::new(source_id, timestamp))
            .collect();

        Self::new_with_entries(t, i, f, provenance_entries)
    }

    /// Creates a new NeutrosophicJudgment with ProvenanceEntry objects
    pub fn new_with_entries(
        t: f64,
        i: f64,
        f: f64,
        provenance_chain: Vec<ProvenanceEntry>,
    ) -> Result<Self> {
        Self::validate(t, i, f, &provenance_chain)?;

        Ok(Self {
            judgment_id: None, // Will be generated later if needed
            t,
            i,
            f,
            provenance_chain,
        })
    }

    /// Validates the judgment parameters
    fn validate(t: f64, i: f64, f: f64, provenance_chain: &[ProvenanceEntry]) -> Result<()> {
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

        // Provenance chain validation
        if provenance_chain.is_empty() {
            return Err(OpenTrustError::EmptyProvenanceChain);
        }

        for (index, entry) in provenance_chain.iter().enumerate() {
            if entry.source_id.trim().is_empty() {
                return Err(OpenTrustError::InvalidProvenanceEntry {
                    index,
                    message: "Provenance entry must have source_id".to_string(),
                });
            }
            if entry.timestamp.trim().is_empty() {
                return Err(OpenTrustError::InvalidProvenanceEntry {
                    index,
                    message: "Provenance entry must have timestamp".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Returns a JSON representation of the judgment
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| OpenTrustError::InvalidFusionInput {
            message: format!("Failed to serialize judgment: {}", e),
        })
    }

    /// Creates a judgment from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| OpenTrustError::InvalidFusionInput {
            message: format!("Failed to deserialize judgment: {}", e),
        })
    }

    /// Checks if this judgment is equal to another (within epsilon tolerance)
    pub fn equals(&self, other: &Self, epsilon: f64) -> bool {
        (self.t - other.t).abs() < epsilon
            && (self.i - other.i).abs() < epsilon
            && (self.f - other.f).abs() < epsilon
            && self.provenance_chain == other.provenance_chain
    }

    /// Returns the sum T + I + F
    pub fn total(&self) -> f64 {
        self.t + self.i + self.f
    }

    /// Returns true if the judgment satisfies conservation constraint
    pub fn is_valid(&self) -> bool {
        self.total() <= 1.0
    }
}

impl fmt::Display for NeutrosophicJudgment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NeutrosophicJudgment(T={:.3}, I={:.3}, F={:.3})",
            self.t, self.i, self.f
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judgment_creation() {
        let judgment = NeutrosophicJudgment::new(
            0.8,
            0.2,
            0.0,
            vec![("test".to_string(), "2023-01-01T00:00:00Z".to_string())],
        )
        .unwrap();

        assert_eq!(judgment.t, 0.8);
        assert_eq!(judgment.i, 0.2);
        assert_eq!(judgment.f, 0.0);
        assert_eq!(judgment.provenance_chain.len(), 1);
    }

    #[test]
    fn test_conservation_constraint() {
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
    fn test_empty_provenance() {
        let result = NeutrosophicJudgment::new(0.8, 0.2, 0.0, vec![]);

        assert!(result.is_err());
        match result.unwrap_err() {
            OpenTrustError::EmptyProvenanceChain => {}
            _ => panic!("Expected EmptyProvenanceChain error"),
        }
    }

    #[test]
    fn test_json_serialization() {
        let judgment = NeutrosophicJudgment::new(
            0.8,
            0.2,
            0.0,
            vec![("test".to_string(), "2023-01-01T00:00:00Z".to_string())],
        )
        .unwrap();

        let json = judgment.to_json().unwrap();
        let deserialized = NeutrosophicJudgment::from_json(&json).unwrap();

        assert!(judgment.equals(&deserialized, 1e-10));
    }
}

