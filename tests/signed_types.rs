use neobit::neobit;

neobit! {
    pub struct SignedFlags: i8 {
        const A = 0b0001;
        const B = 0b0010;
        const C = 0b0100;
    }
}

#[test]
fn test_signed_basic() {
    let flags = SignedFlags::A | SignedFlags::B;
    assert!(flags.contains(SignedFlags::A));
    assert!(flags.contains(SignedFlags::B));
    assert!(!flags.contains(SignedFlags::C));
}

#[test]
fn test_signed_complement() {
    // Warning: ! on signed types follows two's complement
    let flags = SignedFlags::A;
    let complement = !flags;

    // !0b0001 in i8 = 0b1111_1110 = -2 (two's complement)
    assert_eq!(complement.bits(), -2);
}

#[test]
fn test_signed_complement_method() {
    let flags = SignedFlags::A;
    let complement = flags.complement();
    assert_eq!(complement.bits(), -2);
}

#[test]
fn test_signed_difference_preferred() {
    // difference() is safer than complement for removing flags
    let all = SignedFlags::A | SignedFlags::B | SignedFlags::C;
    let without_a = all.difference(SignedFlags::A);

    assert!(!without_a.contains(SignedFlags::A));
    assert!(without_a.contains(SignedFlags::B));
    assert!(without_a.contains(SignedFlags::C));
}

#[test]
fn test_signed_from_into() {
    let flags: SignedFlags = 0x7F.into();
    assert_eq!(flags.bits(), 0x7F);

    let bits: i8 = flags.into();
    assert_eq!(bits, 0x7F);
}

#[test]
fn test_signed_negative_bits() {
    // Can represent negative values if needed for FFI
    let flags: SignedFlags = (-1i8).into();
    assert_eq!(flags.bits(), -1);
}

// i16
neobit! {
    pub struct Flags16: i16 {
        const A = 0x0001;
        const B = 0x0002;
    }
}

#[test]
fn test_i16() {
    let flags = Flags16::A | Flags16::B;
    assert_eq!(flags.bits(), 0x0003);
}

// i32 (common for C FFI)
neobit! {
    pub struct CFlags: i32 {
        const OPTION_A = 0x01;
        const OPTION_B = 0x02;
        const OPTION_C = 0x04;
    }
}

#[test]
fn test_i32_c_ffi_style() {
    let flags = CFlags::OPTION_A | CFlags::OPTION_B;
    assert_eq!(flags.bits(), 0x03);

    // Typical C FFI pattern
    fn simulate_c_call(flags: i32) -> i32 {
        flags
    }

    let result = simulate_c_call(flags.into());
    assert_eq!(result, 0x03);
}

#[test]
fn test_signed_debug() {
    let flags = SignedFlags::A | SignedFlags::B;
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "SignedFlags(A | B)");
}

#[test]
fn test_signed_debug_with_unknown() {
    // Note: for signed types, unknown bits display as hex
    let flags: SignedFlags = 0x71.into(); // A + unknown bits
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "SignedFlags(A | 0x70)");
}
