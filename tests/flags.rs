use neobit::neobit;

// =============================================================================
// Struct Definitions (organized by integer size)
// =============================================================================

// u8 structs
neobit! {
    pub struct Flags8: u8 {
        const A = 0b0001;
        const B = 0b0010;
        const C = 0b0100;
        const D = 0b1000;
    }
}

neobit! {
    pub struct FromIntoFlags: u8 {
        const A = 0b01;
        const B = 0b10;
    }
}

// u16 structs
neobit! {
    pub struct Flags16: u16 {
        const A = 0x0001;
        const B = 0x0100;
    }
}

neobit! {
    pub struct EmptyFlags: u16 {
        // No flags defined
    }
}

// u32 structs
neobit! {
    pub struct Flags32: u32 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;
        const AB = Self::A.union(Self::B).bits();
        const CD = Self::C.union(Self::D).bits();
    }
}

neobit! {
    pub struct SingleFlag: u32 {
        const ONLY = 0x8000_0000;
    }
}

// u64 structs
neobit! {
    pub struct Flags64: u64 {
        const A = 0x01;
        const B = 0x04;
        const C = 0x10;
        const D = 0x40;
        const E = 0x100;
    }
}

// u128 structs
neobit! {
    pub struct Flags128: u128 {
        const A = 0x0000_0000_0000_0001;
        const B = 0x0000_0000_0000_0002;
        const C = 0x0000_0000_0000_0004;
        const D = 0x8000_0000_0000_0000;
        const AB = Self::A.union(Self::B).bits();
    }
}

// i8 structs
neobit! {
    pub struct SignedFlags8: i8 {
        const A = 0b0001;
        const B = 0b0010;
        const C = 0b0100;
    }
}

// i32 structs
neobit! {
    pub struct SignedFlags32: i32 {
        const POSITIVE = 0x01;
        const NEGATIVE_BIT = 0x8000_0000u32 as i32;
    }
}

neobit! {
    pub struct CFlags: i32 {
        const OPTION_A = 0x01;
        const OPTION_B = 0x02;
        const OPTION_C = 0x04;
    }
}

// i128 structs
neobit! {
    pub struct SignedFlags128: i128 {
        const A = 0x0000_0000_0000_0001;
        const B = 0x0000_0000_0000_0002;
        const NEG = -1i128;
    }
}

// =============================================================================
// Flags8 Tests (u8)
// =============================================================================

#[test]
fn test_flags8_basic() {
    let all = Flags8::all();
    assert!(all.contains(Flags8::A));
    assert!(all.contains(Flags8::B));
    assert!(all.contains(Flags8::C));
    assert!(all.contains(Flags8::D));
    assert_eq!(all.bits(), 0b1111);
}

#[test]
fn test_flags8_const_context() {
    const ALL_FLAGS: Flags8 = Flags8::all();
    const ALL_BITS: u8 = Flags8::all().bits();

    assert!(ALL_FLAGS.contains(Flags8::A));
    assert!(ALL_FLAGS.contains(Flags8::B));
    assert!(ALL_FLAGS.contains(Flags8::C));
    assert!(ALL_FLAGS.contains(Flags8::D));
    assert_eq!(ALL_BITS, 0b1111);
}

#[test]
fn test_flags8_operations() {
    let all = Flags8::all();
    let without_b = all.difference(Flags8::B);
    assert!(without_b.contains(Flags8::A));
    assert!(without_b.contains(Flags8::C));
    assert!(!without_b.contains(Flags8::B));

    let a = Flags8::A;
    assert_eq!(all.intersection(a), a);
    assert_eq!(all.union(a), all);
}

