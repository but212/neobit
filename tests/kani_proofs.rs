//! Kani formal verification proofs for neobit.
//!
//! These proof harnesses verify that all bitwise operations are panic-free
//! and mathematically correct across all possible input combinations.
//!
//! Run with: `cargo kani --tests`

#![allow(unused)]

use neobit::neobit;

// Test flags for verification
neobit! {
    /// Flags used for Kani verification proofs
    pub struct TestFlags: u8 {
        const A = 0b0001;
        const B = 0b0010;
        const C = 0b0100;
        const D = 0b1000;
    }
}

/// Verify that union operation never panics for any bit combination.
#[cfg(kani)]
#[kani::proof]
fn proof_union_no_panic() {
    let a: u8 = kani::any();
    let b: u8 = kani::any();

    let flags_a = TestFlags::from_bits_retain(a);
    let flags_b = TestFlags::from_bits_retain(b);

    // Union should never panic
    let result = flags_a.union(flags_b);

    // Verify bitwise OR semantics
    assert_eq!(result.bits(), a | b);
}

/// Verify that intersection operation never panics for any bit combination.
#[cfg(kani)]
#[kani::proof]
fn proof_intersection_no_panic() {
    let a: u8 = kani::any();
    let b: u8 = kani::any();

    let flags_a = TestFlags::from_bits_retain(a);
    let flags_b = TestFlags::from_bits_retain(b);

    // Intersection should never panic
    let result = flags_a.intersection(flags_b);

    // Verify bitwise AND semantics
    assert_eq!(result.bits(), a & b);
}

/// Verify that difference operation never panics for any bit combination.
#[cfg(kani)]
#[kani::proof]
fn proof_difference_no_panic() {
    let a: u8 = kani::any();
    let b: u8 = kani::any();

    let flags_a = TestFlags::from_bits_retain(a);
    let flags_b = TestFlags::from_bits_retain(b);

    // Difference should never panic
    let result = flags_a.difference(flags_b);

    // Verify bitwise AND NOT semantics
    assert_eq!(result.bits(), a & !b);
}

/// Verify that complement operation never panics for any bit combination.
#[cfg(kani)]
#[kani::proof]
fn proof_complement_no_panic() {
    let a: u8 = kani::any();

    let flags = TestFlags::from_bits_retain(a);

    // Complement should never panic
    let result = flags.complement();

    // Verify bitwise NOT semantics
    assert_eq!(result.bits(), !a);
}

/// Verify that symmetric_difference operation never panics for any bit combination.
#[cfg(kani)]
#[kani::proof]
fn proof_symmetric_difference_no_panic() {
    let a: u8 = kani::any();
    let b: u8 = kani::any();

    let flags_a = TestFlags::from_bits_retain(a);
    let flags_b = TestFlags::from_bits_retain(b);

    // Symmetric difference should never panic
    let result = flags_a.symmetric_difference(flags_b);

    // Verify bitwise XOR semantics
    assert_eq!(result.bits(), a ^ b);
}

/// Verify that from_bits correctly validates bit combinations.
///
/// Property: from_bits returns Some if and only if all bits are within defined flags.
#[cfg(kani)]
#[kani::proof]
fn proof_from_bits_soundness() {
    let bits: u8 = kani::any();
    let all_flags = TestFlags::all().bits();

    let result = TestFlags::from_bits(bits);

    // If bits are valid (no unknown bits set), result should be Some
    if (bits & !all_flags) == 0 {
        assert!(result.is_some());
        assert_eq!(result.unwrap().bits(), bits);
    } else {
        // If any unknown bit is set, result should be None
        assert!(result.is_none());
    }

    // Cover both branches
    kani::cover!(result.is_some(), "from_bits returns Some for valid bits");
    kani::cover!(result.is_none(), "from_bits returns None for invalid bits");
}

/// Verify that from_bits_truncate always produces valid flags.
///
/// Property: The result only contains bits that are in all().
#[cfg(kani)]
#[kani::proof]
fn proof_from_bits_truncate_soundness() {
    let bits: u8 = kani::any();
    let all_flags = TestFlags::all().bits();

    let result = TestFlags::from_bits_truncate(bits);

    // Result should only have valid bits (masked with all flags)
    assert_eq!(result.bits(), bits & all_flags);

    // Result should always be convertible back via from_bits
    assert!(TestFlags::from_bits(result.bits()).is_some());
}

/// Verify that contains correctly checks flag membership.
///
/// Property: contains(other) is true iff (self & other) == other.
#[cfg(kani)]
#[kani::proof]
fn proof_contains_correctness() {
    let a: u8 = kani::any();
    let b: u8 = kani::any();

    let flags_a = TestFlags::from_bits_retain(a);
    let flags_b = TestFlags::from_bits_retain(b);

    let contains_result = flags_a.contains(flags_b);

    // Verify contains semantics
    assert_eq!(contains_result, (a & b) == b);

    // Cover both true and false cases
    kani::cover!(contains_result, "contains returns true");
    kani::cover!(!contains_result, "contains returns false");
}

