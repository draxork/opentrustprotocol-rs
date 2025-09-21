# 🦀 OpenTrust Protocol (OTP) - Rust SDK

[![Crates.io](https://img.shields.io/crates/v/opentrustprotocol.svg)](https://crates.io/crates/opentrustprotocol)
[![Documentation](https://docs.rs/opentrustprotocol/badge.svg)](https://docs.rs/opentrustprotocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

> **The official Rust implementation of the OpenTrust Protocol - The open standard for auditable trust in AI and blockchain systems**

## 🚀 **What is OpenTrust Protocol?**

The OpenTrust Protocol (OTP) is a revolutionary framework for representing and managing **uncertainty, trust, and auditability** in AI systems, blockchain applications, and distributed networks. Built on **neutrosophic logic**, OTP provides a mathematical foundation for handling incomplete, inconsistent, and uncertain information.

### **🎯 Why OTP Matters**

- **🔒 Trust & Security**: Quantify trust levels in AI decisions and blockchain transactions
- **📊 Uncertainty Management**: Handle incomplete and contradictory information gracefully  
- **🔍 Full Auditability**: Complete provenance chain for every decision
- **🌐 Cross-Platform**: Interoperable across Python, JavaScript, Rust, and more
- **⚡ Performance**: Zero-cost abstractions with memory safety guarantees

## 🦀 **Rust SDK Features**

### **Core Components**
- **Neutrosophic Judgments**: Represent evidence as (T, I, F) values where T + I + F ≤ 1.0
- **Fusion Operators**: Combine multiple judgments with conflict-aware algorithms
- **OTP Mappers**: Transform raw data into neutrosophic judgments
- **Provenance Chain**: Complete audit trail for every transformation

### **🆕 OTP Mapper System (v0.2.0)**

Transform any data type into neutrosophic judgments:

```rust
use opentrustprotocol::*;

// DeFi Health Factor Mapping
let health_mapper = NumericalMapper::new(NumericalParams {
    base: BaseMapperParams {
        id: "defi-health-factor".to_string(),
        version: "1.0.0".to_string(),
        mapper_type: MapperType::Numerical,
        description: Some("DeFi health factor mapper".to_string()),
        metadata: None,
    },
    falsity_point: 1.0,    // Liquidation threshold
    indeterminacy_point: 1.5, // Warning zone  
    truth_point: 2.0,      // Safe zone
    clamp_to_range: Some(true),
})?;

// Transform health factor to neutrosophic judgment
let judgment = health_mapper.apply(1.8)?;
println!("Health Factor 1.8: T={:.3}, I={:.3}, F={:.3}", 
         judgment.t, judgment.i, judgment.f);
```

### **Available Mappers**

| Mapper Type | Use Case | Example |
|-------------|----------|---------|
| **NumericalMapper** | Continuous data interpolation | DeFi health factors, IoT sensors |
| **CategoricalMapper** | Discrete category mapping | KYC status, product categories |
| **BooleanMapper** | Boolean value transformation | SSL certificates, feature flags |

## 📦 **Installation**

Add to your `Cargo.toml`:

```toml
[dependencies]
opentrustprotocol = "0.2.0"
```

## 🚀 **Quick Start**

### **Basic Neutrosophic Judgment**

```rust
use opentrustprotocol::*;

// Create judgments with provenance
let judgment1 = NeutrosophicJudgment::new(
    0.8, 0.2, 0.0,  // T=0.8, I=0.2, F=0.0
    vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())]
)?;

let judgment2 = NeutrosophicJudgment::new(
    0.6, 0.3, 0.1,  // T=0.6, I=0.3, F=0.1
    vec![("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())]
)?;

// Fuse judgments with conflict-aware weighted average
let fused = conflict_aware_weighted_average(
    &[&judgment1, &judgment2],
    &[0.6, 0.4]  // weights
)?;

println!("Fused: {}", fused);
```

### **Real-World Example: DeFi Risk Assessment**

```rust
use opentrustprotocol::*;
use std::collections::HashMap;

// 1. Health Factor Mapper
let health_mapper = NumericalMapper::new(NumericalParams {
    base: BaseMapperParams {
        id: "health-factor".to_string(),
        version: "1.0.0".to_string(),
        mapper_type: MapperType::Numerical,
        description: None,
        metadata: None,
    },
    falsity_point: 1.0,
    indeterminacy_point: 1.5,
    truth_point: 2.0,
    clamp_to_range: Some(true),
})?;

// 2. KYC Status Mapper
let mut kyc_mappings = HashMap::new();
kyc_mappings.insert("VERIFIED".to_string(), JudgmentData {
    T: 0.9, I: 0.1, F: 0.0,
});

let kyc_mapper = CategoricalMapper::new(CategoricalParams {
    base: BaseMapperParams {
        id: "kyc-status".to_string(),
        version: "1.0.0".to_string(),
        mapper_type: MapperType::Categorical,
        description: None,
        metadata: None,
    },
    mappings: kyc_mappings,
    default_judgment: None,
})?;

// 3. SSL Certificate Mapper
let ssl_mapper = BooleanMapper::new(BooleanParams {
    base: BaseMapperParams {
        id: "ssl-cert".to_string(),
        version: "1.0.0".to_string(),
        mapper_type: MapperType::Boolean,
        description: None,
        metadata: None,
    },
    true_map: JudgmentData { T: 0.9, I: 0.1, F: 0.0 },
    false_map: JudgmentData { T: 0.0, I: 0.0, F: 1.0 },
})?;

// 4. Transform data to judgments
let health_judgment = health_mapper.apply(1.8)?;
let kyc_judgment = kyc_mapper.apply("VERIFIED")?;
let ssl_judgment = ssl_mapper.apply(true)?;

// 5. Fuse for final risk assessment
let risk_assessment = conflict_aware_weighted_average(
    &[&health_judgment, &kyc_judgment, &ssl_judgment],
    &[0.5, 0.3, 0.2]  // Health factor most important
)?;

println!("DeFi Risk Assessment: T={:.3}, I={:.3}, F={:.3}", 
         risk_assessment.t, risk_assessment.i, risk_assessment.f);
```

## 🏗️ **Architecture**

### **Memory Safety & Performance**

- **🔒 Memory Safe**: No null pointers, no data races
- **⚡ Zero-Cost Abstractions**: Zero runtime overhead
- **🔄 Thread Safe**: `Arc<RwLock<>>` for concurrent access
- **📦 Minimal Dependencies**: Only `serde`, `serde_json`, and `thiserror`

### **Mapper Registry System**

```rust
use opentrustprotocol::*;

let registry = get_global_registry();

// Register mappers
registry.register(Box::new(health_mapper))?;
registry.register(Box::new(kyc_mapper))?;

// Retrieve and use
let mapper = registry.get("health-factor")?;
let judgment = mapper.apply(1.5)?;

// Export configurations
let configs = registry.export();
```

## 🧪 **Testing**

Run the comprehensive test suite:

```bash
cargo test
```

Run examples:

```bash
cargo run --example mapper_examples
```

## 📊 **Use Cases**

### **🔗 Blockchain & DeFi**
- **Risk Assessment**: Health factors, liquidation risks
- **KYC/AML**: Identity verification, compliance scoring
- **Oracle Reliability**: Data source trust evaluation

### **🤖 AI & Machine Learning**
- **Uncertainty Quantification**: Model confidence scoring
- **Data Quality**: Input validation and reliability
- **Decision Fusion**: Multi-model ensemble decisions

### **🌐 IoT & Sensors**
- **Sensor Reliability**: Temperature, pressure, motion sensors
- **Data Fusion**: Multi-sensor decision making
- **Anomaly Detection**: Trust-based outlier identification

### **🏭 Supply Chain**
- **Product Tracking**: Status monitoring and verification
- **Quality Control**: Defect detection and classification
- **Compliance**: Regulatory requirement tracking

## 🔧 **Advanced Features**

### **Custom Mapper Creation**

```rust
// Create your own mapper by implementing the Mapper trait
struct CustomMapper {
    // Your implementation
}

impl Mapper for CustomMapper {
    fn apply(&self, input: &dyn std::any::Any) -> Result<NeutrosophicJudgment> {
        // Your transformation logic
    }
    
    fn get_params(&self) -> &dyn std::any::Any {
        // Return your parameters
    }
    
    fn get_type(&self) -> MapperType {
        // Return your mapper type
    }
    
    fn validate(&self) -> Result<()> {
        // Validate your parameters
    }
}
```

### **JSON Schema Validation**

```rust
let validator = MapperValidator::new();
let result = validator.validate(&mapper_params);

if result.valid {
    println!("✅ Valid mapper configuration");
} else {
    for error in result.errors {
        println!("❌ Validation error: {}", error);
    }
}
```

## 🌟 **Why Choose OTP Rust SDK?**

### **🚀 Performance**
- **Zero-cost abstractions** - No runtime overhead
- **Memory safe** - No garbage collector, no memory leaks
- **Fast compilation** - Optimized for development speed

### **🔒 Safety**
- **Memory safety** - Compile-time guarantees
- **Thread safety** - Safe concurrent access
- **Type safety** - Strong typing prevents errors

### **🔧 Developer Experience**
- **Rich error messages** - Clear, actionable feedback
- **Comprehensive docs** - Extensive documentation and examples
- **Active community** - Growing ecosystem and support

## 📈 **Performance Benchmarks**

| Operation | Time | Memory |
|-----------|------|--------|
| Judgment Creation | < 1μs | 48 bytes |
| Mapper Application | < 2μs | 64 bytes |
| Fusion (10 judgments) | < 5μs | 256 bytes |

## 🤝 **Contributing**

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### **Development Setup**

```bash
git clone https://github.com/draxork/opentrustprotocol-rs.git
cd opentrustprotocol-rs
cargo test
cargo run --example mapper_examples
```

## 📚 **Documentation**

- **[API Documentation](https://docs.rs/opentrustprotocol)** - Complete API reference
- **[Examples](examples/)** - Real-world usage examples
- **[Specification](https://github.com/draxork/opentrustprotocol-specification)** - OTP v2.0 specification

## 🌐 **Ecosystem**

OTP is available across multiple platforms:

| Platform | Package | Status |
|----------|---------|--------|
| **Rust** | `opentrustprotocol` | ✅ v0.2.0 |
| **Python** | `opentrustprotocol` | ✅ v1.0.6 |
| **JavaScript** | `opentrustprotocol` | ✅ v1.0.3 |

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 **Acknowledgments**

- **Neutrosophic Logic**: Founded by Florentin Smarandache
- **Rust Community**: For the amazing language and ecosystem
- **Open Source Contributors**: Making trust auditable for everyone

---

<div align="center">

**🌟 Star this repository if you find it useful!**

[![GitHub stars](https://img.shields.io/github/stars/draxork/opentrustprotocol-rs?style=social)](https://github.com/draxork/opentrustprotocol-rs)

**Made with ❤️ by the OpenTrust Protocol Team**

</div>