#[test]
fn test_flags8_debug() {
    let flags = Flags8::A;
    assert_eq!(format!("{:?}", flags), "Flags8(A)");

    let flags = Flags8::A | Flags8::B;
    assert_eq!(format!("{:?}", flags), "Flags8(A | B)");

    let flags = Flags8::all();
    assert_eq!(format!("{:?}", flags), "Flags8(A | B | C | D)");

    let flags = Flags8::empty();
    assert_eq!(format!("{:?}", flags), "Flags8(empty)");

    let flags = Flags8::from_bits_retain(0b1000_0001);
    assert_eq!(format!("{:?}", flags), "Flags8(A | 0x80)");

    let flags = Flags8::from_bits_retain(0b1000_0000);
    assert_eq!(format!("{:?}", flags), "Flags8(0x80)");
}

#[test]
fn test_flags8_binary_format() {
    let flags = Flags8::A | Flags8::B;

    assert_eq!(format!("{:b}", flags), "11");
    assert_eq!(format!("{:08b}", flags), "00000011");
    assert_eq!(format!("{:#010b}", flags), "0b00000011");
}

#[test]
fn test_flags8_from_into() {
    let all = Flags8::all();
    let bits: u8 = all.into();
    let reconstructed: Flags8 = bits.into();

    assert_eq!(all, reconstructed);
    assert_eq!(bits, 0b1111);
}

#[test]
fn test_flags8_unknown_bits() {
    let all = Flags8::all();
    let with_unknown = all.union(Flags8::from_bits_retain(0b1000_0000));

    assert!(with_unknown.contains(Flags8::A));
    assert!(with_unknown.contains(Flags8::B));
    assert!(with_unknown.contains(Flags8::C));
    assert!(with_unknown.contains(Flags8::D));
    assert_eq!(with_unknown.bits(), 0b1000_1111);
}

#[test]
fn test_flags8_complement() {
    let all = Flags8::all();
    let complement = all.complement();

    assert!(!complement.contains(Flags8::A));
    assert!(!complement.contains(Flags8::B));
    assert!(!complement.contains(Flags8::C));
    assert!(!complement.contains(Flags8::D));
    assert_eq!(complement.complement(), all);
}

#[test]
fn test_flags8_ord() {
    let f1 = Flags8::A;
    let f2 = Flags8::B;
    let f3 = Flags8::A | Flags8::B;

    assert!(f1 < f2);
    assert!(f2 < f3);
    assert!(f1 < f3);
    assert!(f1 <= f1);
    assert!(f1 >= f1);
}

#[test]
fn test_flags8_formatting() {
    let f = Flags8::A | Flags8::D;
    assert_eq!(format!("{:x}", f), "9");
    assert_eq!(format!("{:X}", f), "9");
    assert_eq!(format!("{:o}", f), "11");

    let f_all = Flags8::all();
    assert_eq!(format!("{:x}", f_all), "f");
    assert_eq!(format!("{:X}", f_all), "F");
    assert_eq!(format!("{:o}", f_all), "17");
}

#[test]
fn test_flags8_from_bits_truncate() {
    let input = 0b10111;
    let flags = Flags8::from_bits_truncate(input);

    assert_eq!(flags.bits(), 0b111);
    assert!(flags.contains(Flags8::A));
    assert!(flags.contains(Flags8::B));
    assert!(flags.contains(Flags8::C));
    assert!(!flags.contains(Flags8::D));
}

// =============================================================================
// FromIntoFlags Tests (u8)
// =============================================================================

#[test]
fn test_from_into_flags_from_int() {
    let flags: FromIntoFlags = FromIntoFlags::from(0b11);
    assert_eq!(flags.bits(), 0b11);
    assert!(flags.contains(FromIntoFlags::A));
    assert!(flags.contains(FromIntoFlags::B));
}

#[test]
fn test_from_into_flags_into_int() {
    let flags = FromIntoFlags::A | FromIntoFlags::B;
    let bits: u8 = flags.into();
    assert_eq!(bits, 0b11);
}

#[test]
fn test_from_into_flags_roundtrip() {
    let original: u8 = 0xFF;
    let flags: FromIntoFlags = original.into();
    let back: u8 = flags.into();
    assert_eq!(original, back);
}

