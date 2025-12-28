//! Validation and error handling example.
//!
//! Demonstrates the difference between `from_bits()` and `from_bits_retain()`.

use neobit::neobit;

neobit! {
    /// Access control flags
    pub struct AccessControl: u8 {
        const READ    = 0b0001;
        const WRITE   = 0b0010;
        const EXECUTE = 0b0100;
        const DELETE  = 0b1000;
    }
}

neobit! {
    /// Status flags from a hypothetical API response
    pub struct StatusFlags: u16 {
        const SUCCESS    = 1 << 0;
        const WARNING    = 1 << 1;
        const ERROR      = 1 << 2;
        const DEPRECATED = 1 << 3;
        const CACHED     = 1 << 4;
    }
}

/// Parse access control from user input
fn parse_access_strict(bits: u8) -> Result<AccessControl, String> {
    AccessControl::from_bits(bits).ok_or_else(|| format!("Invalid access bits: {:#04x}", bits))
}

/// Parse access control, preserving unknown bits
fn parse_access_lenient(bits: u8) -> AccessControl {
    AccessControl::from_bits_retain(bits)
}

/// Handle API response with validation
fn handle_api_response(status_bits: u16) {
    println!("\n=== API Response: {:#06x} ===", status_bits);

    // Validate the response
    match StatusFlags::from_bits(status_bits) {
        Some(status) => {
            println!("✓ Valid status: {:?}", status);

            if status.contains(StatusFlags::ERROR) {
                println!("  ⚠ Response contains ERROR flag");
            }
            if status.contains(StatusFlags::WARNING) {
                println!("  ⚠ Response contains WARNING flag");
            }
            if status.contains(StatusFlags::SUCCESS) {
                println!("  ✓ Operation successful");
            }
            if status.contains(StatusFlags::CACHED) {
                println!("  ℹ Response from cache");
            }
        }
        None => {
            println!("✗ Invalid status - contains unknown flags");

            // Still process it by retaining unknown bits
            let status = StatusFlags::from_bits_retain(status_bits);
            println!("  Retained flags: {:?}", status);
            println!("  Raw bits: {:#018b}", status_bits);

            // Find which flags we recognize
            let known = StatusFlags::all();
            let unknown_bits = status_bits & !known.bits();
            println!("  Unknown bits: {:#018b}", unknown_bits);
        }
    }
}

fn main() {
    println!("=== Strict Validation (from_bits) ===\n");

    // Valid permissions
    let valid_bits = 0b0011; // READ | WRITE
    match parse_access_strict(valid_bits) {
        Ok(perms) => println!("✓ Parsed {:#04x} as {:?}", valid_bits, perms),
        Err(e) => println!("✗ {}", e),
    }

    // Invalid permissions (unknown bit set)
    let invalid_bits = 0b10011; // READ | WRITE | unknown bit
    match parse_access_strict(invalid_bits) {
        Ok(perms) => println!("✓ Parsed {:#04x} as {:?}", invalid_bits, perms),
        Err(e) => println!("✗ {}", e),
    }

    println!("\n=== Lenient Parsing (from_bits_retain) ===\n");

    // Parse the same invalid bits, keeping everything
    let perms = parse_access_lenient(invalid_bits);
    println!("Parsed {:#04x} as {:?}", invalid_bits, perms);
    println!("Raw bits: {:#08b}", perms.bits());

    // The unknown bits are preserved
    println!("\n=== Checking Individual Flags ===\n");
    println!("Contains READ? {}", perms.contains(AccessControl::READ));
    println!("Contains WRITE? {}", perms.contains(AccessControl::WRITE));
    println!(
        "Contains EXECUTE? {}",
        perms.contains(AccessControl::EXECUTE)
    );

    // All defined flags
    let all = AccessControl::all();
    println!("\nAll defined flags: {:?}", all);
    println!("All bits: {:#08b}", all.bits());

    // Check what's in the parsed perms that's not in all
    let extra_bits = perms.bits() & !all.bits();
    if extra_bits != 0 {
        println!("\nExtra (unknown) bits: {:#08b}", extra_bits);
    }

    println!("\n=== API Response Handling ===");

    // Valid API response
    handle_api_response(0b00001); // SUCCESS only

    // Multiple flags
    handle_api_response(0b00011); // SUCCESS | WARNING

    // Future API version with new flag
    handle_api_response(0b01_00001); // SUCCESS + unknown future flag

    // Completely unknown response
    handle_api_response(0xFF00);

    println!("\n=== Use Case Comparison ===\n");

    println!("Use from_bits() when:");
    println!("  • Validating user input");
    println!("  • Ensuring only known flags are set");
    println!("  • Building type-safe APIs");
    println!("  • Application-level flags\n");

    println!("Use from_bits_retain() when:");
    println!("  • Parsing C FFI data");
    println!("  • Hardware register access");
    println!("  • Protocol parsing with reserved bits");
    println!("  • Forward compatibility with future flags");
    println!("  • Need to preserve all bits exactly\n");

    println!("=== Converting Between Types ===\n");

    let perms = AccessControl::READ | AccessControl::WRITE;

    // To primitive
    let bits_u8: u8 = perms.into();
    let bits_via_method = perms.bits();
    println!("Permissions as u8: {:#04x}", bits_u8);
    assert_eq!(bits_u8, bits_via_method);

    // From primitive (unchecked by default with From)
    let restored: AccessControl = bits_u8.into();
    println!("Restored from u8: {:?}", restored);
    assert_eq!(perms, restored);

    // Demonstrate the difference
    let invalid: u8 = 0xFF;
    let from_trait: AccessControl = invalid.into(); // Uses from_bits_retain
    println!("\nFrom\u{003c}T\u{003e} with 0xFF: {:?}", from_trait);

    match AccessControl::from_bits(invalid) {
        Some(v) => println!("from_bits with 0xFF: {:?}", v),
        None => println!("from_bits with 0xFF: None (invalid)"),
    }
}
