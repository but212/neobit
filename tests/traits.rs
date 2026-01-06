//! Tests for all derived traits of neobit flags

use neobit::neobit;

neobit! {
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
    }
}

// =============================================================================
// Copy Trait Tests
// =============================================================================

#[test]
fn test_copy_trait() {
    let a = Permissions::READ | Permissions::WRITE;
    let b = a;
    assert_eq!(a, b);
    assert!(a.contains(Permissions::READ));
}

// =============================================================================
// Clone Trait Tests
// =============================================================================

#[test]
fn test_clone_trait() {
    let a = Permissions::all();
    let b = a.clone();
    assert_eq!(a, b);
    assert!(b.contains(Permissions::READ));
    assert!(b.contains(Permissions::WRITE));
    assert!(b.contains(Permissions::EXECUTE));
}

// =============================================================================
// Eq Trait Tests
// =============================================================================

#[test]
fn test_eq_trait() {
    let a = Permissions::READ | Permissions::WRITE;
    let b = Permissions::WRITE | Permissions::READ;
    let c = Permissions::READ;

    assert_eq!(a, b);
    assert_eq!(b, a);
    assert_ne!(a, c);
    assert_eq!(a, a);
    assert_eq!(Permissions::empty(), Permissions::empty());
    assert_eq!(Permissions::all(), Permissions::all());
}

// =============================================================================
// PartialEq Trait Tests
// =============================================================================

#[test]
fn test_partial_eq_trait() {
    let a: Permissions = 0b111.into();
    let b: Permissions = 0b111.into();
    let c: Permissions = 0b110.into();

    assert_eq!(a, b);
    assert_ne!(a, c);

    let known = Permissions::READ | Permissions::WRITE;
    let with_unknown: Permissions = 0b111.into();
    assert_ne!(known, with_unknown);
}

// =============================================================================
// Ord Trait Tests
// =============================================================================

#[test]
fn test_ord_trait() {
    let a = Permissions::READ;
    let b = Permissions::WRITE;
    let c = Permissions::EXECUTE;
    let d = Permissions::READ | Permissions::WRITE;
    let e = Permissions::all();

    assert!(a < b);
    assert!(b < c);
    assert!(a < d);
    assert!(d < c);
    assert!(e > d);
    assert!(e > c);

    let flags = [
        Permissions::empty(),
        Permissions::READ,
        Permissions::WRITE,
        Permissions::READ | Permissions::WRITE,
        Permissions::EXECUTE,
        Permissions::READ | Permissions::EXECUTE,
        Permissions::WRITE | Permissions::EXECUTE,
        Permissions::all(),
    ];

    for i in 1..flags.len() {
        assert!(flags[i - 1] < flags[i]);
    }
}

// =============================================================================
// Hash Trait Tests
// =============================================================================

#[test]
fn test_hash_trait() {
    use std::collections::HashSet;

    let a = Permissions::READ | Permissions::WRITE;
    let b = Permissions::WRITE | Permissions::READ;
    let c = Permissions::READ;

    assert_eq!(calc_hash(&a), calc_hash(&b));
    assert_ne!(calc_hash(&a), calc_hash(&c));

    let mut set = HashSet::new();
    set.insert(Permissions::READ);
    set.insert(Permissions::WRITE);
    set.insert(Permissions::READ | Permissions::WRITE);
    set.insert(Permissions::EXECUTE);

    assert!(set.contains(&Permissions::READ));
    assert!(set.contains(&Permissions::WRITE));
    assert!(set.contains(&(Permissions::READ | Permissions::WRITE)));
    assert!(set.contains(&Permissions::EXECUTE));

    let size_before = set.len();
    set.insert(Permissions::READ);
    assert_eq!(size_before, set.len());
}

#[test]
fn test_hash_with_unknown_bits() {
    use std::collections::HashSet;

    let a: Permissions = 0b101.into();
    let b: Permissions = 0b101.into();
    let c = Permissions::READ | Permissions::EXECUTE;

    assert_eq!(calc_hash(&a), calc_hash(&b));
    assert_eq!(calc_hash(&a), calc_hash(&c));

    let mut set = HashSet::new();
    set.insert(Permissions::from(0xFF));
    set.insert(Permissions::from(0xFF));
    assert_eq!(set.len(), 1);

    set.insert(Permissions::from(0xFE));
    assert_eq!(set.len(), 2);
}

// =============================================================================
// All Derived Traits Together Test
// =============================================================================

#[test]
fn test_all_derived_traits_together() {
    let mut vec = Vec::new();

    vec.push(Permissions::empty());
    vec.push(Permissions::READ);
    vec.push(Permissions::WRITE);
    vec.push(Permissions::READ | Permissions::WRITE);
    vec.push(Permissions::EXECUTE);
    vec.push(Permissions::all());
    vec.push(0b1010.into()); // Using From trait

    // Test Copy (clone the vec)
    let vec_clone = vec.clone();
    assert_eq!(vec, vec_clone);

    // Test Ord (sort the vec)
    vec.sort();
    for i in 1..vec.len() {
        assert!(vec[i - 1] <= vec[i]);
    }

    // Test Hash (put in HashSet)
    let set: std::collections::HashSet<_> = vec.into_iter().collect();
    assert_eq!(set.len(), 7);

    // Test Eq/PartialEq
    for item in &vec_clone {
        assert!(set.contains(item));
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

fn calc_hash<T: std::hash::Hash>(item: &T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}
