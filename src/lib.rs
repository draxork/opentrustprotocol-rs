//! # OpenTrust Protocol (OTP) Rust SDK
//!
//! This crate is the official Rust implementation of the OpenTrust Protocol.
//!
//! It provides the necessary tools to create, validate, and fuse
//! Neutrosophic Judgments in Rust applications with **mathematical proof of conformance**
//! and **Performance Oracle capabilities**.
//!
//! ## 🦀 **The Revolutionary Additions: Conformance Seals + Performance Oracle**
//!
//! **OTP v3.0 introduces:**
//! - **Zero Pillar**: Proof-of-Conformance Seals (cryptographic proof of specification compliance)
//! - **First Pillar**: Performance Oracle (Circle of Trust for real-world outcome tracking)
//! 
//! Every fusion operation now generates a cryptographic fingerprint (SHA-256 hash) that proves
//! the operation was performed according to the exact OTP specification. Additionally, the
//! Performance Oracle system enables tracking real-world outcomes to measure the effectiveness
//! of OTP-based decisions.
//!
//! This transforms OTP from a trust protocol into **the mathematical embodiment of trust itself**.
//!
//! ## Core Components
//!
//! - [`NeutrosophicJudgment`]: The main struct for representing evidence (T, I, F).
//! - Fusion operators: Functions for combining multiple judgments with **conformance seals**.
//! - **NEW**: [`generate_conformance_seal`]: Generate cryptographic proof of conformance.
//! - **NEW**: [`verify_conformance_seal_with_inputs`]: Verify mathematical proof of conformance.
//!
//! ## Example with Conformance Seals
//!
//! ```rust
//! use opentrustprotocol::{NeutrosophicJudgment, conflict_aware_weighted_average, verify_conformance_seal_with_inputs};
//!
//! // Create judgments
//! let judgment1 = NeutrosophicJudgment::new(
//!     0.8, 0.2, 0.0,
//!     vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
//! ).unwrap();
//!
//! let judgment2 = NeutrosophicJudgment::new(
//!     0.6, 0.3, 0.1,
//!     vec![("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())]
//! ).unwrap();
//!
//! // Fuse judgments (now with automatic conformance seal generation)
//! let fused = conflict_aware_weighted_average(
//!     &[&judgment1, &judgment2],
//!     &[0.6, 0.4]
//! ).unwrap();
//!
//! // Verify the mathematical proof of conformance
//! let is_mathematically_proven = verify_conformance_seal_with_inputs(
//!     &fused,
//!     &[&judgment1, &judgment2],
//!     &[0.6, 0.4]
//! ).unwrap();
//!
//! if is_mathematically_proven {
//!     println!("✅ MATHEMATICAL PROOF: This judgment is 100% conformant to OTP specification!");
//! } else {
//!     println!("❌ WARNING: Conformance verification failed - possible tampering detected!");
//! }
//!
//! println!("Fused judgment: {}", fused);
//! ```

pub mod conformance;
pub mod error;
pub mod fusion;
pub mod judgment;
pub mod judgment_id;
pub mod mapper;

// Re-export main types and functions
pub use conformance::{
    generate_conformance_seal, verify_conformance_seal, verify_conformance_seal_with_inputs,
    create_fusion_provenance_entry,
};
pub use error::{OpenTrustError, Result};
pub use fusion::{conflict_aware_weighted_average, optimistic_fusion, pessimistic_fusion};
pub use judgment::NeutrosophicJudgment;
pub use judgment_id::{
    generate_judgment_id, ensure_judgment_id, OutcomeJudgment, OutcomeType,
};

// Re-export mapper types and functions
pub use mapper::{
    create_judgment, create_timestamp, get_global_registry, normalize_boolean_input,
    reset_global_registry, validate_judgment_values, BaseMapperParams, BooleanMapper,
    BooleanParams, CategoricalMapper, CategoricalParams, InputError, Mapper, MapperError,
    MapperParams, MapperRegistry, MapperType, MapperValidator, NumericalMapper, NumericalParams,
    ProvenanceEntry, ValidationError,
};

// Re-export mapper sub-types
pub use mapper::types::JudgmentData;
pub use mapper::validator::ValidationResult;

/// Current version of the OpenTrust Protocol SDK
/// 
/// **v3.0.0** introduces the **Performance Oracle** - the First Pillar that completes
/// the Circle of Trust, enabling real-world outcome tracking and performance measurement
/// to validate the effectiveness of OTP-based decisions.
pub const VERSION: &str = "3.0.0";
