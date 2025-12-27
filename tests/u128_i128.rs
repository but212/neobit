use neobit::neobit;

neobit! {
    pub struct Flags128: u128 {
        const A = 0x0000_0000_0000_0001;
        const B = 0x0000_0000_0000_0002;
        const C = 0x0000_0000_0000_0004;
        const D = 0x8000_0000_0000_0000; // 최상위 비트
        const AB = Self::A.union(Self::B).bits();
    }
}

#[test]
fn test_u128_basic() {
    let flags = Flags128::A | Flags128::B;
    assert!(flags.contains(Flags128::A));
    assert!(flags.contains(Flags128::B));
    assert!(!flags.contains(Flags128::C));
    assert!(!flags.contains(Flags128::D));
}

#[test]
fn test_u128_high_bit() {
    let flags = Flags128::D;
    assert_eq!(flags.bits(), 0x8000_0000_0000_0000);

    let combined = Flags128::A | Flags128::D;
    assert_eq!(combined.bits(), 0x8000_0000_0000_0001);
}

#[test]
fn test_u128_all_bits() {
    let flags = Flags128::all();
    assert!(flags.contains(Flags128::A));
    assert!(flags.contains(Flags128::B));
    assert!(flags.contains(Flags128::C));
    assert!(flags.contains(Flags128::D));
}

#[test]
fn test_u128_from_into() {
    let flags: Flags128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF.into();
    assert_eq!(flags.bits(), u128::MAX);

    let bits: u128 = flags.into();
    assert_eq!(bits, u128::MAX);
}

#[test]
fn test_u128_debug() {
    let flags = Flags128::A | Flags128::D;
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "Flags128(A | D)");
}

#[test]
fn test_u128_complement() {
    let flags = Flags128::A;
    let complement = !flags;
    assert_eq!(complement.bits(), !1u128);
}

// i128 테스트
neobit! {
    pub struct SignedFlags128: i128 {
        const A = 0x0000_0000_0000_0001;
        const B = 0x0000_0000_0000_0002;
        const NEG = -1i128;
    }
}

#[test]
fn test_i128_basic() {
    let flags = SignedFlags128::A | SignedFlags128::B;
    assert!(flags.contains(SignedFlags128::A));
    assert!(flags.contains(SignedFlags128::B));
}

#[test]
fn test_i128_negative() {
    let flags = SignedFlags128::NEG;
    assert_eq!(flags.bits(), -1i128);
}

#[test]
fn test_i128_from_into() {
    let flags: SignedFlags128 = 0x7FFF_FFFF_FFFF_FFFF.into();
    assert_eq!(flags.bits(), 0x7FFF_FFFF_FFFF_FFFF);

    let negative: SignedFlags128 = (-1i128).into();
    assert_eq!(negative.bits(), -1i128);
}

#[test]
fn test_i128_debug() {
    let flags = SignedFlags128::A | SignedFlags128::NEG;
    let debug = format!("{:?}", flags);
    assert_eq!(
        debug,
        "SignedFlags128(A | B | 0xfffffffffffffffffffffffffffffffc)"
    );
}

#[test]
fn test_i128_complement() {
    let flags = SignedFlags128::A;
    let complement = !flags;
    // i128에서 !0b1 = -2 (2의 보수)
    assert_eq!(complement.bits(), -2i128);
}
