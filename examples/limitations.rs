//! Example demonstrating macro limitations
//!
//! Shows what you can and cannot do inside the neobit! macro.

use neobit::neobit;

// Allowed forms inside the macro
neobit! {
    pub struct Flags: u8 {
        const A = 0b001;     // Single bit - OK
        const B = 0b010;     // Single bit - OK
        const C = 0b100;     // Single bit - OK

        // Composite constant using union() requires .bits() because macro constants expect raw integers
        const AB = Self::A.union(Self::B).bits();

        // Multi-bit field values / bitwise expressions are allowed as long as they evaluate to integer type
        const BR_16 = 0b011 << 3;
    }
}

impl Flags {
    // Composite constants in impl block returning Self type (cleaner syntax)
    pub const AC: Self = Self::A.union(Self::C);
    pub const BC: Self = Self::B.union(Self::C);
    pub const ABC: Self = Self::AB.union(Self::C);

    // Complex expressions in impl block
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

// Invalid form inside macro (fails to compile because expression returns Flags, not raw u8 integer type)
/*
neobit! {
    pub struct BadFlags: u8 {
        const A = 0b001;
        const B = 0b010;
        const AB = Self::A.union(Self::B); // Fails: expects integer type (needs .bits())
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

    println!("\n=== Macro constant requirements and Debug behavior ===");
    println!("1. Macro flag constants must evaluate to the raw integer type of the struct.");
    println!("2. Expressions producing Self (like Self::A.union(Self::B)) require .bits().");
    println!("3. Multi-bit constants (e.g. 0b011 << 3) compile and work correctly.");
    println!("4. Debug decomposes values into single-bit flag names, so multi-bit constants are shown as combinations of overlapping single-bit flag names.");

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
