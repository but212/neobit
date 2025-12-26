//! C FFI example demonstrating signed integer bitflags.
//!
//! This example shows how neobit can be used with signed integer types
//! for compatibility with C libraries that use `int` for flags.

use neobit::neobit;

neobit! {
    /// Flags compatible with a hypothetical C library.
    ///
    /// Many C libraries define flags as `int` constants:
    /// ```c
    /// #define OPTION_NONE    0x00
    /// #define OPTION_VERBOSE 0x01
    /// #define OPTION_DEBUG   0x02
    /// #define OPTION_FORCE   0x04
    /// ```
    pub struct COptions: i32 {
        const NONE    = 0x00;
        const VERBOSE = 0x01;
        const DEBUG   = 0x02;
        const FORCE   = 0x04;
        const ALL     = Self::VERBOSE.union(Self::DEBUG).union(Self::FORCE).bits();
    }
}

// Simulated C function
fn c_library_init(flags: i32) {
    println!("C library initialized with flags: {:#x}", flags);
}

fn c_library_process(flags: i32) -> i32 {
    // Simulate some processing
    flags | 0x100 // Add some "result" flag
}

fn main() {
    // Building flags for C call
    let options = COptions::VERBOSE | COptions::DEBUG;
    println!("Options: {:?}", options);

    // Passing to C function (using Into)
    c_library_init(options.into());

    // Alternative: using .bits()
    c_library_init(options.bits());

    // Receiving flags from C
    let result = c_library_process(options.into());
    let result_flags: COptions = result.into();
    println!("Result flags: {:?}", result_flags);
    println!(
        "Result contains VERBOSE? {}",
        result_flags.contains(COptions::VERBOSE)
    );

    // Unknown bits are preserved (important for forward compatibility)
    println!("Unknown bits preserved: {:#x}", result_flags.bits());

    // Warning about complement on signed types
    println!("\n--- Signed complement warning ---");
    let flags = COptions::VERBOSE;
    let complement = !flags;
    println!("!VERBOSE = {} (two's complement)", complement.bits());
    println!("Use .difference() instead for removing flags:");

    let all_but_verbose = COptions::ALL.difference(COptions::VERBOSE);
    println!("ALL - VERBOSE = {:?}", all_but_verbose);
}
