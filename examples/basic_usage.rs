//! Basic usage example for OpenTrust Protocol Rust SDK

use opentrustprotocol::{
    conflict_aware_weighted_average, optimistic_fusion, pessimistic_fusion, NeutrosophicJudgment,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 OpenTrust Protocol Rust SDK - Basic Usage Example\n");

    // Create sample judgments
    let judgment1 = NeutrosophicJudgment::new(
        0.8,
        0.2,
        0.0,
        vec![("sensor1".to_string(), "2023-01-01T00:00:00Z".to_string())],
    )?;

    let judgment2 = NeutrosophicJudgment::new(
        0.6,
        0.3,
        0.1,
        vec![("sensor2".to_string(), "2023-01-01T00:00:00Z".to_string())],
    )?;

    let judgment3 = NeutrosophicJudgment::new(
        0.7,
        0.1,
        0.2,
        vec![("sensor3".to_string(), "2023-01-01T00:00:00Z".to_string())],
    )?;

    println!("📊 Original Judgments:");
    println!("  Judgment 1: {}", judgment1);
    println!("  Judgment 2: {}", judgment2);
    println!("  Judgment 3: {}", judgment3);
    println!();

    // Conflict-Aware Weighted Average (Recommended)
    println!("🔀 Conflict-Aware Weighted Average:");
    let cawa_result =
        conflict_aware_weighted_average(&[&judgment1, &judgment2, &judgment3], &[0.5, 0.3, 0.2])?;
    println!("  Result: {}", cawa_result);
    println!("  Total: {:.3}", cawa_result.total());
    println!();

    // Optimistic Fusion
    println!("☀️ Optimistic Fusion (Best Case):");
    let optimistic_result = optimistic_fusion(&[&judgment1, &judgment2, &judgment3])?;
    println!("  Result: {}", optimistic_result);
    println!("  Total: {:.3}", optimistic_result.total());
    println!();

    // Pessimistic Fusion
    println!("🌧️ Pessimistic Fusion (Worst Case):");
    let pessimistic_result = pessimistic_fusion(&[&judgment1, &judgment2, &judgment3])?;
    println!("  Result: {}", pessimistic_result);
    println!("  Total: {:.3}", pessimistic_result.total());
    println!();

    // JSON Serialization Example
    println!("📄 JSON Serialization:");
    let json = cawa_result.to_json()?;
    println!("  JSON: {}", json);
    println!();

    // Provenance Chain Example
    println!("🔍 Provenance Chain:");
    for (i, entry) in cawa_result.provenance_chain.iter().enumerate() {
        println!(
            "  Entry {}: {} @ {}",
            i + 1,
            entry.source_id,
            entry.timestamp
        );
        if let Some(ref desc) = entry.description {
            println!("    Description: {}", desc);
        }
    }

    Ok(())
}
