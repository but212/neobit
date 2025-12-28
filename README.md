# neobit

[![Crates.io](https://img.shields.io/crates/v/neobit)](https://crates.io/crates/neobit)
[![Docs](https://docs.rs/neobit/badge.svg)](https://docs.rs/neobit)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/but212/neobit)

A zero-dependency, bitflags macro for systems programming. Designed for `no_std` environments.

## Why neobit?

Bit operations are simple. The library should be too.

neobit provides union, intersection, difference, complement, and membership testing. Nothing more, nothing less.

## Quick Start

```rust
use neobit::neobit;

neobit! {
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
    }
}

fn main() {
    let perms = Permissions::READ | Permissions::WRITE;

    assert!(perms.contains(Permissions::READ));
    assert!(!perms.contains(Permissions::EXECUTE));

    println!("{:?}", perms);  // Permissions(READ | WRITE)
    
    // All defined flags
    let all = Permissions::all();
    
    // Bit validation
    let valid = Permissions::from_bits(0b011);    // Some(...)
    let invalid = Permissions::from_bits(0b1000); // None
}
```

## Features

- **Zero dependencies** - Pure Rust, nothing else
- **`no_std` compatible** - Works in embedded environments
- **All integer types** - `u8`-`u128` and `i8`-`i128`
- **Readable debug output** - `Flags(READ | WRITE)` instead of `Flags { bits: 3 }`
- **`const fn` operations** - Use in const contexts
- **Pure bitwise complement** - `!flags` inverts all bits, not just known flags

## Use Cases

- Hardware register manipulation
- Network protocol flags  
- System call flags (C FFI)
- Embedded systems (`no_std`)
- Security-sensitive projects requiring easy code audit

## neobit vs bitflags

| | neobit | bitflags |
|--|--------|----------|
| Focus | Bit operations only | Bit operations + parsing + iteration + serde |
| Code size | ~520 lines | ~3,500+ lines |
| Dependencies | Zero | Optional (serde, arbitrary, bytemuck) |
| `complement()` | Pure bitwise NOT | Masked to known flags |
| `From<T>` | Included | Manual conversion needed |

Choose **neobit** for FFI, embedded, or when you want simplicity.

Choose **bitflags** if you need string parsing, iteration, or serde integration.

## Limitations

### Composite Constants in Macro

Composite constants can be defined in the macro, but require using `.union().bits()` syntax:

```rust
neobit! {
    pub struct Flags: u8 {
        const A = 0b001;     // ✅ Single bit
        const B = 0b010;     // ✅ Single bit
        const AB = Self::A.union(Self::B).bits();  // ✅ Composite constant - requires .bits()
    }
}
```

Alternatively, define composite constants outside the macro for cleaner syntax:

```rust
impl Flags {
    pub const AB: Self = Self::A.union(Self::B);  // ✅ Cleaner approach
}
```

## API Overview

### Construction

```rust
Flags::empty()              // No flags set
Flags::all()                // All defined flags
Flags::from_bits(bits)      // Validated, returns Option<Self>
Flags::from_bits_truncate(bits)  // Truncate unknown bits
Flags::from_bits_retain(bits)    // Keep all bits (for FFI)
```

### Operations

```rust
flags.contains(other)       // All bits in other are in flags
flags.intersects(other)     // Any bits in other are in flags
flags.is_empty()            // No bits set
flags.is_all()              // All defined flags set
flags.bits()                // Raw bit value
```

### Mutation

```rust
flags.insert(other)         // Add flags
flags.remove(other)         // Remove flags
flags.toggle(other)         // Flip flags
flags.set(other, condition) // Set or remove based on bool
```

### Operators

| Operator | Meaning | const fn equivalent |
|----------|---------|---------------------|
| `\|` | Union | `union()` |
| `&` | Intersection | `intersection()` |
| `^` | Symmetric difference | `symmetric_difference()` |
| `!` | Complement | `complement()` |
| `-` | Difference | `difference()` |

All operators have `*Assign` variants (`|=`, `&=`, etc.).

### Const Context

```rust
const MASK: Flags = Flags::A.union(Flags::B);
const ALL: Flags = Flags::all();
```

### Type Conversion

```rust
// From/Into (uses from_bits_retain)
let flags: Flags = 0b11.into();
let bits: u8 = flags.into();

// Explicit
let flags = Flags::from_bits_retain(0b11);
let bits = flags.bits();
```

## Complement Behavior

neobit and bitflags implement `complement()` differently:

```rust
neobit! {
    pub struct Flags: u8 {
        const A = 0b01;
        const B = 0b10;
    }
}

let flags = Flags::A;  // 0b00000001

// neobit: Pure bitwise NOT
let comp = !flags;     // 0b11111110

// bitflags: Masked to known flags
// !flags             -> 0b00000010
```

neobit preserves all bit information, which is essential for hardware registers and protocol handling.

## Signed Types

Signed integers are supported for C FFI compatibility, but be careful with `!` (complement):

```rust
neobit! {
    pub struct SignedFlags: i8 {
        const A = 0b0001;
    }
}

let complement = !SignedFlags::A;
// i8: !0b0001 = -2 (two's complement)
// u8: !0b0001 = 254

// Prefer difference() for removing flags:
let all = SignedFlags::all();
let without_a = all.difference(SignedFlags::A);
```

## C FFI Example

```rust
use neobit::neobit;
use std::ffi::c_uint;

// Define flags matching a C header
neobit! {
    #[repr(transparent)]
    pub struct RegisterFlags: c_uint {
        const READY   = 0x01;
        const ERROR   = 0x02;
        const BUSY    = 0x04;
        const DATA_RDY = 0x08;
    }
}

// Safe Rust wrapper around C functions
fn read_status() -> RegisterFlags {
    let raw = unsafe { read_register() };
    RegisterFlags::from_bits_retain(raw)  // Preserves all bits!
}

fn set_ready_flag() {
    let current = read_status();
    let updated = current | RegisterFlags::READY;
    unsafe { write_register(updated.bits()) };
}

// See examples/c_ffi_simple.rs for a complete runnable example
```

## Debug Output

Single-bit flags are shown by name. Composite constants are expanded:

```rust
println!("{:?}", Flags::READ);                    // Flags(READ)
println!("{:?}", Flags::READ | Flags::WRITE);     // Flags(READ | WRITE)
println!("{:?}", Flags::all());                    // Flags(READ | WRITE | EXECUTE)
println!("{:?}", Flags::empty());                  // Flags(empty)
println!("{:?}", Flags::from_bits_retain(0x80));   // Flags(0x80)
```

## Examples

Check out the `examples/` directory for comprehensive demonstrations:

- `quick_start.rs` - Basic usage with file permissions
- `all_method.rs` - Using the built-in `all()` method
- `bit_validation.rs` - Safe vs unchecked bit operations
- `complement_difference.rs` - How neobit differs from bitflags
- `operators_and_methods.rs` - All available operations
- `type_conversion.rs` - Converting between integers and flags
- `c_ffi_simple.rs` - C FFI and hardware register example
- `limitations.rs` - Macro limitations and workarounds

Run them with:

```bash
cargo run --example <example_name>
```

## Minimum Rust Version

Rust 1.56 or later.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
