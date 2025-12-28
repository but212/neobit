# neobit

[![Crates.io](https://img.shields.io/crates/v/neobit)](https://crates.io/crates/neobit)
[![Docs](https://docs.rs/neobit/badge.svg)](https://docs.rs/neobit)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/but212/neobit)

A zero-dependency, bitflags macro for systems programming. Designed for `no_std` environments, fast compilation, and drop-in replacements.

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

Signed integers are supported for C FFI compatibility:

```rust
neobit! {
    pub struct SignedFlags: i32 {
        const A = 0b0001;
    }
}
```

Note: `complement()` follows Rust's two's complement semantics.

## Minimum Rust Version

Rust 1.56 or later.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
