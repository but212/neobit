//! Example demonstrating type conversion
//!
//! Shows how to convert between integer types and flag types.

use neobit::neobit;

neobit! {
    pub struct Flags: u8 {
        const A = 0b001;
        const B = 0b010;
        const C = 0b100;
    }
}

fn main() {
    println!("=== From/Into traits ===");

    // From integer to flags
    let flags1: Flags = 0b011.into();
    println!("0b011.into(): {:?}", flags1);

    // From flags to integer
    let bits1: u8 = flags1.into();
    println!("flags.into(): {}", bits1);

    println!("\n=== Explicit methods ===");

    // From bits (validated)
    let flags2 = Flags::from_bits(0b011);
    match flags2 {
        Some(f) => println!("from_bits(0b011): {:?}", f),
        None => println!("from_bits(0b011): None (invalid bits)"),
    }

    let invalid = Flags::from_bits(0b1000);
    match invalid {
        Some(f) => println!("from_bits(0b1000): {:?}", f),
        None => println!("from_bits(0b1000): None (invalid bits)"),
    }

    // From bits retain (unchecked)
    let flags3 = Flags::from_bits_retain(0b1011);
    println!("from_bits_retain(0b1011): {:?}", flags3);
    println!("Raw bits: {:#010b}", flags3.bits());

    // Get bits
    let bits2 = flags3.bits();
    println!("bits(): {:#010b}", bits2);

    println!("\n=== Practical usage ===");

    // Parsing from a string
    let input = "3"; // 0b011
    let parsed: u8 = input.parse().expect("Invalid number");
    let flags = Flags::from_bits(parsed);

    match flags {
        Some(f) => {
            println!("Parsed '{}' as {:?}", input, f);
            if f.contains(Flags::A) {
                println!("  - Flag A is set");
            }
            if f.contains(Flags::B) {
                println!("  - Flag B is set");
            }
            if f.contains(Flags::C) {
                println!("  - Flag C is set");
            }
        }
        None => println!("'{}' contains invalid flag bits", input),
    }

    // Serializing to integer
    let flags = Flags::A | Flags::C;
    let serialized = flags.bits();
    println!("Serialized {:?} to integer: {}", flags, serialized);

    println!("\nAll examples passed!");
}
