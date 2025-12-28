use neobit::neobit;

neobit! {
    pub struct Flags: u8 {
        const A = 0b0001;
        const B = 0b0010;
        const C = 0b0100;
        const D = 0b1000;
    }
}

#[test]
fn test_ord_partial_ord() {
    let f1 = Flags::A;
    let f2 = Flags::B;
    let f3 = Flags::A | Flags::B;

    assert!(f1 < f2); // 1 < 2
    assert!(f2 < f3); // 2 < 3
    assert!(f1 < f3); // 1 < 3
    assert!(f1 <= f1);
    assert!(f1 >= f1);
}

#[test]
fn test_formatting() {
    let f = Flags::A | Flags::D; // 1 | 8 = 9
    assert_eq!(format!("{:x}", f), "9");
    assert_eq!(format!("{:X}", f), "9");
    assert_eq!(format!("{:o}", f), "11");

    let f_all = Flags::all(); // 15 (0xF)
    assert_eq!(format!("{:x}", f_all), "f");
    assert_eq!(format!("{:X}", f_all), "F");
    assert_eq!(format!("{:o}", f_all), "17");
}

#[test]
fn test_from_bits_truncate() {
    // defined: 1, 2, 4, 8 -> mask 15 (0xF)
    // input: 0b10111 (23) -> 16(unset) + 4(C) + 2(B) + 1(A)
    // truncated: 0b00111 (7) -> C | B | A
    let input = 0b10111;
    let flags = Flags::from_bits_truncate(input);

    assert_eq!(flags.bits(), 0b111);
    assert!(flags.contains(Flags::A));
    assert!(flags.contains(Flags::B));
    assert!(flags.contains(Flags::C));
    assert!(!flags.contains(Flags::D));
}
