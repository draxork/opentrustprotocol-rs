//! **REVOLUTIONARY DEMO**: Judgment ID System + Performance Oracle
//! 
//! This example demonstrates the complete Circle of Trust functionality:
//! - Automatic Judgment ID generation in fusion operations
//! - Outcome Judgment creation for real-world results
//! - Linking decisions with outcomes for performance tracking

use opentrustprotocol::{
    NeutrosophicJudgment, OutcomeJudgment, OutcomeType,
    conflict_aware_weighted_average, optimistic_fusion, pessimistic_fusion,
    generate_judgment_id, ensure_judgment_id,
};
use opentrustprotocol::judgment::ProvenanceEntry;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 **OPENTRUST PROTOCOL v3.0 - CIRCLE OF TRUST DEMO** 🚀\n");

    // **STEP 1**: Create initial judgments from different sources
    println!("📊 **STEP 1: Creating Initial Judgments**");
    let sensor_judgment = NeutrosophicJudgment::new(
        0.8, 0.15, 0.05, // High confidence in positive outcome
        vec![("sensor-network".to_string(), "2023-01-01T10:00:00Z".to_string())]
    )?;
    
    let expert_judgment = NeutrosophicJudgment::new(
        0.6, 0.25, 0.15, // Moderate confidence with some uncertainty
        vec![("expert-analysis".to_string(), "2023-01-01T10:01:00Z".to_string())]
    )?;
    
    let market_judgment = NeutrosophicJudgment::new(
        0.7, 0.2, 0.1, // Good market conditions
        vec![("market-data".to_string(), "2023-01-01T10:02:00Z".to_string())]
    )?;

    println!("✅ Sensor Judgment: T={:.1}, I={:.1}, F={:.1}", 
             sensor_judgment.t, sensor_judgment.i, sensor_judgment.f);
    println!("✅ Expert Judgment: T={:.1}, I={:.1}, F={:.1}", 
             expert_judgment.t, expert_judgment.i, expert_judgment.f);
    println!("✅ Market Judgment: T={:.1}, I={:.1}, F={:.1}", 
             market_judgment.t, market_judgment.i, market_judgment.f);

    // **STEP 2**: Fuse judgments with automatic Judgment ID generation
    println!("\n🔄 **STEP 2: Fusion Operations with Automatic Judgment IDs**");
    
    let judgments = vec![&sensor_judgment, &expert_judgment, &market_judgment];
    let weights = vec![0.4, 0.3, 0.3]; // Sensor gets highest weight

    // Conflict-Aware Weighted Average (Primary operator)
    let fused_cawa = conflict_aware_weighted_average(&judgments, &weights)?;
    println!("🎯 CAWA Result: T={:.3}, I={:.3}, F={:.3}", 
             fused_cawa.t, fused_cawa.i, fused_cawa.f);
    println!("🔐 Judgment ID: {}", fused_cawa.judgment_id.as_ref().unwrap());

    // Optimistic Fusion (Best-case scenario)
    let fused_optimistic = optimistic_fusion(&judgments)?;
    println!("☀️ Optimistic Result: T={:.3}, I={:.3}, F={:.3}", 
             fused_optimistic.t, fused_optimistic.i, fused_optimistic.f);
    println!("🔐 Judgment ID: {}", fused_optimistic.judgment_id.as_ref().unwrap());

    // Pessimistic Fusion (Worst-case scenario)
    let fused_pessimistic = pessimistic_fusion(&judgments)?;
    println!("🌧️ Pessimistic Result: T={:.3}, I={:.3}, F={:.3}", 
             fused_pessimistic.t, fused_pessimistic.i, fused_pessimistic.f);
    println!("🔐 Judgment ID: {}", fused_pessimistic.judgment_id.as_ref().unwrap());

    // **STEP 3**: Simulate real-world outcomes
    println!("\n🌍 **STEP 3: Real-World Outcome Tracking**");
    
    // Simulate successful outcome
    let success_outcome = OutcomeJudgment::new(
        fused_cawa.judgment_id.as_ref().unwrap().clone(), // Link to original decision
        1.0, 0.0, 0.0, // Complete success
        OutcomeType::Success,
        "trading-oracle".to_string(),
        vec![ProvenanceEntry::new("trading-oracle".to_string(), "2023-01-01T15:00:00Z".to_string())],
    )?;
    
    println!("✅ Success Outcome Recorded!");
    println!("🔗 Links to Decision ID: {}", success_outcome.links_to_judgment_id);
    println!("📊 Outcome: T={:.1}, I={:.1}, F={:.1}", 
             success_outcome.t, success_outcome.i, success_outcome.f);
    println!("🔐 Outcome Judgment ID: {}", success_outcome.judgment_id);

    // **STEP 4**: Demonstrate manual Judgment ID generation
    println!("\n🛠️ **STEP 4: Manual Judgment ID Operations**");
    
    // Create a judgment without ID
    let manual_judgment = NeutrosophicJudgment::new(
        0.9, 0.1, 0.0,
        vec![("manual-input".to_string(), "2023-01-01T12:00:00Z".to_string())]
    )?;
    
    println!("📝 Manual Judgment (no ID): T={:.1}, I={:.1}, F={:.1}", 
             manual_judgment.t, manual_judgment.i, manual_judgment.f);
    println!("❓ Has Judgment ID: {}", manual_judgment.judgment_id.is_some());

    // Generate ID manually
    let manual_id = generate_judgment_id(&manual_judgment)?;
    println!("🔐 Generated Manual ID: {}", manual_id);

    // Ensure ID is set
    let judgment_with_id = ensure_judgment_id(manual_judgment)?;
    println!("✅ Judgment with ID: T={:.1}, I={:.1}, F={:.1}", 
             judgment_with_id.t, judgment_with_id.i, judgment_with_id.f);
    println!("🔐 Final Judgment ID: {}", judgment_with_id.judgment_id.unwrap());

    // **STEP 5**: Performance Oracle Analysis
    println!("\n📈 **STEP 5: Performance Oracle Analysis**");
    
    // Simulate multiple outcomes for analysis
    let outcomes = vec![
        ("Decision 1", &fused_cawa, &success_outcome),
        ("Decision 2", &fused_optimistic, &success_outcome),
        ("Decision 3", &fused_pessimistic, &success_outcome),
    ];

    println!("📊 Performance Analysis:");
    for (name, decision, outcome) in outcomes {
        let calibration = if decision.t > 0.7 && outcome.t == 1.0 {
            "✅ Well Calibrated"
        } else if decision.t <= 0.5 && outcome.t == 1.0 {
            "⚠️ Underconfident"
        } else if decision.t > 0.8 && outcome.t == 0.0 {
            "❌ Overconfident"
        } else {
            "📊 Neutral"
        };
        
        println!("  {}: {} (Decision T={:.2}, Outcome T={:.1})", 
                 name, calibration, decision.t, outcome.t);
    }

    println!("\n🎉 **CIRCLE OF TRUST COMPLETE!** 🎉");
    println!("✅ All judgments have unique IDs for tracking");
    println!("✅ Real-world outcomes are linked to decisions");
    println!("✅ Performance can be measured and analyzed");
    println!("✅ The mathematical embodiment of trust is achieved!");

    Ok(())
}
