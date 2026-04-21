use neobit::neobit;

neobit! {
    /// Test struct with a flag named __NEOBIT_INTERNAL_FLAGS_REGISTRY to test hygiene
    /// This should not collide with the internal name.
    pub struct HygieneFlags: u8 {
        const A = 0b0001;
        const B = 0b0010;
        // Collision attempt with the OLD internal name
        const __FLAGS = 0b0100;
        // Collision attempt with the NEW internal name
        const __NEOBIT_INTERNAL_FLAGS_REGISTRY = 0b1000;
    }
}

#[test]
fn test_macro_hygiene() {
    let f =
        HygieneFlags::A | HygieneFlags::__FLAGS | HygieneFlags::__NEOBIT_INTERNAL_FLAGS_REGISTRY;
    assert!(f.contains(HygieneFlags::A));
    assert!(f.contains(HygieneFlags::__FLAGS));
    assert!(f.contains(HygieneFlags::__NEOBIT_INTERNAL_FLAGS_REGISTRY));

    // Test Debug output to ensure it still works
    let debug_str = format!("{:?}", f);
    assert!(debug_str.contains("A"));
    assert!(debug_str.contains("__FLAGS"));
    assert!(debug_str.contains("__NEOBIT_INTERNAL_FLAGS_REGISTRY"));
}
