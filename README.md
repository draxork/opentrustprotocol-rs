# OpenTrust Protocol Rust SDK

> 🚀 **CI/CD Active**: Automated testing, linting, security audits, and crates.io publishing

[![Crates.io](https://img.shields.io/crates/v/opentrustprotocol.svg)](https://crates.io/crates/opentrustprotocol)
[![Documentation](https://docs.rs/opentrustprotocol/badge.svg)](https://docs.rs/opentrustprotocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

The official Rust implementation of the OpenTrust Protocol (OTP), the open standard for auditable trust using neutrosophic judgments.

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
opentrustprotocol = "0.1.0"
```

## 📖 Basic Usage

```rust
use opentrustprotocol::{NeutrosophicJudgment, conflict_aware_weighted_average};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create judgments
    let judgment1 = NeutrosophicJudgment::new(
        0.8, 0.2, 0.0,
        vec![("source1".to_string(), "2023-01-01T00:00:00Z".to_string())]
    )?;
    
    let judgment2 = NeutrosophicJudgment::new(
        0.6, 0.3, 0.1,
        vec![("source2".to_string(), "2023-01-01T00:00:00Z".to_string())]
    )?;
    
    // Fuse judgments using conflict-aware weighted average
    let fused = conflict_aware_weighted_average(
        &[&judgment1, &judgment2],
        &[0.6, 0.4]
    )?;
    
    println!("Fused judgment: {}", fused);
    
    Ok(())
}
```

## 🧠 Core Concepts

### Neutrosophic Judgments

A Neutrosophic Judgment represents evidence with three components:

- **T (Truth)**: Degree of truth [0.0, 1.0]
- **I (Indeterminacy)**: Degree of uncertainty [0.0, 1.0]  
- **F (Falsity)**: Degree of falsity [0.0, 1.0]

**Conservation Constraint**: T + I + F ≤ 1.0

### Fusion Operators

#### Conflict-Aware Weighted Average (Recommended)
```rust
let fused = conflict_aware_weighted_average(
    &[&judgment1, &judgment2, &judgment3],
    &[0.5, 0.3, 0.2]
)?;
```

#### Optimistic Fusion
```rust
let fused = optimistic_fusion(&[&judgment1, &judgment2])?;
// Takes max T, min F, average I
```

#### Pessimistic Fusion
```rust
let fused = pessimistic_fusion(&[&judgment1, &judgment2])?;
// Takes min T, max F, average I
```

## 🔍 Advanced Features

### Provenance Chain

Every judgment includes a complete audit trail:

```rust
let judgment = NeutrosophicJudgment::new_with_entries(
    0.8, 0.2, 0.0,
    vec![
        ProvenanceEntry::new("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string()),
        ProvenanceEntry::with_description(
            "validator".to_string(),
            "2023-01-01T00:01:00Z".to_string(),
            "Validated by automated system".to_string()
        ),
    ]
)?;

// Access provenance
for entry in &judgment.provenance_chain {
    println!("Source: {}, Time: {}", entry.source_id, entry.timestamp);
}
```

### Serialization

```rust
// Convert to JSON
let json = judgment.to_json()?;
println!("JSON: {}", json);

// Create from JSON
let judgment_from_json = NeutrosophicJudgment::from_json(&json)?;
```

### Error Handling

```rust
use opentrustprotocol::OpenTrustError;

match NeutrosophicJudgment::new(1.5, 0.0, 0.0, vec![("test".to_string(), "2023-01-01T00:00:00Z".to_string())]) {
    Err(OpenTrustError::InvalidValue { field, value, .. }) => {
        println!("Invalid {} value: {}", field, value);
    }
    Ok(judgment) => {
        println!("Valid judgment: {}", judgment);
    }
}
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run benchmarks:

```bash
cargo bench
```

## 📚 Documentation

- [API Documentation](https://docs.rs/opentrustprotocol)
- [OpenTrust Protocol Specification](https://github.com/draxork/opentrustprotocol-specification)
- [Examples](examples/)

## 🎯 Use Cases

### Blockchain & DeFi
- Smart contract risk assessment
- Decentralized identity verification
- DeFi protocol security evaluation

### AI & Machine Learning
- Model confidence scoring
- Uncertainty quantification
- Multi-source data fusion

### Enterprise Systems
- Compliance auditing
- Risk management
- Decision support systems

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run the test suite: `cargo test`
5. Commit your changes: `git commit -am 'Add feature'`
6. Push to the branch: `git push origin feature-name`
7. Submit a pull request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/draxork/opentrustprotocol-rs.git
cd opentrustprotocol-rs

# Install dependencies
cargo build

# Run tests
cargo test

# Run clippy for linting
cargo clippy

# Format code
cargo fmt
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Website**: https://opentrustprotocol.com
- **Documentation**: https://docs.opentrustprotocol.com
- **Specification**: https://github.com/draxork/opentrustprotocol-specification
- **Python SDK**: https://github.com/draxork/opentrustprotocol-py
- **JavaScript SDK**: https://github.com/draxork/opentrustprotocol-js

## 📊 Performance

The Rust SDK is optimized for high-performance applications:

- **Zero-copy** operations where possible
- **SIMD** optimizations for bulk operations
- **Memory-safe** with compile-time guarantees
- **Thread-safe** by default

Benchmark results (on typical hardware):
- Judgment creation: ~10ns
- Fusion operations: ~100ns per judgment
- JSON serialization: ~1μs per judgment
