//! Error types for the OpenTrust Protocol SDK

use std::fmt;

/// Result type alias for OpenTrust Protocol operations
pub type Result<T> = std::result::Result<T, OpenTrustError>;

/// Errors that can occur in OpenTrust Protocol operations
#[derive(Debug, Clone, PartialEq)]
pub enum OpenTrustError {
    /// Invalid T, I, or F values (must be between 0.0 and 1.0)
    InvalidValue {
        field: String,
        value: f64,
        message: String,
    },
    /// Conservation constraint violated (T + I + F > 1.0)
    ConservationViolation { t: f64, i: f64, f: f64, sum: f64 },
    /// Empty provenance chain
    EmptyProvenanceChain,
    /// Invalid provenance entry
    InvalidProvenanceEntry { index: usize, message: String },
    /// Invalid input for fusion operations
    InvalidFusionInput { message: String },
    /// Weights and judgments length mismatch
    WeightsLengthMismatch {
        judgments_len: usize,
        weights_len: usize,
    },
    /// All weights are zero
    AllWeightsZero,
    /// Serialization/deserialization error
    SerializationError { message: String },
}

impl fmt::Display for OpenTrustError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenTrustError::InvalidValue {
                field,
                value,
                message,
            } => {
                write!(f, "Invalid {} value {}: {}", field, value, message)
            }
            OpenTrustError::ConservationViolation {
                t,
                i,
                f: falsity,
                sum,
            } => {
                write!(
                    f,
                    "Conservation constraint violated: T + I + F = {} + {} + {} = {} > 1.0",
                    t, i, falsity, sum
                )
            }
            OpenTrustError::EmptyProvenanceChain => {
                write!(f, "Provenance chain cannot be empty")
            }
            OpenTrustError::InvalidProvenanceEntry { index, message } => {
                write!(
                    f,
                    "Invalid provenance entry at index {}: {}",
                    index, message
                )
            }
            OpenTrustError::InvalidFusionInput { message } => {
                write!(f, "Invalid fusion input: {}", message)
            }
            OpenTrustError::WeightsLengthMismatch {
                judgments_len,
                weights_len,
            } => {
                write!(
                    f,
                    "Weights length ({}) must match judgments length ({})",
                    weights_len, judgments_len
                )
            }
            OpenTrustError::AllWeightsZero => {
                write!(f, "All weights cannot be zero")
            }
            OpenTrustError::SerializationError { message } => {
                write!(f, "Serialization error: {}", message)
            }
        }
    }
}

impl std::error::Error for OpenTrustError {}

// Conversion from mapper errors
impl From<crate::mapper::types::ValidationError> for OpenTrustError {
    fn from(error: crate::mapper::types::ValidationError) -> Self {
        match error {
            crate::mapper::types::ValidationError::InvalidJudgment { message } => {
                OpenTrustError::InvalidFusionInput { message }
            }
            crate::mapper::types::ValidationError::ConservationViolation { sum } => {
                OpenTrustError::ConservationViolation {
                    t: 0.0,
                    i: 0.0,
                    f: 0.0,
                    sum,
                }
            }
            crate::mapper::types::ValidationError::MissingParameter { param } => {
                OpenTrustError::InvalidFusionInput {
                    message: format!("Missing required parameter: {}", param),
                }
            }
        }
    }
}

impl From<crate::mapper::types::InputError> for OpenTrustError {
    fn from(error: crate::mapper::types::InputError) -> Self {
        OpenTrustError::InvalidFusionInput {
            message: error.to_string(),
        }
    }
}

impl From<crate::mapper::types::MapperError> for OpenTrustError {
    fn from(error: crate::mapper::types::MapperError) -> Self {
        OpenTrustError::InvalidFusionInput {
            message: error.to_string(),
        }
    }
}
