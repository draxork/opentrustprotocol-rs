//! # OpenTrust Protocol (OTP) Rust SDK
//!
//! This crate is the official Rust implementation of the OpenTrust Protocol.
//!
//! It provides the necessary tools to create, validate, and fuse
//! Neutrosophic Judgments in Rust applications.
//!
//! ## Core Components
//!
//! - [`NeutrosophicJudgment`]: The main struct for representing evidence (T, I, F).
//! - Fusion operators: Functions for combining multiple judgments.
//!
//! ## Example
//!
//! ```rust
//! use opentrustprotocol::{NeutrosophicJudgment, conflict_aware_weighted_average};
//!
//! // Create judgments
//! let judgment1 = NeutrosophicJudgment::new(
//!     0.8, 0.2, 0.0,
//!     vec![("source1".to_string(), "2023-01-01T00:00:00Z".to_string())]
//! ).unwrap();
//!
//! let judgment2 = NeutrosophicJudgment::new(
//!     0.6, 0.3, 0.1,
//!     vec![("source2".to_string(), "2023-01-01T00:00:00Z".to_string())]
//! ).unwrap();
//!
//! // Fuse judgments
//! let fused = conflict_aware_weighted_average(
//!     &[&judgment1, &judgment2],
//!     &[0.6, 0.4]
//! ).unwrap();
//!
//! println!("Fused judgment: {}", fused);
//! ```

pub mod error;
pub mod fusion;
pub mod judgment;

// Re-export main types and functions
pub use error::{OpenTrustError, Result};
pub use fusion::{conflict_aware_weighted_average, optimistic_fusion, pessimistic_fusion};
pub use judgment::NeutrosophicJudgment;

/// Current version of the OpenTrust Protocol SDK
pub const VERSION: &str = "0.1.0";
