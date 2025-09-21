//! OTP Mapper module for OpenTrust Protocol Rust SDK
//!
//! This module provides mappers for transforming raw data into Neutrosophic Judgments.
//! It includes Numerical, Categorical, and Boolean mappers with full provenance support.

pub mod boolean;
pub mod categorical;
pub mod numerical;
pub mod registry;
pub mod types;
pub mod validator;

// Re-export main types and traits
pub use boolean::BooleanMapper;
pub use categorical::CategoricalMapper;
pub use numerical::NumericalMapper;
pub use registry::{get_global_registry, reset_global_registry};
pub use types::MapperRegistry;
pub use types::{
    create_judgment, create_timestamp, normalize_boolean_input, validate_judgment_values,
    BaseMapperParams, BooleanParams, CategoricalParams, InputError, Mapper, MapperError,
    MapperParams, MapperType, NumericalParams, ProvenanceEntry, ValidationError,
};
pub use validator::MapperValidator;
