//! Test integration with libc types.
//! This test is only compiled on Unix platforms where libc is available.

#[cfg(unix)]
use neobit::neobit;

#[cfg(unix)]
neobit! {
    /// Test flags using libc types
    pub struct TestFlags: libc::c_int {
        const A = 0x01;
        const B = 0x02;
        const C = 0x04;
        const ALL = Self::A.union(Self::B).union(Self::C).bits();
    }
}

#[cfg(unix)]
neobit! {
    /// Test file permissions using libc mode_t
    pub struct TestMode: libc::mode_t {
        const RUSR = libc::S_IRUSR;
        const WUSR = libc::S_IWUSR;
        const XUSR = libc::S_IXUSR;
        const RGRP = libc::S_IRGRP;
        const WGRP = libc::S_IWGRP;
        const XGRP = libc::S_IXGRP;
        const ROTH = libc::S_IROTH;
        const WOTH = libc::S_IWOTH;
        const XOTH = libc::S_IXOTH;
        const ALL_READ = Self::RUSR.union(Self::RGRP).union(Self::ROTH).bits();
        const ALL_WRITE = Self::WUSR.union(Self::WGRP).union(Self::WOTH).bits();
        const ALL_EXEC = Self::XUSR.union(Self::XGRP).union(Self::XOTH).bits();
    }
}

#[test]
#[cfg(unix)]
fn test_libc_c_int_basic() {
    let flags = TestFlags::A | TestFlags::B;
    assert!(flags.contains(TestFlags::A));
    assert!(flags.contains(TestFlags::B));
    assert!(!flags.contains(TestFlags::C));

    // Test const operations
    const FLAGS: TestFlags = TestFlags::A.union(TestFlags::B);
    assert_eq!(FLAGS.bits(), 0x03);

    // Test conversion to libc::c_int
    let c_value: libc::c_int = flags.into();
    assert_eq!(c_value, 0x03);

    // Test conversion from libc::c_int
    let flags2: TestFlags = (0x03 as libc::c_int).into();
    assert_eq!(flags, flags2);
}

#[test]
#[cfg(unix)]
fn test_libc_mode_t() {
    let mode = TestMode::RUSR | TestMode::WUSR | TestMode::XUSR;
    assert!(mode.contains(TestMode::RUSR));
    assert!(mode.contains(TestMode::WUSR));
    assert!(mode.contains(TestMode::XUSR));

    // Test const operations
    const USER_PERMS: TestMode = TestMode::RUSR.union(TestMode::WUSR).union(TestMode::XUSR);
    assert_eq!(
        USER_PERMS.bits(),
        libc::S_IRUSR | libc::S_IWUSR | libc::S_IXUSR
    );

    // Test conversion to libc::mode_t
    let mode_value: libc::mode_t = mode.into();
    assert_eq!(mode_value, libc::S_IRUSR | libc::S_IWUSR | libc::S_IXUSR);

    // Test conversion from libc::mode_t
    let mode2: TestMode = (libc::S_IRUSR | libc::S_IWUSR | libc::S_IXUSR).into();
    assert_eq!(mode, mode2);
}

#[test]
#[cfg(unix)]
fn test_libc_const_context() {
    // Test using libc types in const context
    const FLAGS: TestFlags = TestFlags::ALL;
    const MODE: TestMode = TestMode::ALL_READ.union(TestMode::ALL_WRITE);

    assert_eq!(FLAGS.bits(), 0x07);
    assert_eq!(
        MODE.bits(),
        (libc::S_IRUSR
            | libc::S_IWUSR
            | libc::S_IRGRP
            | libc::S_IWGRP
            | libc::S_IROTH
            | libc::S_IWOTH)
    );

    // Test operations in const context
    const DERIVED_FLAGS: TestFlags = TestFlags::ALL.difference(TestFlags::C);
    assert_eq!(DERIVED_FLAGS.bits(), 0x03);
}

#[test]
#[cfg(unix)]
fn test_libc_debug_format() {
    let flags = TestFlags::A | TestFlags::B;
    let debug_str = format!("{:?}", flags);
    assert!(debug_str.contains("A"));
    assert!(debug_str.contains("B"));

    let mode = TestMode::RUSR | TestMode::WUSR;
    let debug_str = format!("{:?}", mode);
    assert!(debug_str.contains("RUSR"));
    assert!(debug_str.contains("WUSR"));
}

#[test]
#[cfg(unix)]
fn test_libc_unknown_bits() {
    // Test that unknown bits are preserved when using libc types
    let raw_value: libc::c_int = 0x80; // Unknown bit
    let flags: TestFlags = raw_value.into();

    assert_eq!(flags.bits(), 0x80);
    assert!(!flags.contains(TestFlags::A));
    assert!(!flags.contains(TestFlags::B));
    assert!(!flags.contains(TestFlags::C));

    // Test combining known and unknown bits
    let flags2 = flags | TestFlags::A;
    assert_eq!(flags2.bits(), 0x81);
    assert!(flags2.contains(TestFlags::A));
}

#[test]
#[cfg(unix)]
fn test_libc_signed_operations() {
    // Test operations with signed libc types
    let flags = TestFlags::A;

    // Test complement (should work with signed types)
    let complement = flags.complement();
    assert_eq!(complement.bits(), !0x01 as libc::c_int);

    // Test difference (preferred way to remove flags)
    let all = TestFlags::ALL;
    let without_a = all.difference(TestFlags::A);
    assert_eq!(without_a.bits(), 0x06);
    assert!(!without_a.contains(TestFlags::A));
    assert!(without_a.contains(TestFlags::B));
    assert!(without_a.contains(TestFlags::C));
}

#[test]
#[cfg(not(unix))]
fn test_libc_not_unix() {
    // This test ensures the test suite runs on non-Unix platforms
    // even though the actual libc tests are skipped
    assert!(true);
}
