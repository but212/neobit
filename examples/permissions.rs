//! Basic permissions example demonstrating neobit usage.

use neobit::neobit;

neobit! {
    /// Unix-style file permissions
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
        const ALL     = Self::READ.union(Self::WRITE).union(Self::EXECUTE).bits();
        const RW      = Self::READ.union(Self::WRITE).bits();
        const RX      = Self::READ.union(Self::EXECUTE).bits();
    }
}

fn main() {
    // Creating flags
    let mut perms = Permissions::READ | Permissions::WRITE;
    println!("Initial: {:?}", perms);
    // Output: Permissions(READ | WRITE)

    // Checking flags
    println!("Can read? {}", perms.contains(Permissions::READ));
    println!("Can execute? {}", perms.contains(Permissions::EXECUTE));

    // Modifying flags
    perms.insert(Permissions::EXECUTE);
    println!("After insert EXECUTE: {:?}", perms);

    perms.remove(Permissions::WRITE);
    println!("After remove WRITE: {:?}", perms);

    perms.toggle(Permissions::WRITE);
    println!("After toggle WRITE: {:?}", perms);

    // Using operators
    let script_perms = Permissions::RX;
    let combined = perms | script_perms;
    println!("Combined: {:?}", combined);

    // Checking intersections
    if perms.intersects(Permissions::RW) {
        println!("Has at least one of READ or WRITE");
    }

    // Binary representation
    println!("Binary: {:08b}", perms.bits());
    println!("Binary (formatted): {:#010b}", perms);

    // Type conversion
    let raw: u8 = perms.into();
    let restored: Permissions = raw.into();
    assert_eq!(perms, restored);
    println!("Round-trip successful: {} -> {:?}", raw, restored);
}
