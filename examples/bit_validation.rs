//! Example demonstrating flexible bit validation
//!
//! Shows both validated and unchecked bit operations.

use neobit::neobit;

neobit! {
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
    }
}

fn main() {
    println!("=== Safe validation - returns None for unknown bits ===");

    // Valid bits
    let flags = Permissions::from_bits(0b011);
    assert!(flags.is_some());
    println!("Valid bits (0b011): {:?}", flags.unwrap());

    // Invalid bits
    let invalid = Permissions::from_bits(0xFF);
    assert!(invalid.is_none());
    println!("Invalid bits (0xFF): None");

    println!("\n=== Unchecked retention - preserves all bits ===");

    // All bits preserved
    let flags = Permissions::from_bits_retain(0xFF);
    println!("All bits preserved (0xFF): {:?}", flags);
    println!("Raw bits: {:#010b}", flags.bits());

    println!("\n=== This is a deliberate trade-off ===");
    println!("neobit provides both options:");
    println!("- from_bits() for safe validation");
    println!("- from_bits_retain() for maximum compatibility");

    println!("\nAll examples passed!");
}
