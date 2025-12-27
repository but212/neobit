use neobit::neobit;

neobit! {
    pub struct Flags: u32 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;
        const AB = Self::A.union(Self::B).bits();
        const CD = Self::C.union(Self::D).bits();
    }
}

// Compile-time constants
const CONST_UNION: Flags = Flags::A.union(Flags::B);
const CONST_INTERSECTION: Flags = Flags::AB.intersection(Flags::A);
const CONST_DIFFERENCE: Flags = Flags::all().difference(Flags::AB);
const CONST_SYMMETRIC_DIFF: Flags = Flags::AB.symmetric_difference(Flags::CD);
const CONST_COMPLEMENT: Flags = Flags::A.complement();
const CONST_EMPTY: Flags = Flags::empty();
const CONST_FROM_BITS: Flags = Flags::from_bits_retain(0xFF);

#[test]
fn test_const_union() {
    assert_eq!(CONST_UNION, Flags::AB);
    assert_eq!(CONST_UNION.bits(), 0b0011);
}

#[test]
fn test_const_intersection() {
    assert_eq!(CONST_INTERSECTION, Flags::A);
}

#[test]
fn test_const_difference() {
    assert_eq!(CONST_DIFFERENCE, Flags::CD);
}

#[test]
fn test_const_symmetric_difference() {
    // AB ^ CD = A | B | C | D (since no overlap)
    assert_eq!(CONST_SYMMETRIC_DIFF, Flags::all());
}

#[test]
fn test_const_complement() {
    assert_eq!(CONST_COMPLEMENT.bits(), !1u32);
}

#[test]
fn test_const_empty() {
    assert!(CONST_EMPTY.is_empty());
}

#[test]
fn test_const_from_bits() {
    assert_eq!(CONST_FROM_BITS.bits(), 0xFF);
}

#[test]
fn test_const_checks() {
    // These should all compile (const evaluation)
    const IS_EMPTY: bool = Flags::empty().is_empty();
    const CONTAINS: bool = Flags::AB.contains(Flags::A);
    const INTERSECTS: bool = Flags::AB.intersects(Flags::A);
    const BITS: u32 = Flags::AB.bits();

    assert!(IS_EMPTY);
    assert!(CONTAINS);
    assert!(INTERSECTS);
    assert_eq!(BITS, 0b0011);
}

#[test]
fn test_const_chaining() {
    const COMPLEX: Flags = Flags::A
        .union(Flags::B)
        .union(Flags::C)
        .difference(Flags::B);

    assert_eq!(COMPLEX.bits(), 0b0101); // A | C
    assert!(COMPLEX.contains(Flags::A));
    assert!(!COMPLEX.contains(Flags::B));
    assert!(COMPLEX.contains(Flags::C));
}

#[test]
fn test_const_all() {
    // Test all() in const context
    const ALL_FLAGS: Flags = Flags::all();
    const ALL_BITS: u32 = Flags::all().bits();

    assert_eq!(ALL_FLAGS.bits(), 0b1111);
    assert_eq!(ALL_BITS, 0b1111);
    assert!(ALL_FLAGS.contains(Flags::A));
    assert!(ALL_FLAGS.contains(Flags::B));
    assert!(ALL_FLAGS.contains(Flags::C));
    assert!(ALL_FLAGS.contains(Flags::D));

    // Test that all() contains the manual ALL constant
    assert!(ALL_FLAGS.contains(Flags::AB.union(Flags::CD)));
}