#[test]
fn test_from_into_flags_preserves_unknown_bits() {
    let flags: FromIntoFlags = 0xFF.into();
    assert_eq!(flags.bits(), 0xFF);
}

// =============================================================================
// Flags16 Tests (u16)
// =============================================================================

#[test]
fn test_flags16_basic() {
    let flags = Flags16::A | Flags16::B;
    assert_eq!(flags.bits(), 0x0101); // 0x0001 | 0x0100 = 0x0101
}

#[test]
fn test_flags16_debug() {
    let flags = Flags16::A | Flags16::B;
    assert_eq!(format!("{:?}", flags), "Flags16(A | B)");
}

#[test]
fn test_flags16_from_into() {
    let flags: Flags16 = 0xFFFF.into();
    assert_eq!(flags.bits(), 0xFFFF);

    let bits: u16 = flags.into();
    assert_eq!(bits, 0xFFFF);
}

// =============================================================================
// EmptyFlags Tests (u16)
// =============================================================================

#[test]
fn test_empty_flags_all() {
    let all = EmptyFlags::all();
    assert!(all.is_empty());
    assert_eq!(all.bits(), 0);
}

// =============================================================================
// Flags32 Tests (u32)
// =============================================================================

const CONST_UNION: Flags32 = Flags32::A.union(Flags32::B);
const CONST_INTERSECTION: Flags32 = Flags32::AB.intersection(Flags32::A);
const CONST_DIFFERENCE: Flags32 = Flags32::all().difference(Flags32::AB);
const CONST_SYMMETRIC_DIFF: Flags32 = Flags32::AB.symmetric_difference(Flags32::CD);
const CONST_COMPLEMENT: Flags32 = Flags32::A.complement();
const CONST_EMPTY: Flags32 = Flags32::empty();
const CONST_FROM_BITS: Flags32 = Flags32::from_bits_retain(0xFF);

#[test]
fn test_flags32_const_union() {
    assert_eq!(CONST_UNION, Flags32::AB);
    assert_eq!(CONST_UNION.bits(), 0b0011);
}

#[test]
fn test_flags32_const_intersection() {
    assert_eq!(CONST_INTERSECTION, Flags32::A);
}

#[test]
fn test_flags32_const_difference() {
    assert_eq!(CONST_DIFFERENCE, Flags32::CD);
}

#[test]
fn test_flags32_const_symmetric_difference() {
    assert_eq!(CONST_SYMMETRIC_DIFF, Flags32::all());
}

#[test]
fn test_flags32_const_complement() {
    assert_eq!(CONST_COMPLEMENT.bits(), !1u32);
}

#[test]
fn test_flags32_const_empty() {
    assert!(CONST_EMPTY.is_empty());
}

#[test]
fn test_flags32_const_from_bits() {
    assert_eq!(CONST_FROM_BITS.bits(), 0xFF);
}

#[test]
fn test_flags32_const_checks() {
    const IS_EMPTY: bool = Flags32::empty().is_empty();
    const CONTAINS: bool = Flags32::AB.contains(Flags32::A);
    const INTERSECTS: bool = Flags32::AB.intersects(Flags32::A);
    const BITS: u32 = Flags32::AB.bits();

    assert!(IS_EMPTY);
    assert!(CONTAINS);
    assert!(INTERSECTS);
    assert_eq!(BITS, 0b0011);
}

#[test]
fn test_flags32_const_chaining() {
    const COMPLEX: Flags32 = Flags32::A
        .union(Flags32::B)
        .union(Flags32::C)
        .difference(Flags32::B);

    assert_eq!(COMPLEX.bits(), 0b0101);
    assert!(COMPLEX.contains(Flags32::A));
    assert!(!COMPLEX.contains(Flags32::B));
    assert!(COMPLEX.contains(Flags32::C));
}

