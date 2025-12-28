//! Simple C FFI example
//!
//! Demonstrates using neobit for C interoperability with hardware registers.

use neobit::neobit;
use std::ffi::c_uint;

// Define flags matching a C header
neobit! {
    /// Hardware register flags (matches C definition)
    #[repr(transparent)]
    pub struct RegisterFlags: c_uint {
        /// Ready bit
        const READY   = 0x01;
        /// Error bit
        const ERROR   = 0x02;
        /// Busy bit
        const BUSY    = 0x04;
        /// Data available bit
        const DATA_RDY = 0x08;
    }
}

// Mock implementations for demonstration
#[no_mangle]
pub extern "C" fn read_register() -> c_uint {
    // Simulate a register with READY and DATA_RDY set
    RegisterFlags::READY.bits() | RegisterFlags::DATA_RDY.bits()
}

#[no_mangle]
pub extern "C" fn write_register(value: c_uint) {
    println!("Wrote register value: {:#010x}", value);
}

// Safe Rust wrapper
#[allow(dead_code)]
fn read_status() -> RegisterFlags {
    // In real C FFI, this would be unsafe
    // We use unsafe here to demonstrate the proper pattern
    let raw = unsafe { read_register() };
    RegisterFlags::from_bits_retain(raw)
}

#[allow(dead_code)]
fn set_ready_flag() {
    let current = read_status();
    let updated = current | RegisterFlags::READY;
    // In real C FFI, this would be unsafe
    // We use unsafe here to demonstrate the proper pattern
    unsafe { write_register(updated.bits()) };
}

#[allow(dead_code)]
fn clear_error_flag() {
    let current = read_status();
    let updated = current & !RegisterFlags::ERROR;
    // In real C FFI, this would be unsafe
    // We use unsafe here to demonstrate the proper pattern
    unsafe { write_register(updated.bits()) };
}

fn main() {
    println!("=== C FFI Example ===");

    // Read status
    let status = read_status();
    println!("Status: {:?}", status);
    println!("Raw bits: {:#010x}", status.bits());

    // Check flags
    if status.contains(RegisterFlags::READY) {
        println!("Device is ready");
    }

    if status.contains(RegisterFlags::ERROR) {
        println!("Device has error");
    } else {
        println!("No error detected");
    }

    if status.intersects(RegisterFlags::BUSY | RegisterFlags::DATA_RDY) {
        if status.contains(RegisterFlags::DATA_RDY) {
            println!("Data is ready to read");
        }
        if status.contains(RegisterFlags::BUSY) {
            println!("Device is busy");
        }
    }

    // Modify flags
    println!("\n=== Modifying flags ===");
    set_ready_flag();
    clear_error_flag();

    let new_status = read_status();
    println!("New status: {:?}", new_status);

    // Demonstrate preserving unknown bits
    println!("\n=== Preserving unknown bits ===");
    let raw_with_unknown = 0x1234; // Contains unknown bits
    let flags = RegisterFlags::from_bits_retain(raw_with_unknown);
    println!("Raw: {:#010x}", raw_with_unknown);
    println!("Parsed: {:?}", flags);
    println!("Preserved unknown bits: {:#010x}", flags.bits() & 0x1230);

    println!("\nAll examples passed!");
}
