//! # neobit
//!
//! Zero-dependency, lightweight bitflags with readable debug output.
//!
//! ## Quick Start
//!
//! ```rust
//! use neobit::neobit;
//!
//! neobit! {
//!     /// File permissions
//!     pub struct Permissions: u8 {
//!         const READ    = 0b001;
//!         const WRITE   = 0b010;
//!         const EXECUTE = 0b100;
//!     }
//! }
//!
//! let perms = Permissions::READ | Permissions::WRITE;
//! assert!(perms.contains(Permissions::READ));
//! println!("{:?}", perms);  // Permissions(READ | WRITE)
//!
//! // Get all flags
//! let all = Permissions::all();
//! assert!(all.contains(Permissions::READ | Permissions::WRITE | Permissions::EXECUTE));
//!
//! // Validate bits
//! let valid = Permissions::from_bits(0b011);
//! assert!(valid.is_some());
//! let invalid = Permissions::from_bits(0b1000);
//! assert!(invalid.is_none());
//! ```
//!
//! ## Design Philosophy
//!
//! **Flexible Bit Validation**: neobit provides both validated and unchecked bit operations.
//! - `from_bits()` validates bits and returns `Option<Self>`
//! - `from_bits_retain()` preserves all bits without validation
//!
//! This preserves all bit information when needed, which is essential for:
//! - C FFI bindings
//! - Protocol parsing
//! - Hardware register access
//!
//! ## Signed Types Warning
//!
//! Signed integer types are supported for ABI compatibility, but be careful
//! with the `!` (complement) operator - it follows Rust's two's complement
//! semantics which may produce unexpected results.

#![no_std]

