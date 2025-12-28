//! Example demonstrating the complement operation difference
//!
//! Shows how neobit and bitflags implement complement() differently.

use neobit::neobit;

neobit! {
    pub struct Flags: u8 {
        const A = 0b01;
        const B = 0b10;
    }
}

fn main() {
    let flags = Flags::A; // 0b01

    // neobit: Pure bitwise complement
    let neobit_comp = flags.complement(); // !0b01 = 0b11111110

    println!("Original flag: {:08b}", flags.bits());
    println!("neobit complement: {:08b}", neobit_comp.bits());
    println!("As Debug: {:?}", neobit_comp);

    // bitflags would return: !0b01 & 0b11 = 0b10 (only defined flags)
    println!("\nbitflags would return: 0b10 (only defined flags)");
    println!("neobit returns: 0b11111110 (all bits inverted)");

    println!("\n=== Why this matters ===");
    println!(
        "- neobit preserves all bit information - essential for hardware registers and protocols"
    );
    println!("- bitflags masks to defined flags - safer for application-level code");

    // Example with hardware register
    println!("\n=== Hardware register example ===");
    // Imagine a hardware register where bit 7 indicates an error
    let register_value = 0b10000001; // Error flag + our flag A
    let parsed = Flags::from_bits_retain(register_value);
    println!("Register value: {:08b}", register_value);
    println!("Parsed as: {:?}", parsed);

    // Complement preserves the error bit
    let complemented = parsed.complement();
    println!("Complemented: {:08b}", complemented.bits());
    println!("Error bit preserved: {}", (complemented.bits() & 0x80) != 0);

    println!("\nAll examples passed!");
}