#[test]
fn test_flags32_const_all() {
    const ALL_FLAGS: Flags32 = Flags32::all();
    const ALL_BITS: u32 = Flags32::all().bits();

    assert_eq!(ALL_FLAGS.bits(), 0b1111);
    assert_eq!(ALL_BITS, 0b1111);
    assert!(ALL_FLAGS.contains(Flags32::A));
    assert!(ALL_FLAGS.contains(Flags32::B));
    assert!(ALL_FLAGS.contains(Flags32::C));
    assert!(ALL_FLAGS.contains(Flags32::D));
    assert!(ALL_FLAGS.contains(Flags32::AB.union(Flags32::CD)));
}

#[test]
fn test_flags32_from_into() {
    let flags: Flags32 = 0xFFFF_FFFF.into();
    assert_eq!(flags.bits(), 0xFFFF_FFFF);
}

#[test]
fn test_flags32_debug() {
    let flags = Flags32::A | Flags32::D;
    assert_eq!(format!("{:?}", flags), "Flags32(A | D)");
}

// =============================================================================
// SingleFlag Tests (u32)
// =============================================================================

#[test]
fn test_single_flag_all() {
    let all = SingleFlag::all();
    assert!(all.contains(SingleFlag::ONLY));
    assert_eq!(all.bits(), 0x8000_0000);
}

// =============================================================================
// Flags64 Tests (u64)
// =============================================================================

#[test]
fn test_flags64_basic() {
    let all = Flags64::all();
    assert!(all.contains(Flags64::A));
    assert!(all.contains(Flags64::B));
    assert!(all.contains(Flags64::C));
    assert!(all.contains(Flags64::D));
    assert!(all.contains(Flags64::E));
    assert_eq!(all.bits(), 0x01 | 0x04 | 0x10 | 0x40 | 0x100);
}

#[test]
fn test_flags64_from_into() {
    let flags: Flags64 = 0xFFFF_FFFF_FFFF_FFFF.into();
    assert_eq!(flags.bits(), 0xFFFF_FFFF_FFFF_FFFF);
}

// =============================================================================
// Flags128 Tests (u128)
// =============================================================================

#[test]
fn test_flags128_basic() {
    let flags = Flags128::A | Flags128::B;
    assert!(flags.contains(Flags128::A));
    assert!(flags.contains(Flags128::B));
    assert!(!flags.contains(Flags128::C));
    assert!(!flags.contains(Flags128::D));
}

#[test]
fn test_flags128_high_bit() {
    let flags = Flags128::D;
    assert_eq!(flags.bits(), 0x8000_0000_0000_0000);

    let combined = Flags128::A | Flags128::D;
    assert_eq!(combined.bits(), 0x8000_0000_0000_0001);
}

#[test]
fn test_flags128_all() {
    let flags = Flags128::all();
    assert!(flags.contains(Flags128::A));
    assert!(flags.contains(Flags128::B));
    assert!(flags.contains(Flags128::C));
    assert!(flags.contains(Flags128::D));
}

#[test]
fn test_flags128_from_into() {
    let flags: Flags128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF.into();
    assert_eq!(flags.bits(), u128::MAX);

    let bits: u128 = flags.into();
    assert_eq!(bits, u128::MAX);
}

#[test]
fn test_flags128_debug() {
    let flags = Flags128::A | Flags128::D;
    assert_eq!(format!("{:?}", flags), "Flags128(A | D)");
}

#[test]
fn test_flags128_complement() {
    let flags = Flags128::A;
    let complement = !flags;
    assert_eq!(complement.bits(), !1u128);
}

// =============================================================================
// SignedFlags8 Tests (i8)
// =============================================================================

#[test]
fn test_signed_flags8_basic() {
    let flags = SignedFlags8::A | SignedFlags8::B;
    assert!(flags.contains(SignedFlags8::A));
    assert!(flags.contains(SignedFlags8::B));
    assert!(!flags.contains(SignedFlags8::C));
}

