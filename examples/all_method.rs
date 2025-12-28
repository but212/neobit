//! Example demonstrating the `all()` method
//!
//! Shows how to get the union of all defined flags without manual constants.

use neobit::neobit;

neobit! {
    pub struct Flags: u8 {
        const A = 0b001;
        const B = 0b010;
        const C = 0b100;
    }
}

fn main() {
    // No need for manual ALL constants!
    let all = Flags::all(); // Contains A | B | C

    println!("All flags: {:?}", all);
    assert!(all.contains(Flags::A));
    assert!(all.contains(Flags::B));
    assert!(all.contains(Flags::C));

    // Works in const context too
    const ALL_FLAGS: Flags = Flags::all();
    println!("All flags (const): {:?}", ALL_FLAGS);

    // Benefits:
    // - Less boilerplate - No need to manually define ALL constants
    // - Always in sync - Automatically includes all flags, even when new ones are added
    // - Const-compatible - Can be used in compile-time expressions

    println!("All examples passed!");
}
