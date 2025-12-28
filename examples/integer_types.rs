//! Multiple integer types example.
//!
//! Demonstrates neobit support for different integer types (u8-u128, i8-i128).

use neobit::neobit;

neobit! {
    /// Small flags (u8)
    pub struct SmallFlags: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;
        const E = 1 << 4;
        const F = 1 << 5;
        const G = 1 << 6;
        const H = 1 << 7;
    }
}

neobit! {
    /// Medium flags (u32)
    pub struct MediumFlags: u32 {
        const FLAG_0  = 1 << 0;
        const FLAG_10 = 1 << 10;
        const FLAG_20 = 1 << 20;
        const FLAG_30 = 1 << 30;
    }
}

neobit! {
    /// Large flags (u64) - for feature flags with many options
    pub struct LargeFlags: u64 {
        const FEATURE_1  = 1 << 0;
        const FEATURE_32 = 1 << 32;
        const FEATURE_48 = 1 << 48;
        const FEATURE_63 = 1 << 63;
    }
}

neobit! {
    /// Huge flags (u128) - for cryptographic or specialized uses
    pub struct HugeFlags: u128 {
        const LOW      = 1 << 0;
        const MID_LOW  = 1 << 32;
        const MID_HIGH = 1 << 64;
        const HIGH     = 1 << 96;
    }
}

neobit! {
    /// Signed flags (i32) - for C FFI compatibility
    pub struct SignedFlags: i32 {
        const POS_1 = 1 << 0;
        const POS_2 = 1 << 1;
        const POS_3 = 1 << 2;
    }
}

neobit! {
    /// Different sized types for different use cases
    pub struct CompactFlags: u16 {
        const ENABLED     = 1 << 0;
        const COMPRESSED  = 1 << 1;
        const ENCRYPTED   = 1 << 2;
        const VERIFIED    = 1 << 3;
    }
}

fn print_type_info<T>() {
    println!("  Size: {} bytes", std::mem::size_of::<T>());
    println!("  Alignment: {} bytes", std::mem::align_of::<T>());
}

fn main() {
    println!("=== Size Comparison ===\n");

    println!("SmallFlags (u8):");
    print_type_info::<SmallFlags>();

    println!("\nCompactFlags (u16):");
    print_type_info::<CompactFlags>();

    println!("\nMediumFlags (u32):");
    print_type_info::<MediumFlags>();

    println!("\nLargeFlags (u64):");
    print_type_info::<LargeFlags>();

    println!("\nHugeFlags (u128):");
    print_type_info::<HugeFlags>();

    println!("\n=== u8 Flags (8 bits) ===\n");
    let small = SmallFlags::A | SmallFlags::C | SmallFlags::H;
    println!("Flags: {:?}", small);
    println!("Binary: {:08b}", small.bits());
    println!("All possible: {:?}", SmallFlags::all());

    println!("\n=== u16 Flags (16 bits) ===\n");
    let compact = CompactFlags::ENABLED | CompactFlags::ENCRYPTED;
    println!("Flags: {:?}", compact);
    println!("Binary: {:016b}", compact.bits());
    println!("Hex: {:#06x}", compact.bits());

    println!("\n=== u32 Flags (32 bits) ===\n");
    let medium = MediumFlags::FLAG_0 | MediumFlags::FLAG_20 | MediumFlags::FLAG_30;
    println!("Flags: {:?}", medium);
    println!("Binary: {:032b}", medium.bits());
    println!("Hex: {:#010x}", medium.bits());

    println!("\n=== u64 Flags (64 bits) ===\n");
    let large = LargeFlags::FEATURE_1 | LargeFlags::FEATURE_32 | LargeFlags::FEATURE_63;
    println!("Flags: {:?}", large);
    println!("Hex: {:#018x}", large.bits());
    println!(
        "Can use bits beyond u32: {}",
        large.contains(LargeFlags::FEATURE_63)
    );

    println!("\n=== u128 Flags (128 bits) ===\n");
    let huge = HugeFlags::LOW | HugeFlags::MID_HIGH | HugeFlags::HIGH;
    println!("Flags: {:?}", huge);
    println!("Hex: {:#034x}", huge.bits());
    println!("Decimal: {}", huge.bits());

    println!("\n=== Signed Integer Support (i32) ===\n");
    let signed = SignedFlags::POS_1 | SignedFlags::POS_2;
    println!("Flags: {:?}", signed);
    println!("As i32: {}", signed.bits());
    println!("Binary: {:032b}", signed.bits() as u32);

    // Warning about complement with signed types
    println!("\n=== Signed Type Complement Behavior ===\n");
    let pos = SignedFlags::POS_1;
    println!("Original: {:?} ({})", pos, pos.bits());

    let comp = pos.complement();
    println!("Complement: {:?} ({})", comp, comp.bits());
    println!("Warning: Complement with signed types uses two's complement!");

    // Safer alternative for signed types
    let all_signed = SignedFlags::all();
    let diff = all_signed.difference(pos);
    println!("Difference (safer): {:?} ({})", diff, diff.bits());

    println!("\n=== Memory Efficiency ===\n");

    // Use the smallest type that fits your needs
    println!(
        "For ≤8 flags:   use u8  ({} bytes)",
        std::mem::size_of::<u8>()
    );
    println!(
        "For ≤16 flags:  use u16 ({} bytes)",
        std::mem::size_of::<u16>()
    );
    println!(
        "For ≤32 flags:  use u32 ({} bytes)",
        std::mem::size_of::<u32>()
    );
    println!(
        "For ≤64 flags:  use u64 ({} bytes)",
        std::mem::size_of::<u64>()
    );
    println!(
        "For ≤128 flags: use u128 ({} bytes)",
        std::mem::size_of::<u128>()
    );

    println!("\n=== Type Conversion ===\n");

    let compact_bits: u16 = compact.into();
    println!("u16 value: {}", compact_bits);

    let large_bits: u64 = large.into();
    println!("u64 value: {}", large_bits);

    let huge_bits: u128 = huge.into();
    println!("u128 value: {}", huge_bits);

    println!("\n=== const fn Support Across Types ===\n");

    const SMALL_COMBO: SmallFlags = SmallFlags::A.union(SmallFlags::B);
    const MEDIUM_COMBO: MediumFlags = MediumFlags::FLAG_0.union(MediumFlags::FLAG_10);
    const LARGE_COMBO: LargeFlags = LargeFlags::FEATURE_1.union(LargeFlags::FEATURE_32);

    println!("Small combo (const): {:?}", SMALL_COMBO);
    println!("Medium combo (const): {:?}", MEDIUM_COMBO);
    println!("Large combo (const): {:?}", LARGE_COMBO);

    println!("\nAll const operations work with any integer type! ✓");
}
