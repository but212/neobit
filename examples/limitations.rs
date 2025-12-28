//! Example demonstrating macro limitations
//!
//! Shows what you can and cannot do inside the neobit! macro.

use neobit::neobit;

// ✅ GOOD: Single-bit constants only
neobit! {
    pub struct Flags: u8 {
        const A = 0b001;     // Single bit - OK
        const B = 0b010;     // Single bit - OK
        const C = 0b100;     // Single bit - OK
    }
}

impl Flags {
    // ✅ GOOD: Composite constants using union()
    pub const AB: Self = Self::A.union(Self::B);
    pub const AC: Self = Self::A.union(Self::C);
    pub const BC: Self = Self::B.union(Self::C);
    pub const ABC: Self = Self::AB.union(Self::C);

    // ✅ GOOD: Complex expressions in impl block
    pub const ALL_EXCEPT_A: Self = Self::all().difference(Self::A);
    pub const MIDDLE_BITS: Self = Self::all().intersection(Self::BC);

    // Additional constants for demonstration
    pub const READ: Self = Self::A; // Rename for clarity
    pub const WRITE: Self = Self::B;
    pub const EXECUTE: Self = Self::C;

    pub const READ_WRITE: Self = Self::READ.union(Self::WRITE);
    pub const WRITE_EXECUTE: Self = Self::WRITE.union(Self::EXECUTE);
    pub const ALL: Self = Self::READ_WRITE.union(Self::EXECUTE);
}

// ❌ BAD: This would fail to compile!
/*
neobit! {
    pub struct BadFlags: u8 {
        const A = 0b001;     // ✅ OK
        const B = 0b010;     // ✅ OK
        const AB = 0b011;    // ❌ Multi-bit constant NOT allowed!
        const COMPLEX = 1 << 2 | 1 << 3;  // ❌ Complex expressions NOT allowed!
    }
}
*/

fn main() {
    println!("=== Using single-bit flags from macro ===");
    let a = Flags::A;
    let b = Flags::B;
    let c = Flags::C;

    println!("A: {:?}", a);
    println!("B: {:?}", b);
    println!("C: {:?}", c);

    println!("\n=== Using composite constants from impl ===");
    println!("AB: {:?}", Flags::AB);
    println!("AC: {:?}", Flags::AC);
    println!("BC: {:?}", Flags::BC);
    println!("ABC: {:?}", Flags::ABC);
    println!("ALL_EXCEPT_A: {:?}", Flags::ALL_EXCEPT_A);
    println!("MIDDLE_BITS: {:?}", Flags::MIDDLE_BITS);

    println!("\n=== Why this limitation exists ===");
    println!("1. Keeps the macro simple and fast to compile");
    println!("2. Avoids bit validation complexity");
    println!("3. Makes it clear which are 'primitive' flags");
    println!("4. Composite constants can still be defined in impl blocks");

    println!("\n=== Workarounds for complex patterns ===");

    // Pattern 1: Use union() for combinations
    let user_perm = Flags::READ.union(Flags::WRITE);
    println!("User permissions: {:?}", user_perm);

    // Pattern 2: Constants are already defined in the impl block above
    println!("READ_WRITE: {:?}", Flags::READ_WRITE);
    println!("WRITE_EXECUTE: {:?}", Flags::WRITE_EXECUTE);
    println!("ALL: {:?}", Flags::ALL);

    println!("\nAll examples passed!");
}
