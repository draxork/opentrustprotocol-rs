//! # Conformance Seal Demo
//! 
//! This example demonstrates the revolutionary **Conformance Seal** functionality
//! that transforms OTP from a trust protocol into the mathematical embodiment of trust itself.
//! 
//! ## What You'll See
//! 
//! 1. **Generate Conformance Seals**: Every fusion operation now generates a cryptographic fingerprint
//! 2. **Verify Mathematical Proof**: Anyone can verify that a judgment was computed according to OTP spec
//! 3. **Tamper Detection**: Any modification to inputs or algorithm will break the seal
//! 
//! ## The Revolution
//! 
//! This solves the fundamental paradox: "Who audits the auditor?"
//! With Conformance Seals, OTP audits itself through mathematics.

use opentrustprotocol::{
    NeutrosophicJudgment, 
    conflict_aware_weighted_average, 
    generate_conformance_seal,
    verify_conformance_seal_with_inputs
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 OpenTrust Protocol v0.3.0 - Conformance Seal Demo");
    println!("==================================================");
    println!();
    
    // Create some sensor judgments
    println!("📡 Creating sensor judgments...");
    let sensor1 = NeutrosophicJudgment::new(
        0.8, 0.2, 0.0,  // High truth, low indeterminacy, no falsity
        vec![("temperature_sensor".to_string(), "2023-01-01T10:00:00Z".to_string())]
    )?;
    
    let sensor2 = NeutrosophicJudgment::new(
        0.6, 0.3, 0.1,  // Medium truth, some indeterminacy, low falsity
        vec![("humidity_sensor".to_string(), "2023-01-01T10:00:00Z".to_string())]
    )?;
    
    let sensor3 = NeutrosophicJudgment::new(
        0.9, 0.05, 0.05,  // Very high truth, minimal indeterminacy/falsity
        vec![("pressure_sensor".to_string(), "2023-01-01T10:00:00Z".to_string())]
    )?;
    
    println!("✅ Sensor 1: {}", sensor1);
    println!("✅ Sensor 2: {}", sensor2);
    println!("✅ Sensor 3: {}", sensor3);
    println!();
    
    // Perform fusion with automatic Conformance Seal generation
    println!("🔄 Performing fusion with Conformance Seal generation...");
    let judgments = [&sensor1, &sensor2, &sensor3];
    let weights = [0.4, 0.3, 0.3];
    
    let fused = conflict_aware_weighted_average(&judgments, &weights)?;
    println!("✅ Fused Judgment: {}", fused);
    println!();
    
    // Extract the Conformance Seal from the provenance chain
    let last_entry = fused.provenance_chain.last().unwrap();
    let conformance_seal = last_entry.conformance_seal.as_ref().unwrap();
    
    println!("🔐 Conformance Seal: {}", conformance_seal);
    println!("📊 Seal Length: {} characters (SHA-256)", conformance_seal.len());
    println!();
    
    // Demonstrate manual seal generation
    println!("🔧 Manually generating Conformance Seal...");
    let manual_seal = generate_conformance_seal(&judgments, &weights, "otp-cawa-v1.1")?;
    println!("🔐 Manual Seal: {}", manual_seal);
    
    if manual_seal == *conformance_seal {
        println!("✅ SEALS MATCH! Mathematical proof of consistency.");
    } else {
        println!("❌ SEALS DIFFER! Something is wrong.");
    }
    println!();
    
    // Verify the seal
    println!("🔍 Verifying Conformance Seal...");
    let is_valid = verify_conformance_seal_with_inputs(&fused, &judgments, &weights)?;
    
    if is_valid {
        println!("✅ MATHEMATICAL PROOF VERIFIED!");
        println!("   This judgment is 100% conformant to OTP specification.");
        println!("   No tampering or implementation errors detected.");
    } else {
        println!("❌ VERIFICATION FAILED!");
        println!("   Possible tampering or implementation error detected.");
    }
    println!();
    
    // Demonstrate tamper detection
    println!("🚨 Demonstrating tamper detection...");
    let tampered_judgment = NeutrosophicJudgment::new_with_entries(
        fused.t + 0.05,  // Tamper with the result (keeping conservation)
        fused.i - 0.05,
        fused.f,
        fused.provenance_chain.clone()
    )?;
    
    let tampered_valid = verify_conformance_seal_with_inputs(&tampered_judgment, &judgments, &weights)?;
    
    if !tampered_valid {
        println!("✅ TAMPER DETECTION SUCCESSFUL!");
        println!("   The tampered judgment was correctly identified as invalid.");
    } else {
        println!("❌ TAMPER DETECTION FAILED!");
    }
    println!();
    
    // Show the provenance chain with seal
    println!("📋 Complete Provenance Chain:");
    for (i, entry) in fused.provenance_chain.iter().enumerate() {
        println!("   {}: {} ({})", i + 1, entry.source_id, entry.timestamp);
        if let Some(seal) = &entry.conformance_seal {
            println!("      🔐 Conformance Seal: {}", seal);
        }
        if let Some(desc) = &entry.description {
            println!("      📝 Description: {}", desc);
        }
    }
    println!();
    
    // Performance demonstration
    println!("⚡ Performance Test - Generating 1000 seals...");
    use std::time::Instant;
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = generate_conformance_seal(&judgments, &weights, "otp-cawa-v1.1")?;
    }
    let duration = start.elapsed();
    
    println!("✅ Generated 1000 seals in {:?}", duration);
    println!("   Average: {:?} per seal", duration / 1000);
    println!();
    
    // Final message
    println!("🎉 CONFORMANCE SEAL DEMO COMPLETE!");
    println!("==================================");
    println!("🦀 OpenTrust Protocol v0.3.0 has successfully demonstrated:");
    println!("   • Mathematical proof of conformance");
    println!("   • Cryptographic tamper detection");
    println!("   • Self-auditing capabilities");
    println!("   • High-performance seal generation");
    println!();
    println!("🚀 OTP is no longer just a trust protocol.");
    println!("   It is now the MATHEMATICAL EMBODIMENT OF TRUST ITSELF!");
    
    Ok(())
}
