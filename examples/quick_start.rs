//! Quick Start example from README.md
//!
//! This example demonstrates the basic usage of neobit with file permissions.

use neobit::neobit;

neobit! {
    /// File permissions
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
    }
}

fn main() {
    let perms = Permissions::READ | Permissions::WRITE;

    assert!(perms.contains(Permissions::READ));
    assert!(!perms.contains(Permissions::EXECUTE));

    println!("{:?}", perms); // Permissions(READ | WRITE)

    // Get all flags
    let all = Permissions::all();
    assert!(all.contains(Permissions::READ | Permissions::WRITE | Permissions::EXECUTE));

    // Validate bits safely
    let valid = Permissions::from_bits(0b011);
    assert!(valid.is_some());
    let invalid = Permissions::from_bits(0b1000);
    assert!(invalid.is_none());

    println!("All examples passed!");
}
