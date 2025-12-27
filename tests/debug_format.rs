use neobit::neobit;

neobit! {
    pub struct Flags: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
    }
}

#[test]
fn test_debug_single_flag() {
    let flags = Flags::READ;
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "Flags(READ)");
}

#[test]
fn test_debug_multiple_flags() {
    let flags = Flags::READ | Flags::WRITE;
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "Flags(READ | WRITE)");
}

#[test]
fn test_debug_all_flags() {
    let flags = Flags::READ | Flags::WRITE | Flags::EXECUTE;
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "Flags(READ | WRITE | EXECUTE)");
}

#[test]
fn test_debug_composite_constant() {
    // Composite constants are expanded to their single-bit components
    let debug = format!("{:?}", Flags::all());
    assert_eq!(debug, "Flags(READ | WRITE | EXECUTE)");

    let rw = Flags::READ.union(Flags::WRITE);
    let debug = format!("{:?}", rw);
    assert_eq!(debug, "Flags(READ | WRITE)");
}

#[test]
fn test_debug_empty() {
    let flags = Flags::empty();
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "Flags(empty)");
}

#[test]
fn test_debug_unknown_bits() {
    let flags = Flags::from_bits_retain(0b1000_0001);
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "Flags(READ | 0x80)");
}

#[test]
fn test_debug_only_unknown_bits() {
    let flags = Flags::from_bits_retain(0b1000_0000);
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "Flags(0x80)");
}

#[test]
fn test_binary_format() {
    let flags = Flags::READ | Flags::WRITE;

    let binary = format!("{:b}", flags);
    assert_eq!(binary, "11");

    let binary_padded = format!("{:08b}", flags);
    assert_eq!(binary_padded, "00000011");

    let binary_prefixed = format!("{:#010b}", flags);
    assert_eq!(binary_prefixed, "0b00000011");
}

// Test with different integer types
neobit! {
    pub struct Flags16: u16 {
        const A = 0x0001;
        const B = 0x0100;
    }
}

#[test]
fn test_debug_u16() {
    let flags = Flags16::A | Flags16::B;
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "Flags16(A | B)");
}

neobit! {
    pub struct Flags32: u32 {
        const LOW  = 0x0000_0001;
        const HIGH = 0x8000_0000;
    }
}

#[test]
fn test_debug_u32() {
    let flags = Flags32::LOW | Flags32::HIGH;
    let debug = format!("{:?}", flags);
    assert_eq!(debug, "Flags32(LOW | HIGH)");
}
