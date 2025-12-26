use neobit::neobit;

neobit! {
    pub struct Flags: u8 {
        const A = 0b01;
        const B = 0b10;
    }
}

#[test]
fn test_from_int() {
    let flags: Flags = Flags::from(0b11);
    assert_eq!(flags.bits(), 0b11);
    assert!(flags.contains(Flags::A));
    assert!(flags.contains(Flags::B));
}

#[test]
fn test_into_int() {
    let flags = Flags::A | Flags::B;
    let bits: u8 = flags.into();
    assert_eq!(bits, 0b11);
}

#[test]
fn test_from_into_roundtrip() {
    let original: u8 = 0xFF;
    let flags: Flags = original.into();
    let back: u8 = flags.into();
    assert_eq!(original, back);
}

#[test]
fn test_from_preserves_unknown_bits() {
    let flags: Flags = 0xFF.into();
    assert_eq!(flags.bits(), 0xFF);
}

// Test with different integer types
neobit! {
    pub struct Flags16: u16 {
        const A = 1;
    }
}

#[test]
fn test_from_u16() {
    let flags: Flags16 = 0xFFFF.into();
    assert_eq!(flags.bits(), 0xFFFF);

    let bits: u16 = flags.into();
    assert_eq!(bits, 0xFFFF);
}

neobit! {
    pub struct Flags32: u32 {
        const A = 1;
    }
}

#[test]
fn test_from_u32() {
    let flags: Flags32 = 0xFFFF_FFFF.into();
    assert_eq!(flags.bits(), 0xFFFF_FFFF);
}

neobit! {
    pub struct Flags64: u64 {
        const A = 1;
    }
}

#[test]
fn test_from_u64() {
    let flags: Flags64 = 0xFFFF_FFFF_FFFF_FFFF.into();
    assert_eq!(flags.bits(), 0xFFFF_FFFF_FFFF_FFFF);
}