#[test]
fn test_signed_flags8_complement() {
    let flags = SignedFlags8::A;
    let complement = !flags;
    assert_eq!(complement.bits(), -2);
}

#[test]
fn test_signed_flags8_complement_method() {
    let flags = SignedFlags8::A;
    let complement = flags.complement();
    assert_eq!(complement.bits(), -2);
}

#[test]
fn test_signed_flags8_difference() {
    let all = SignedFlags8::A | SignedFlags8::B | SignedFlags8::C;
    let without_a = all.difference(SignedFlags8::A);

    assert!(!without_a.contains(SignedFlags8::A));
    assert!(without_a.contains(SignedFlags8::B));
    assert!(without_a.contains(SignedFlags8::C));
}

#[test]
fn test_signed_flags8_from_into() {
    let flags: SignedFlags8 = 0x7F.into();
    assert_eq!(flags.bits(), 0x7F);

    let bits: i8 = flags.into();
    assert_eq!(bits, 0x7F);
}

#[test]
fn test_signed_flags8_negative_bits() {
    let flags: SignedFlags8 = (-1i8).into();
    assert_eq!(flags.bits(), -1);
}

#[test]
fn test_signed_flags8_debug() {
    let flags = SignedFlags8::A | SignedFlags8::B;
    assert_eq!(format!("{:?}", flags), "SignedFlags8(A | B)");
}

#[test]
fn test_signed_flags8_debug_with_unknown() {
    let flags: SignedFlags8 = 0x71.into();
    assert_eq!(format!("{:?}", flags), "SignedFlags8(A | 0x70)");
}

// =============================================================================
// SignedFlags32 Tests (i32)
// =============================================================================

#[test]
fn test_signed_flags32_all() {
    let all = SignedFlags32::all();

    assert!(all.contains(SignedFlags32::POSITIVE));
    assert!(all.contains(SignedFlags32::NEGATIVE_BIT));
    assert_eq!(all.bits(), 0x8000_0001u32 as i32);
}

// =============================================================================
// CFlags Tests (i32 - C FFI style)
// =============================================================================

#[test]
fn test_cflags_c_ffi_style() {
    let flags = CFlags::OPTION_A | CFlags::OPTION_B;
    assert_eq!(flags.bits(), 0x03);

    fn simulate_c_call(flags: i32) -> i32 {
        flags
    }

    let result = simulate_c_call(flags.into());
    assert_eq!(result, 0x03);
}

// =============================================================================
// SignedFlags128 Tests (i128)
// =============================================================================

#[test]
fn test_signed_flags128_basic() {
    let flags = SignedFlags128::A | SignedFlags128::B;
    assert!(flags.contains(SignedFlags128::A));
    assert!(flags.contains(SignedFlags128::B));
}

#[test]
fn test_signed_flags128_negative() {
    let flags = SignedFlags128::NEG;
    assert_eq!(flags.bits(), -1i128);
}

#[test]
fn test_signed_flags128_from_into() {
    let flags: SignedFlags128 = 0x7FFF_FFFF_FFFF_FFFF.into();
    assert_eq!(flags.bits(), 0x7FFF_FFFF_FFFF_FFFF);

    let negative: SignedFlags128 = (-1i128).into();
    assert_eq!(negative.bits(), -1i128);
}

#[test]
fn test_signed_flags128_debug() {
    // SignedFlags128::NEG is -1i128 (all bits set), so ORing with A still has all bits set.
    // This means the result contains both A and B flags, plus all remaining bits as unknown.
    let flags = SignedFlags128::A | SignedFlags128::NEG;
    assert_eq!(
        format!("{:?}", flags),
        "SignedFlags128(A | B | 0xfffffffffffffffffffffffffffffffc)"
    );
}

#[test]
fn test_signed_flags128_complement() {
    let flags = SignedFlags128::A;
    let complement = !flags;
    assert_eq!(complement.bits(), -2i128);
}
