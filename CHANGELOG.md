# Changelog

## [0.1.1] - 2025-12-28

### Added - 0.1.1

- Added 6 comprehensive example files demonstrating various neobit use cases:
  - `network_protocol.rs`: TCP flags, HTTP method flags, and protocol parsing
  - `const_operations.rs`: Compile-time flag operations and configurations
  - `hardware_register.rs`: Embedded systems GPIO, SPI, and DMA register manipulation
  - `validation.rs`: Comparison of `from_bits()` vs `from_bits_retain()` with use case guidelines
  - `composite_flags.rs`: Unix file permissions and common composite flag patterns
  - `integer_types.rs`: Support for u8-u128 and signed integer types

## [0.1.0] - 2025-12-28

### Added - 0.1.0

- Initial release of neobit - zero-dependency bitflags library
- Core `neobit!` macro for defining bitflags with any integer type (`u8`-`u128`, `i8`-`i128`)
- Built-in `all()` method returning union of all defined flags
- Readable debug output formatting (e.g., `Flags(READ | WRITE)`)
- `const fn` operations: `union()`, `intersection()`, `difference()`, `symmetric_difference()`, `complement()`
- Flexible bit validation with `from_bits()` (validated) and `from_bits_retain()` (unchecked)
- Standard bitwise operators: `|`, `&`, `^`, `!`, `-` with `*Assign` variants
- Methods: `empty()`, `bits()`, `is_empty()`, `is_all()`, `contains()`, `intersects()`, `insert()`, `remove()`, `toggle()`, `set()`
- `no_std` compatibility for embedded systems
- Trait implementations: `Default`, `From<T>`, `Into<T>`, `Debug`, `Copy`, `Clone`, `Eq`, `PartialEq`, `Hash`, and operator traits (`BitOr`, `BitAnd`, `BitXor`, `Not`, `Sub` with `*Assign` variants)
- Apache 2.0 and MIT dual licensing
- Comprehensive documentation with examples
- CI/CD infrastructure with GitHub Actions
- Test coverage across all methods and edge cases
- Examples for C FFI and permissions use cases
- README with quick start, API overview, design philosophy, and limitations
- Inline documentation for all public APIs
- Comparison with `bitflags` crate highlighting design differences
- NOTICE file with Apache 2.0 license attribution
- Limitations documentation explaining composite constants restriction
- Cargo.toml configuration with metadata
- clippy.toml for linting rules
- rustfmt.toml for code formatting
- deny.toml for dependency auditing
