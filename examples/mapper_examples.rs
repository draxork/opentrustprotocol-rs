//! Real-world examples of OTP Mapper usage in Rust

use opentrustprotocol::{
    conflict_aware_weighted_average, get_global_registry, reset_global_registry, BaseMapperParams,
    BooleanMapper, BooleanParams, CategoricalMapper, CategoricalParams, JudgmentData,
    MapperType, MapperValidator, NumericalMapper,
    NumericalParams,
};
use std::collections::HashMap;

/// Helper function to create base mapper parameters
fn create_base_params(id: &str, mapper_type: MapperType) -> BaseMapperParams {
    BaseMapperParams {
        id: id.to_string(),
        version: "1.0.0".to_string(),
        mapper_type,
        description: Some(format!("Real-world mapper for {}", id)),
        metadata: Some({
            let mut metadata = HashMap::new();
            metadata.insert(
                "industry".to_string(),
                serde_json::Value::String("finance".to_string()),
            );
            metadata.insert(
                "use_case".to_string(),
                serde_json::Value::String(id.to_string()),
            );
            metadata
        }),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 OpenTrust Protocol Rust SDK - OTP Mapper Examples\n");

    // Reset global registry for clean examples
    reset_global_registry();
    let registry = get_global_registry();

    // Example 1: DeFi Health Factor Mapping
    println!("📊 Example 1: DeFi Health Factor Mapping");
    println!("=========================================");

    let health_factor_params = NumericalParams {
        base: create_base_params("defi-health-factor", MapperType::Numerical),
        falsity_point: 1.0,       // Liquidation threshold
        indeterminacy_point: 1.5, // Warning zone
        truth_point: 2.0,         // Safe zone
        clamp_to_range: Some(true),
    };

    let health_mapper = NumericalMapper::new(health_factor_params)?;
    registry.register(Box::new(health_mapper))?;

    // Test different health factor values
    let health_values = vec![0.8, 1.1, 1.3, 1.7, 2.2, 3.0];

    for value in health_values {
        let judgment = registry.get("defi-health-factor").unwrap().apply(&value)?;
        println!(
            "Health Factor {}: T={:.3}, I={:.3}, F={:.3}",
            value, judgment.t, judgment.i, judgment.f
        );
    }

    println!();

    // Example 2: KYC/AML Verification Status
    println!("🔐 Example 2: KYC/AML Verification Status");
    println!("==========================================");

    let mut kyc_mappings = HashMap::new();
    kyc_mappings.insert(
        "VERIFIED".to_string(),
        JudgmentData {
            T: 0.9,
            I: 0.1,
            F: 0.0, // High trust
        },
    );
    kyc_mappings.insert(
        "PENDING".to_string(),
        JudgmentData {
            T: 0.0,
            I: 1.0,
            F: 0.0, // Complete uncertainty
        },
    );
    kyc_mappings.insert(
        "REJECTED".to_string(),
        JudgmentData {
            T: 0.0,
            I: 0.0,
            F: 1.0, // Complete falsity
        },
    );
    kyc_mappings.insert(
        "UNDER_REVIEW".to_string(),
        JudgmentData {
            T: 0.2,
            I: 0.7,
            F: 0.1, // Mostly uncertain
        },
    );

    let kyc_params = CategoricalParams {
        base: create_base_params("kyc-verification", MapperType::Categorical),
        mappings: kyc_mappings,
        default_judgment: Some(JudgmentData {
            T: 0.0,
            I: 0.5,
            F: 0.5, // Default: neutral uncertainty
        }),
    };

    let kyc_mapper = CategoricalMapper::new(kyc_params)?;
    registry.register(Box::new(kyc_mapper))?;

    // Test different verification statuses
    let statuses = vec!["VERIFIED", "PENDING", "REJECTED", "UNDER_REVIEW", "UNKNOWN"];

    for status in statuses {
        let judgment = registry
            .get("kyc-verification")
            .unwrap()
            .apply(&status.to_string())?;
        println!(
            "KYC Status '{}': T={:.3}, I={:.3}, F={:.3}",
            status, judgment.t, judgment.i, judgment.f
        );
    }

    println!();

    // Example 3: IoT Temperature Sensor
    println!("🌡️  Example 3: IoT Temperature Sensor");
    println!("=====================================");

    let temp_params = NumericalParams {
        base: create_base_params("iot-temperature", MapperType::Numerical),
        falsity_point: -10.0,      // Too cold (dangerous)
        indeterminacy_point: 20.0, // Room temperature (neutral)
        truth_point: 25.0,         // Optimal temperature
        clamp_to_range: Some(true),
    };

    let temp_mapper = NumericalMapper::new(temp_params)?;
    registry.register(Box::new(temp_mapper))?;

    // Test different temperature values
    let temperatures = vec![-15.0, -5.0, 10.0, 20.0, 25.0, 30.0, 40.0];

    for temp in temperatures {
        let judgment = registry.get("iot-temperature").unwrap().apply(&temp)?;
        println!(
            "Temperature {}°C: T={:.3}, I={:.3}, F={:.3}",
            temp, judgment.t, judgment.i, judgment.f
        );
    }

    println!();

    // Example 4: SSL Certificate Validation
    println!("🔒 Example 4: SSL Certificate Validation");
    println!("========================================");

    let ssl_params = BooleanParams {
        base: create_base_params("ssl-certificate", MapperType::Boolean),
        true_map: JudgmentData {
            T: 0.9,
            I: 0.1,
            F: 0.0, // Valid certificate
        },
        false_map: JudgmentData {
            T: 0.0,
            I: 0.0,
            F: 1.0, // Invalid certificate
        },
    };

    let ssl_mapper = BooleanMapper::new(ssl_params)?;
    registry.register(Box::new(ssl_mapper))?;

    // Test different certificate states
    let cert_states = vec![true, false];

    for state in cert_states {
        let judgment = registry.get("ssl-certificate").unwrap().apply(&state)?;
        println!(
            "SSL Certificate {}: T={:.3}, I={:.3}, F={:.3}",
            state, judgment.t, judgment.i, judgment.f
        );
    }

    println!();

    // Example 5: Supply Chain Product Status
    println!("📦 Example 5: Supply Chain Product Status");
    println!("=========================================");

    let mut supply_mappings = HashMap::new();
    supply_mappings.insert(
        "MANUFACTURED".to_string(),
        JudgmentData {
            T: 0.8,
            I: 0.2,
            F: 0.0, // High confidence in manufacturing
        },
    );
    supply_mappings.insert(
        "IN_TRANSIT".to_string(),
        JudgmentData {
            T: 0.3,
            I: 0.6,
            F: 0.1, // Some uncertainty during transit
        },
    );
    supply_mappings.insert(
        "DELIVERED".to_string(),
        JudgmentData {
            T: 0.9,
            I: 0.1,
            F: 0.0, // High confidence in delivery
        },
    );
    supply_mappings.insert(
        "DAMAGED".to_string(),
        JudgmentData {
            T: 0.0,
            I: 0.3,
            F: 0.7, // Mostly false (damaged)
        },
    );
    supply_mappings.insert(
        "LOST".to_string(),
        JudgmentData {
            T: 0.0,
            I: 0.0,
            F: 1.0, // Completely false (lost)
        },
    );

    let supply_params = CategoricalParams {
        base: create_base_params("supply-chain", MapperType::Categorical),
        mappings: supply_mappings,
        default_judgment: Some(JudgmentData {
            T: 0.0,
            I: 0.8,
            F: 0.2, // Default: high uncertainty
        }),
    };

    let supply_mapper = CategoricalMapper::new(supply_params)?;
    registry.register(Box::new(supply_mapper))?;

    // Test different product statuses
    let product_statuses = vec![
        "MANUFACTURED",
        "IN_TRANSIT",
        "DELIVERED",
        "DAMAGED",
        "LOST",
        "UNKNOWN",
    ];

    for status in product_statuses {
        let judgment = registry
            .get("supply-chain")
            .unwrap()
            .apply(&status.to_string())?;
        println!(
            "Product Status '{}': T={:.3}, I={:.3}, F={:.3}",
            status, judgment.t, judgment.i, judgment.f
        );
    }

    println!();

    // Example 6: Multi-Mapper Fusion
    println!("🔄 Example 6: Multi-Mapper Fusion");
    println!("==================================");

    // Create judgments from different mappers
    let health_judgment = registry.get("defi-health-factor").unwrap().apply(&1.8f64)?;
    let kyc_judgment = registry
        .get("kyc-verification")
        .unwrap()
        .apply(&"VERIFIED".to_string())?;
    let ssl_judgment = registry.get("ssl-certificate").unwrap().apply(&true)?;

    println!("Individual Judgments:");
    println!(
        "Health Factor (1.8): T={:.3}, I={:.3}, F={:.3}",
        health_judgment.t, health_judgment.i, health_judgment.f
    );
    println!(
        "KYC (VERIFIED): T={:.3}, I={:.3}, F={:.3}",
        kyc_judgment.t, kyc_judgment.i, kyc_judgment.f
    );
    println!(
        "SSL (true): T={:.3}, I={:.3}, F={:.3}",
        ssl_judgment.t, ssl_judgment.i, ssl_judgment.f
    );

    // Fuse judgments using conflict-aware weighted average
    let judgments = vec![&health_judgment, &kyc_judgment, &ssl_judgment];
    let weights = vec![0.4, 0.4, 0.2]; // Health and KYC are more important than SSL

    let fused_judgment = conflict_aware_weighted_average(&judgments, &weights)?;

    println!("\nFused Judgment (Weighted Average):");
    println!(
        "T={:.3}, I={:.3}, F={:.3}",
        fused_judgment.t, fused_judgment.i, fused_judgment.f
    );
    println!("Total: {:.3}", fused_judgment.total());

    println!();

    // Example 7: Mapper Validation
    println!("✅ Example 7: Mapper Validation");
    println!("===============================");

    let validator = MapperValidator::new();

    // Export all registered mappers
    let exported_configs = registry.export();
    println!("Registered {} mappers:", exported_configs.len());

    for (i, config) in exported_configs.iter().enumerate() {
        let result = validator.validate(config);
        let status = if result.valid {
            "✅ Valid"
        } else {
            "❌ Invalid"
        };
        println!(
            "{}. {} - {}",
            i + 1,
            match config {
                opentrustprotocol::MapperParams::Numerical(p) => &p.base.id,
                opentrustprotocol::MapperParams::Categorical(p) => &p.base.id,
                opentrustprotocol::MapperParams::Boolean(p) => &p.base.id,
            },
            status
        );

        if !result.valid {
            for error in &result.errors {
                println!("   Error: {}", error);
            }
        }
    }

    println!();

    // Example 8: Registry Management
    println!("📋 Example 8: Registry Management");
    println!("=================================");

    println!("All registered mappers:");
    for mapper_id in registry.list() {
        println!("- {}", mapper_id);
    }

    println!("\nMappers by type:");
    let numerical_mappers = registry.get_by_type(MapperType::Numerical);
    let categorical_mappers = registry.get_by_type(MapperType::Categorical);
    let boolean_mappers = registry.get_by_type(MapperType::Boolean);

    println!("Numerical: {} mappers", numerical_mappers.len());
    println!("Categorical: {} mappers", categorical_mappers.len());
    println!("Boolean: {} mappers", boolean_mappers.len());

    println!("\n🎉 All examples completed successfully!");

    Ok(())
}