/// Defines a bitflags struct with the specified flags.
///
/// # Example
///
/// ```rust
/// use neobit::neobit;
///
/// neobit! {
///     pub struct Flags: u8 {
///         const A = 0b0001;
///         const B = 0b0010;
///         const C = 0b0100;
///     }
/// }
///
/// let flags = Flags::A | Flags::B;
/// let all = Flags::all();
/// assert!(all.contains(flags));
/// ```
#[macro_export]
macro_rules! neobit {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident: $int_ty:ty {
            $(
                $(#[$const_meta:meta])*
                const $flag_name:ident = $flag_value:expr;
            )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        $vis struct $name {
            bits: $int_ty,
        }

        impl $name {
            $(
                $(#[$const_meta])*
                pub const $flag_name: Self = Self { bits: $flag_value };
            )*

            /// Internal: flag names and values for Debug output
            const __FLAGS: &'static [(&'static str, $int_ty)] = &[
                $((stringify!($flag_name), $flag_value),)*
            ];

            /// Creates an empty flags value (all bits unset).
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; } }
            /// let flags = Flags::empty();
            /// assert!(flags.is_empty());
            /// ```
            #[inline(always)]
            pub const fn empty() -> Self {
                Self { bits: 0 }
            }

            /// Creates a flags value from raw bits if all bits are valid.
            ///
            /// Returns `None` if any bits are set that don't correspond to a defined flag.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; } }
            /// assert!(Flags::from_bits(0b11).is_some());
            /// assert!(Flags::from_bits(0b100).is_none());
            /// ```
            #[inline(always)]
            pub const fn from_bits(bits: $int_ty) -> Option<Self> {
                let all = Self::all().bits;

                if (bits & !all) == 0 {
                    Some(Self { bits })
                } else {
                    None
                }
            }

            /// Creates a flags value from raw bits, truncating any unknown bits.
            ///
            /// This is equivalent to `from_bits_retain(bits & Self::all().bits())`.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; } }
            /// let flags = Flags::from_bits_truncate(0b101); // 0b101 & 0b001 = 0b001
            /// assert_eq!(flags, Flags::A);
            /// ```
            #[inline(always)]
            pub const fn from_bits_truncate(bits: $int_ty) -> Self {
                Self::from_bits_retain(bits & Self::all().bits())
            }


            /// Creates a flags value from raw bits, retaining all bits.
            ///
            /// This does not validate the bits - unknown bits are preserved.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; } }
            /// let flags = Flags::from_bits_retain(0xFF);
            /// assert_eq!(flags.bits(), 0xFF);
            /// ```
            #[inline(always)]
            pub const fn from_bits_retain(bits: $int_ty) -> Self {
                Self { bits }
            }

            /// Returns the raw bit value.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; } }
            /// let flags = Flags::A | Flags::B;
            /// assert_eq!(flags.bits(), 0b11);
            /// ```
            #[inline(always)]
            pub const fn bits(self) -> $int_ty {
                self.bits
            }

            /// Sets or removes the specified flags based on a boolean condition.
            ///
            /// If `condition` is `true`, the flags in `other` are inserted.
            /// If `condition` is `false`, the flags in `other` are removed.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; } }
            /// let mut flags = Flags::A;
            /// flags.set(Flags::B, true);
            /// assert_eq!(flags, Flags::A | Flags::B);
            /// flags.set(Flags::A, false);
            /// assert_eq!(flags, Flags::B);
            /// ```
            #[inline(always)]
            pub fn set(&mut self, other: Self, condition: bool) {
                let m = (condition as $int_ty).wrapping_neg();
                self.bits = (self.bits & !other.bits) | (other.bits & m);
            }

            /// Returns the union of two flags (OR).
            ///
            /// This is the `const fn` equivalent of the `|` operator.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; } }
            /// const AB: Flags = Flags::A.union(Flags::B);
            /// assert_eq!(AB.bits(), 0b11);
            /// ```
            #[inline(always)]
            pub const fn union(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }

            /// Returns the intersection of two flags (AND).
            ///
            /// This is the `const fn` equivalent of the `&` operator.
            #[inline(always)]
            pub const fn intersection(self, other: Self) -> Self {
                Self { bits: self.bits & other.bits }
            }

            /// Returns the difference of two flags (self AND NOT other).
            ///
            /// This is the `const fn` equivalent of the `-` operator.
            #[inline(always)]
            pub const fn difference(self, other: Self) -> Self {
                Self { bits: self.bits & !other.bits }
            }

            /// Returns the symmetric difference of two flags (XOR).
            ///
            /// This is the `const fn` equivalent of the `^` operator.
            #[inline(always)]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }

            /// Returns the bitwise complement (NOT).
            ///
            /// This is the `const fn` equivalent of the `!` operator.
            ///
            /// # Semantic Difference from bitflags
            ///
            /// **neobit**: Returns the pure bitwise complement (all bits inverted).
            /// **bitflags**: Returns the complement of defined flags only (masked with ALL).
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 0b01; const B = 0b10; } }
            /// let flags = Flags::A;  // 0b01
            /// let complement = flags.complement();  // neobit: !0b01 = 0b11111110
            /// // bitflags would return: !0b01 & 0b11 = 0b10 (only defined flags)
            /// ```
            ///
            /// # Warning
            ///
            /// For signed integer types, this follows Rust's two's complement
            /// semantics which may produce unexpected results.
            #[inline(always)]
            pub const fn complement(self) -> Self {
                Self { bits: !self.bits }
            }

            /// Returns the union of all defined flags.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; } }
            /// let all = Flags::all();
            /// assert!(all.contains(Flags::A));
            /// assert!(all.contains(Flags::B));
            /// ```
            #[inline(always)]
            pub const fn all() -> Self {
                let mut result = Self { bits: 0 };
                $(result.bits |= $flag_value;)*
                result
            }

            /// Returns `true` if no flags are set.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; } }
            /// assert!(Flags::empty().is_empty());
            /// assert!(!Flags::A.is_empty());
            /// ```
            #[inline(always)]
            pub const fn is_empty(self) -> bool {
                self.bits == 0
            }

            /// Returns `true` if all defined flags are set.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; } }
            /// assert!(Flags::all().is_all());
            /// assert!(!(Flags::A).is_all());
            /// ```
            #[inline(always)]
            pub const fn is_all(self) -> bool {
                self.bits == Self::all().bits
            }

            /// Returns `true` if all flags in `other` are contained in `self`.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; } }
            /// let ab = Flags::A | Flags::B;
            /// assert!(ab.contains(Flags::A));
            /// assert!(ab.contains(Flags::A | Flags::B));
            /// assert!(!Flags::A.contains(Flags::B));
            /// ```
            #[inline(always)]
            pub const fn contains(self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits
            }

            /// Returns `true` if any flags in `other` are contained in `self`.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; const C = 4; } }
            /// let ab = Flags::A | Flags::B;
            /// assert!(ab.intersects(Flags::A));
            /// assert!(ab.intersects(Flags::B | Flags::C));
            /// assert!(!ab.intersects(Flags::C));
            /// ```
            #[inline(always)]
            pub const fn intersects(self, other: Self) -> bool {
                (self.bits & other.bits) != 0
            }

            /// Inserts the flags in `other` into `self`.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; } }
            /// let mut flags = Flags::A;
            /// flags.insert(Flags::B);
            /// assert_eq!(flags, Flags::A | Flags::B);
            /// ```
            #[inline(always)]
            pub fn insert(&mut self, other: Self) {
                self.bits |= other.bits;
            }

            /// Removes the flags in `other` from `self`.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; } }
            /// let mut flags = Flags::A | Flags::B;
            /// flags.remove(Flags::A);
            /// assert_eq!(flags, Flags::B);
            /// ```
            #[inline(always)]
            pub fn remove(&mut self, other: Self) {
                self.bits &= !other.bits;
            }

            /// Toggles the flags in `other` in `self`.
            ///
            /// # Example
            ///
            /// ```rust
            /// # use neobit::neobit;
            /// # neobit! { pub struct Flags: u8 { const A = 1; const B = 2; } }
            /// let mut flags = Flags::A;
            /// flags.toggle(Flags::B);
            /// assert_eq!(flags, Flags::A | Flags::B);
            /// flags.toggle(Flags::A);
            /// assert_eq!(flags, Flags::B);
            /// ```
            #[inline(always)]
            pub fn toggle(&mut self, other: Self) {
                self.bits ^= other.bits;
            }
        }

        impl Default for $name {
            #[inline(always)]
            fn default() -> Self {
                Self::empty()
            }
        }

        impl From<$int_ty> for $name {
            #[inline(always)]
            fn from(bits: $int_ty) -> Self {
                Self::from_bits_retain(bits)
            }
        }

        impl From<$name> for $int_ty {
            #[inline(always)]
            fn from(flags: $name) -> $int_ty {
                flags.bits()
            }
        }

        impl core::ops::BitOr for $name {
            type Output = Self;
            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self {
                self.union(rhs)
            }
        }

        impl core::ops::BitOrAssign for $name {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                *self = self.union(rhs);
            }
        }

        impl core::ops::BitAnd for $name {
            type Output = Self;
            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self {
                self.intersection(rhs)
            }
        }

        impl core::ops::BitAndAssign for $name {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self) {
                *self = self.intersection(rhs);
            }
        }

        impl core::ops::BitXor for $name {
            type Output = Self;
            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self {
                self.symmetric_difference(rhs)
            }
        }

        impl core::ops::BitXorAssign for $name {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = self.symmetric_difference(rhs);
            }
        }

        impl core::ops::Not for $name {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self {
                self.complement()
            }
        }

        impl core::ops::Sub for $name {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                self.difference(rhs)
            }
        }

        impl core::ops::SubAssign for $name {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                *self = self.difference(rhs);
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}(", stringify!($name))?;

                let mut bits = self.bits;
                let mut first = true;

                // Output single-bit flags by name
                for &(name, value) in Self::__FLAGS {
                    // Check if single bit (power of 2)
                    let is_single_bit: bool = value != 0 && (value & (value.wrapping_sub(1))) == 0;
                    if is_single_bit && (bits & value) == value {
                        if !first {
                            write!(f, " | ")?;
                        }
                        write!(f, "{}", name)?;
                        bits &= !value;
                        first = false;
                    }
                }

                // Output remaining unknown bits as hex
                if bits != 0 {
                    if !first {
                        write!(f, " | ")?;
                    }
                    write!(f, "{:#x}", bits)?;
                    first = false;
                }

                // Empty case
                if first {
                    write!(f, "empty")?;
                }

                write!(f, ")")
            }
        }

        impl core::fmt::Binary for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Binary::fmt(&self.bits, f)
            }
        }

        impl core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::LowerHex::fmt(&self.bits, f)
            }
        }

        impl core::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::UpperHex::fmt(&self.bits, f)
            }
        }

        impl core::fmt::Octal for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Octal::fmt(&self.bits, f)
            }
        }
    };
}
