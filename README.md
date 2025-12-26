# neobit

[![Crates.io](https://img.shields.io/crates/v/neobit.svg)](https://crates.io/crates/neobit)
[![Documentation](https://docs.rs/neobit/badge.svg)](https://docs.rs/neobit)
[![License](https://img.shields.io/crates/l/neobit.svg)](LICENSE-MIT)

Zero-dependency, lightweight bitflags with readable debug output.

## Features

- **Zero dependencies** - Pure Rust, no external crates
- **Lightweight** - ~260 lines of code
- **Readable debug output** - `Flags(READ | WRITE)` instead of `Flags { bits: 3 }`
- **`const fn` operations** - Use in const contexts with `union()`, `intersection()`, etc.
- **`no_std` compatible** - Works in embedded environments
- **All integer types** - Supports `u8`-`u128` and `i8`-`i128`

## Quick Start

```rust
use neobit::neobit;

neobit! {
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
        const ALL     = Self::READ.union(Self::WRITE).union(Self::EXECUTE);
    }
}

fn main() {
    let perms = Permissions::READ | Permissions::WRITE;

    assert!(perms.contains(Permissions::READ));
    assert!(!perms.contains(Permissions::EXECUTE));

    println!("{:?}", perms);  // Permissions(READ | WRITE)
}
```

## Who Should Use This

### ✅ Good Fit

- C FFI bindings (hardware registers, system calls)
- Protocol parsing (network packets, binary formats)
- Embedded systems (`no_std` environments)
- Libraries that want minimal dependencies

### ⚠️ Consider `bitflags` Instead

- You need validated flags (`from_bits() -> Option<T>`)
- You need iterator support
- You're building a beginner-friendly application

## Design Philosophy

### Unchecked by Design

neobit **intentionally does not validate bits**. Unknown bits are preserved, not rejected.

```rust
// bitflags: unknown bits → None
let flags = Flags::from_bits(0xFF);  // None

// neobit: unknown bits → preserved
let flags = Flags::from_bits_retain(0xFF);  // All bits kept
```

This is a deliberate trade-off:

| Aspect | neobit | bitflags |
|--------|--------|----------|
| C FFI / Registers | ✅ No data loss | ⚠️ May lose unknown bits |
| Protocol parsing | ✅ Future-compatible | ⚠️ Version mismatch issues |
| Beginner safety | ⚠️ No validation | ✅ Option protection |

## API Overview

### Operators

| Operator | Meaning | const fn equivalent |
|----------|---------|---------------------|
| `\|` | Union | `union()` |
| `&` | Intersection | `intersection()` |
| `^` | Symmetric difference | `symmetric_difference()` |
| `!` | Complement | `complement()` |
| `-` | Difference | `difference()` |

All operators have `*Assign` variants (`|=`, `&=`, etc.).

### Methods

```rust
// Construction
Flags::empty()
Flags::from_bits_retain(bits)

// Access
flags.bits()
flags.is_empty()
flags.contains(other)
flags.intersects(other)

// Mutation
flags.insert(other)
flags.remove(other)
flags.toggle(other)

// Const operations
flags.union(other)
flags.intersection(other)
flags.difference(other)
flags.symmetric_difference(other)
flags.complement()
```

### Const Context

Use const methods for compile-time flag combinations:

```rust
neobit! {
    pub struct Flags: u32 {
        const A = 1 << 0;
        const B = 1 << 1;
        const AB = Self::A.union(Self::B);  // const fn
    }
}

const MASK: Flags = Flags::A.union(Flags::B);  // Compile-time
```

### Type Conversion

```rust
// From/Into
let flags: Flags = 0b11.into();
let bits: u8 = flags.into();

// Explicit
let flags = Flags::from_bits_retain(0b11);
let bits = flags.bits();
```

## Signed Types Warning

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
let without_a = all.difference(SignedFlags::A);
```

## Debug Output

Single-bit flags are shown by name. Composite constants are expanded:

```rust
println!("{:?}", Flags::READ);           // Flags(READ)
println!("{:?}", Flags::READ | Flags::WRITE);  // Flags(READ | WRITE)
println!("{:?}", Flags::ALL);            // Flags(READ | WRITE | EXECUTE)
println!("{:?}", Flags::empty());        // Flags(empty)
println!("{:?}", Flags::from_bits_retain(0x80));  // Flags(0x80)
```

## Minimum Rust Version

neobit requires Rust 1.56 or later.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
