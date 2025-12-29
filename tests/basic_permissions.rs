use neobit::neobit;

neobit! {
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
    }
}

#[test]
fn test_creation() {
    let flags = Permissions::READ;
    assert_eq!(flags.bits(), 0b001);

    let flags = Permissions::empty();
    assert!(flags.is_empty());

    let all = Permissions::all();
    assert_eq!(all.bits(), 0b111);
    assert!(all.contains(Permissions::READ));
    assert!(all.contains(Permissions::WRITE));
    assert!(all.contains(Permissions::EXECUTE));
}

#[test]
fn test_from_bits_retain() {
    let flags = Permissions::from_bits_retain(0xFF);
    assert_eq!(flags.bits(), 0xFF);
}

#[test]
fn test_from_bits() {
    let flags = Permissions::from_bits(0b111);
    assert_eq!(flags.expect("Invalid bits").bits(), 0b111);

    let invalid = Permissions::from_bits(0xFF);
    assert!(invalid.is_none());
}

#[test]
fn test_operators() {
    let rw = Permissions::READ | Permissions::WRITE;
    assert_eq!(rw.bits(), 0b011);

    let r = rw & Permissions::READ;
    assert_eq!(r, Permissions::READ);

    let x = Permissions::READ ^ Permissions::READ.union(Permissions::WRITE);
    assert_eq!(x, Permissions::WRITE);

    let w = Permissions::READ.union(Permissions::WRITE) - Permissions::READ;
    assert_eq!(w, Permissions::WRITE);

    let not_read = !Permissions::READ;
    assert_eq!(not_read.bits(), !0b001u8);
}

#[test]
fn test_set() {
    let mut flags = Permissions::empty();
    flags.set(Permissions::READ, true);
    assert_eq!(flags.bits(), 0b001);
    flags.set(Permissions::WRITE, true);
    assert_eq!(flags.bits(), 0b011);
    flags.set(Permissions::READ, false);
    assert_eq!(flags.bits(), 0b010);
}

#[test]
fn test_assign_operators() {
    let mut flags = Permissions::READ;

    flags |= Permissions::WRITE;
    assert_eq!(flags, Permissions::READ.union(Permissions::WRITE));

    flags &= Permissions::READ;
    assert_eq!(flags, Permissions::READ);

    flags ^= Permissions::WRITE;
    assert_eq!(flags, Permissions::READ.union(Permissions::WRITE));

    flags -= Permissions::WRITE;
    assert_eq!(flags, Permissions::READ);
}

#[test]
fn test_contains() {
    let all = Permissions::all();
    let rw = Permissions::READ.union(Permissions::WRITE);

    assert!(all.contains(Permissions::READ));
    assert!(all.contains(Permissions::WRITE));
    assert!(all.contains(Permissions::EXECUTE));
    assert!(all.contains(rw));
    assert!(all.contains(Permissions::all()));

    assert!(rw.contains(Permissions::READ));
    assert!(rw.contains(Permissions::WRITE));
    assert!(!rw.contains(Permissions::EXECUTE));
    assert!(!rw.contains(Permissions::all()));
}

#[test]
fn test_intersects() {
    let rw = Permissions::READ.union(Permissions::WRITE);
    let all = Permissions::all();

    assert!(rw.intersects(Permissions::READ));
    assert!(rw.intersects(Permissions::WRITE));
    assert!(!rw.intersects(Permissions::EXECUTE));
    assert!(rw.intersects(all));

    assert!(all.intersects(Permissions::READ));
    assert!(all.intersects(Permissions::WRITE));
    assert!(all.intersects(Permissions::EXECUTE));
    assert!(all.intersects(rw));
}

#[test]
fn test_insert_remove_toggle() {
    let mut flags = Permissions::READ;

    flags.insert(Permissions::WRITE);
    assert_eq!(flags, Permissions::READ.union(Permissions::WRITE));

    flags.remove(Permissions::READ);
    assert_eq!(flags, Permissions::WRITE);

    flags.toggle(Permissions::READ);
    assert_eq!(flags, Permissions::READ.union(Permissions::WRITE));

    flags.toggle(Permissions::READ);
    assert_eq!(flags, Permissions::WRITE);
}

#[test]
fn test_default() {
    let flags: Permissions = Default::default();
    assert!(flags.is_empty());
}

#[test]
fn test_is_all() {
    let all = Permissions::all();
    assert!(all.is_all());
    assert!(!Permissions::READ.is_all());
}

#[test]
fn test_equality() {
    let a = Permissions::READ | Permissions::WRITE;
    let b = Permissions::READ.union(Permissions::WRITE);
    assert_eq!(a, b);

    let c = Permissions::READ;
    assert_ne!(a, c);
}