/// Verify that set operation never panics and produces correct results.
#[cfg(kani)]
#[kani::proof]
fn proof_set_no_panic() {
    let initial: u8 = kani::any();
    let other: u8 = kani::any();
    let condition: bool = kani::any();

    let mut flags = TestFlags::from_bits_retain(initial);
    let other_flags = TestFlags::from_bits_retain(other);

    // Set should never panic
    flags.set(other_flags, condition);

    // Verify set semantics
    if condition {
        // If condition is true, other bits should be added
        assert!((flags.bits() & other) == other);
    } else {
        // If condition is false, other bits should be removed
        assert!((flags.bits() & other) == 0);
    }
}

/// Verify that From<u8> and From<TestFlags> are inverses.
///
/// Property: Converting from u8 to Flags and back always preserves the value.
#[cfg(kani)]
#[kani::proof]
fn proof_roundtrip_conversion() {
    let bits: u8 = kani::any();

    // u8 -> Flags -> u8 should preserve value
    let flags: TestFlags = bits.into();
    let result: u8 = flags.into();

    assert_eq!(result, bits);
}

/// Verify intersects correctly checks for any common bits.
///
/// Property: intersects(other) is true iff (self & other) != 0.
#[cfg(kani)]
#[kani::proof]
fn proof_intersects_correctness() {
    let a: u8 = kani::any();
    let b: u8 = kani::any();

    let flags_a = TestFlags::from_bits_retain(a);
    let flags_b = TestFlags::from_bits_retain(b);

    let intersects_result = flags_a.intersects(flags_b);

    // Verify intersects semantics
    assert_eq!(intersects_result, (a & b) != 0);

    // Cover edge cases
    kani::cover!(intersects_result, "intersects returns true");
    kani::cover!(!intersects_result, "intersects returns false");
}

/// Verify is_empty and is_all are correct.
#[cfg(kani)]
#[kani::proof]
fn proof_empty_all_correctness() {
    let bits: u8 = kani::any();
    let flags = TestFlags::from_bits_retain(bits);
    let all_flags = TestFlags::all().bits();

    // is_empty is true iff bits == 0
    assert_eq!(flags.is_empty(), bits == 0);

    // is_all is true iff bits == all_flags
    assert_eq!(flags.is_all(), bits == all_flags);

    // Cover all cases
    kani::cover!(flags.is_empty(), "is_empty returns true");
    kani::cover!(flags.is_all(), "is_all returns true");
    kani::cover!(
        !flags.is_empty() && !flags.is_all(),
        "neither empty nor all"
    );
}

/// Verify algebraic properties of bitwise operations.
///
/// Property: Union is commutative: a | b == b | a
/// Property: Intersection is commutative: a & b == b & a
/// Property: Symmetric difference is commutative: a ^ b == b ^ a
#[cfg(kani)]
#[kani::proof]
fn proof_commutative_properties() {
    let a: u8 = kani::any();
    let b: u8 = kani::any();

    let flags_a = TestFlags::from_bits_retain(a);
    let flags_b = TestFlags::from_bits_retain(b);

    // Union is commutative
    assert_eq!(flags_a.union(flags_b).bits(), flags_b.union(flags_a).bits());

    // Intersection is commutative
    assert_eq!(
        flags_a.intersection(flags_b).bits(),
        flags_b.intersection(flags_a).bits()
    );

    // Symmetric difference is commutative
    assert_eq!(
        flags_a.symmetric_difference(flags_b).bits(),
        flags_b.symmetric_difference(flags_a).bits()
    );
}

/// Verify De Morgan's laws hold for complement operations.
///
/// Property: !(a | b) == !a & !b
/// Property: !(a & b) == !a | !b
#[cfg(kani)]
#[kani::proof]
fn proof_de_morgan_laws() {
    let a: u8 = kani::any();
    let b: u8 = kani::any();

    let flags_a = TestFlags::from_bits_retain(a);
    let flags_b = TestFlags::from_bits_retain(b);

    // !(a | b) == !a & !b
    let lhs1 = flags_a.union(flags_b).complement();
    let rhs1 = flags_a.complement().intersection(flags_b.complement());
    assert_eq!(lhs1.bits(), rhs1.bits());

    // !(a & b) == !a | !b
    let lhs2 = flags_a.intersection(flags_b).complement();
    let rhs2 = flags_a.complement().union(flags_b.complement());
    assert_eq!(lhs2.bits(), rhs2.bits());
}
