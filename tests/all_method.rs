//! Tests for the `all()` method

use neobit::neobit;

neobit! {
    pub struct BasicFlags: u8 {
        const A = 0b001;
        const B = 0b010;
        const C = 0b100;
    }
}

neobit! {
    pub struct SingleFlag: u32 {
        const ONLY = 0x8000_0000;
    }
}

neobit! {
    pub struct EmptyFlags: u16 {
        // No flags defined
    }
}

neobit! {
    pub struct NonContiguous: u64 {
        const FIRST = 0x01;
        const SECOND = 0x04;
        const THIRD = 0x10;
        const FOURTH = 0x40;
        const FIFTH = 0x100;
    }
}

neobit! {
    pub struct SignedFlags: i32 {
        const POSITIVE = 0x01;
        const NEGATIVE_BIT = 0x8000_0000u32 as i32;
    }
}

#[test]
fn test_all_basic() {
    let all = BasicFlags::all();

    // Check that all flags are included
    assert!(all.contains(BasicFlags::A));
    assert!(all.contains(BasicFlags::B));
    assert!(all.contains(BasicFlags::C));

    // Check that it matches the expected value
    assert_eq!(all.bits(), 0b111);
}

#[test]
fn test_all_single_flag() {
    let all = SingleFlag::all();

    assert!(all.contains(SingleFlag::ONLY));
    assert_eq!(all.bits(), 0x8000_0000);
}

#[test]
fn test_all_empty() {
    let all = EmptyFlags::all();

    // Should be empty since no flags are defined
    assert!(all.is_empty());
    assert_eq!(all.bits(), 0);
}

#[test]
fn test_all_non_contiguous() {
    let all = NonContiguous::all();

    // Check all flags are included
    assert!(all.contains(NonContiguous::FIRST));
    assert!(all.contains(NonContiguous::SECOND));
    assert!(all.contains(NonContiguous::THIRD));
    assert!(all.contains(NonContiguous::FOURTH));
    assert!(all.contains(NonContiguous::FIFTH));

    // Check the exact bit pattern
    assert_eq!(all.bits(), 0x01 | 0x04 | 0x10 | 0x40 | 0x100);
}

#[test]
fn test_all_signed() {
    let all = SignedFlags::all();

    assert!(all.contains(SignedFlags::POSITIVE));
    assert!(all.contains(SignedFlags::NEGATIVE_BIT));

    // Note: The negative bit will be interpreted as a negative number in i32
    // but the bit pattern should be correct
    assert_eq!(all.bits(), 0x8000_0001u32 as i32);
}

#[test]
fn test_all_const_context() {
    // Test that all() works in const context
    const ALL_FLAGS: BasicFlags = BasicFlags::all();
    const ALL_BITS: u8 = BasicFlags::all().bits();

    assert!(ALL_FLAGS.contains(BasicFlags::A));
    assert!(ALL_FLAGS.contains(BasicFlags::B));
    assert!(ALL_FLAGS.contains(BasicFlags::C));
    assert_eq!(ALL_BITS, 0b111);
}

#[test]
fn test_all_operations() {
    let all = BasicFlags::all();

    // Test removing from all
    let without_b = all.difference(BasicFlags::B);
    assert!(without_b.contains(BasicFlags::A));
    assert!(without_b.contains(BasicFlags::C));
    assert!(!without_b.contains(BasicFlags::B));

    // Test intersection with all (should return the other flag)
    let a = BasicFlags::A;
    assert_eq!(all.intersection(a), a);

    // Test union with all (should return all)
    let a = BasicFlags::A;
    assert_eq!(all.union(a), all);
}

#[test]
fn test_all_debug_format() {
    let all = BasicFlags::all();
    let debug_str = format!("{:?}", all);

    // Should contain all flag names
    assert!(debug_str.contains("A"));
    assert!(debug_str.contains("B"));
    assert!(debug_str.contains("C"));
}

#[test]
fn test_all_from_into() {
    let all = BasicFlags::all();
    let bits: u8 = all.into();
    let reconstructed: BasicFlags = bits.into();

    assert_eq!(all, reconstructed);
    assert_eq!(bits, 0b111);
}

#[test]
fn test_all_unknown_bits() {
    // Start with all flags
    let all = BasicFlags::all();

    // Add unknown bits
    let with_unknown = all.union(BasicFlags::from_bits_retain(0b1000_0000));

    // Should still contain all original flags
    assert!(with_unknown.contains(BasicFlags::A));
    assert!(with_unknown.contains(BasicFlags::B));
    assert!(with_unknown.contains(BasicFlags::C));

    // But should also have the unknown bits
    assert_eq!(with_unknown.bits(), 0b1000_0111);
}

#[test]
fn test_all_complement() {
    let all = BasicFlags::all();
    let complement = all.complement();

    // Complement should not contain any of the original flags
    assert!(!complement.contains(BasicFlags::A));
    assert!(!complement.contains(BasicFlags::B));
    assert!(!complement.contains(BasicFlags::C));

    // Complement of complement should be original
    assert_eq!(complement.complement(), all);
}

#[test]
fn test_all_is_all() {
    let all = BasicFlags::all();

    // all() should contain all flags
    assert!(all.contains(BasicFlags::A));
    assert!(all.contains(BasicFlags::B));
    assert!(all.contains(BasicFlags::C));

    // all() should contain itself
    assert!(all.contains(all));

    // all() should contain any subset
    assert!(all.contains(BasicFlags::A.union(BasicFlags::B)));
}
