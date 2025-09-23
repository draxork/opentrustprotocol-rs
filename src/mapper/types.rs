//! Types and traits for OTP Mappers

use crate::judgment::NeutrosophicJudgment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Enumeration of supported mapper types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MapperType {
    /// Numerical mapper for continuous data
    Numerical,
    /// Categorical mapper for discrete categories
    Categorical,
    /// Boolean mapper for boolean values
    Boolean,
}

/// Base parameters for all mappers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseMapperParams {
    /// Unique identifier for the mapper
    pub id: String,
    /// Version of the mapper
    pub version: String,
    /// Type of the mapper
    pub mapper_type: MapperType,
    /// Optional description
    pub description: Option<String>,
    /// Optional metadata
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for NumericalMapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericalParams {
    /// Base parameters
    #[serde(flatten)]
    pub base: BaseMapperParams,
    /// Point representing falsity (F=1.0)
    pub falsity_point: f64,
    /// Point representing indeterminacy (I=1.0)
    pub indeterminacy_point: f64,
    /// Point representing truth (T=1.0)
    pub truth_point: f64,
    /// Whether to clamp values to the defined range
    pub clamp_to_range: Option<bool>,
}

/// Parameters for CategoricalMapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoricalParams {
    /// Base parameters
    #[serde(flatten)]
    pub base: BaseMapperParams,
    /// Mapping from categories to judgment data
    pub mappings: HashMap<String, JudgmentData>,
    /// Default judgment for unknown categories
    pub default_judgment: Option<JudgmentData>,
}

/// Parameters for BooleanMapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooleanParams {
    /// Base parameters
    #[serde(flatten)]
    pub base: BaseMapperParams,
    /// Mapping for true values
    pub true_map: JudgmentData,
    /// Mapping for false values
    pub false_map: JudgmentData,
}

/// Judgment data structure
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgmentData {
    /// Truth degree [0.0, 1.0]
    pub T: f64,
    /// Indeterminacy degree [0.0, 1.0]
    pub I: f64,
    /// Falsity degree [0.0, 1.0]
    pub F: f64,
}

/// Union type for all mapper parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mapper_type", content = "params")]
pub enum MapperParams {
    /// Numerical mapper parameters
    #[serde(rename = "numerical")]
    Numerical(NumericalParams),
    /// Categorical mapper parameters
    #[serde(rename = "categorical")]
    Categorical(CategoricalParams),
    /// Boolean mapper parameters
    #[serde(rename = "boolean")]
    Boolean(BooleanParams),
}

/// Provenance entry for tracking transformations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEntry {
    /// Source identifier
    pub source_id: String,
    /// Timestamp of the transformation
    pub timestamp: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional metadata
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// **NEW**: Conformance Seal for fusion operations
    /// 
    /// This field contains the SHA-256 hash that proves the operation
    /// was performed according to OTP specification. Only present for
    /// fusion operations that generate Conformance Seals.
    pub conformance_seal: Option<String>,
}

/// Base trait for all mappers
pub trait Mapper: Send + Sync {
    /// Apply the mapper to transform input data
    fn apply(&self, input: &dyn std::any::Any) -> crate::Result<NeutrosophicJudgment>;

    /// Get the mapper parameters
    fn get_params(&self) -> &dyn std::any::Any;

    /// Get the mapper type
    fn get_type(&self) -> MapperType;

    /// Validate the mapper parameters
    fn validate(&self) -> crate::Result<()>;
}

/// Custom error types for mappers
#[derive(Debug, thiserror::Error)]
pub enum MapperError {
    #[error("Input error: {message}")]
    Input { message: String },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },
}

/// Input-related errors
#[derive(Debug, thiserror::Error)]
pub enum InputError {
    #[error("Invalid input type: expected {expected}, got {actual}")]
    InvalidType { expected: String, actual: String },

    #[error("Input value out of range: {value}")]
    OutOfRange { value: String },

    #[error("Invalid input format: {message}")]
    InvalidFormat { message: String },
}

