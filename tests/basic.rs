use neobit::neobit;

neobit! {
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
        const ALL     = Self::READ.union(Self::WRITE).union(Self::EXECUTE).bits();
        const RW      = Self::READ.union(Self::WRITE).bits();
    }
}

#[test]
fn test_creation() {
    let flags = Permissions::READ;
    assert_eq!(flags.bits(), 0b001);

    let flags = Permissions::empty();
    assert!(flags.is_empty());

    let flags = Permissions::from_bits_retain(0xFF);
    assert_eq!(flags.bits(), 0xFF);
}

#[test]
fn test_operators() {
    // OR
    let rw = Permissions::READ | Permissions::WRITE;
    assert_eq!(rw.bits(), 0b011);

    // AND
    let r = rw & Permissions::READ;
    assert_eq!(r, Permissions::READ);

    // XOR
    let x = Permissions::READ ^ Permissions::RW;
    assert_eq!(x, Permissions::WRITE);

    // SUB
    let w = Permissions::RW - Permissions::READ;
    assert_eq!(w, Permissions::WRITE);

    // NOT
    let not_read = !Permissions::READ;
    assert_eq!(not_read.bits(), !0b001u8);
}

#[test]
fn test_assign_operators() {
    let mut flags = Permissions::READ;

    flags |= Permissions::WRITE;
    assert_eq!(flags, Permissions::RW);

    flags &= Permissions::READ;
    assert_eq!(flags, Permissions::READ);

    flags ^= Permissions::WRITE;
    assert_eq!(flags, Permissions::RW);

    flags -= Permissions::WRITE;
    assert_eq!(flags, Permissions::READ);
}

#[test]
fn test_contains() {
    let all = Permissions::ALL;

    assert!(all.contains(Permissions::READ));
    assert!(all.contains(Permissions::WRITE));
    assert!(all.contains(Permissions::EXECUTE));
    assert!(all.contains(Permissions::RW));
    assert!(all.contains(Permissions::ALL));

    let rw = Permissions::RW;
    assert!(rw.contains(Permissions::READ));
    assert!(rw.contains(Permissions::WRITE));
    assert!(!rw.contains(Permissions::EXECUTE));
    assert!(!rw.contains(Permissions::ALL));
}

#[test]
fn test_intersects() {
    let rw = Permissions::RW;

    assert!(rw.intersects(Permissions::READ));
    assert!(rw.intersects(Permissions::WRITE));
    assert!(!rw.intersects(Permissions::EXECUTE));
    assert!(rw.intersects(Permissions::ALL));
}

#[test]
fn test_insert_remove_toggle() {
    let mut flags = Permissions::READ;

    flags.insert(Permissions::WRITE);
    assert_eq!(flags, Permissions::RW);

    flags.remove(Permissions::READ);
    assert_eq!(flags, Permissions::WRITE);

    flags.toggle(Permissions::READ);
    assert_eq!(flags, Permissions::RW);

    flags.toggle(Permissions::READ);
    assert_eq!(flags, Permissions::WRITE);
}

#[test]
fn test_default() {
    let flags: Permissions = Default::default();
    assert!(flags.is_empty());
}

#[test]
fn test_equality() {
    let a = Permissions::READ | Permissions::WRITE;
    let b = Permissions::RW;
    assert_eq!(a, b);

    let c = Permissions::READ;
    assert_ne!(a, c);
}

#[test]
fn test_copy_clone() {
    let a = Permissions::READ;
    let b = a; // Copy
    let c = a.clone();
    assert_eq!(a, b);
    assert_eq!(a, c);
}
neobit! {
    pub struct TestFlags: u8 {
        const A = 0b0001;
        const B = 0b0010;
        const C = 0b0100;
        const AB = Self::A.union(Self::B).bits();
    }
}

#[test]
fn test_basic() {
    let flags = TestFlags::A | TestFlags::B;
    assert!(flags.contains(TestFlags::A));
    assert!(flags.contains(TestFlags::B));
    assert!(!flags.contains(TestFlags::C));
}

#[test]
fn test_empty() {
    let flags = TestFlags::empty();
    assert!(flags.is_empty());
    assert_eq!(flags.bits(), 0);
}

#[test]
fn test_from_bits_retain() {
    let flags = TestFlags::from_bits_retain(0xFF);
    assert_eq!(flags.bits(), 0xFF);
}