/// Validation-related errors
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid judgment values: {message}")]
    InvalidJudgment { message: String },

    #[error("Conservation constraint violated: T + I + F = {sum} > 1.0")]
    ConservationViolation { sum: f64 },

    #[error("Missing required parameter: {param}")]
    MissingParameter { param: String },
}

/// Global mapper registry
static GLOBAL_REGISTRY: Mutex<Option<Arc<dyn MapperRegistry>>> = Mutex::new(None);

/// Trait for mapper registry
pub trait MapperRegistry: Send + Sync {
    /// Register a mapper
    fn register(&self, mapper: Box<dyn Mapper>) -> crate::Result<()>;

    /// Get a mapper by ID
    fn get(&self, id: &str) -> Option<Arc<dyn Mapper>>;

    /// Get all mappers of a specific type
    fn get_by_type(&self, mapper_type: MapperType) -> Vec<Arc<dyn Mapper>>;

    /// List all registered mappers
    fn list(&self) -> Vec<String>;

    /// Export all mappers as configuration
    fn export(&self) -> Vec<MapperParams>;
}

/// Create a timestamp string
pub fn create_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string()
}

/// Validate judgment values (T, I, F)
#[allow(non_snake_case)]
pub fn validate_judgment_values(T: f64, I: f64, F: f64) -> Result<(), ValidationError> {
    if !(0.0..=1.0).contains(&T) {
        return Err(ValidationError::InvalidJudgment {
            message: format!("T value must be between 0 and 1, but got {}", T),
        });
    }

    if !(0.0..=1.0).contains(&I) {
        return Err(ValidationError::InvalidJudgment {
            message: format!("I value must be between 0 and 1, but got {}", I),
        });
    }

    if !(0.0..=1.0).contains(&F) {
        return Err(ValidationError::InvalidJudgment {
            message: format!("F value must be between 0 and 1, but got {}", F),
        });
    }

    let sum = T + I + F;
    if sum > 1.0 {
        return Err(ValidationError::ConservationViolation { sum });
    }

    Ok(())
}

/// Create a NeutrosophicJudgment with provenance
#[allow(non_snake_case)]
pub fn create_judgment(
    T: f64,
    I: f64,
    F: f64,
    provenance_chain: Vec<ProvenanceEntry>,
) -> crate::Result<NeutrosophicJudgment> {
    validate_judgment_values(T, I, F)?;

    let provenance: Vec<(String, String)> = provenance_chain
        .into_iter()
        .map(|entry| (entry.source_id, entry.timestamp))
        .collect();

    NeutrosophicJudgment::new(T, I, F, provenance)
}

/// Normalize boolean input from various types
pub fn normalize_boolean_input(input: &dyn std::any::Any) -> Result<bool, InputError> {
    if let Some(val) = input.downcast_ref::<bool>() {
        return Ok(*val);
    }

    if let Some(val) = input.downcast_ref::<i32>() {
        match *val {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(InputError::InvalidFormat {
                message: format!(
                    "Integer input for BooleanMapper must be 0 or 1, got {}",
                    val
                ),
            }),
        }
    } else if let Some(val) = input.downcast_ref::<String>() {
        let lower = val.to_lowercase().trim().to_string();
        match lower.as_str() {
            "true" | "yes" | "1" | "on" | "enabled" => Ok(true),
            "false" | "no" | "0" | "off" | "disabled" => Ok(false),
            _ => Err(InputError::InvalidFormat {
                message: format!(
                    "String input must be a valid boolean representation, got '{}'",
                    val
                ),
            }),
        }
    } else {
        Err(InputError::InvalidType {
            expected: "bool, i32, or String".to_string(),
            actual: std::any::type_name_of_val(input).to_string(),
        })
    }
}

/// Get the global mapper registry
pub fn get_global_registry() -> Arc<dyn MapperRegistry> {
    let mut registry = GLOBAL_REGISTRY.lock().unwrap();
    if registry.is_none() {
        *registry = Some(Arc::new(
            crate::mapper::registry::DefaultMapperRegistry::new(),
        ));
    }
    registry.as_ref().unwrap().clone()
}

/// Reset the global mapper registry
pub fn reset_global_registry() {
    let mut registry = GLOBAL_REGISTRY.lock().unwrap();
    *registry = None;
}
