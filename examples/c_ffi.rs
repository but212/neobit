//! C FFI example demonstrating signed integer bitflags without libc.
//!
//! This example shows how neobit can be used with C-compatible types
//! for compatibility with C libraries.

use neobit::neobit;

// C-compatible type definitions
type CInt = i32;

#[cfg(unix)]
type ModeT = u32;

// Common open flags (POSIX values)
const O_RDONLY: CInt = 0;
const O_WRONLY: CInt = 1;
const O_RDWR: CInt = 2;
const O_CREAT: CInt = 0o100;
const O_TRUNC: CInt = 0o1000;
const O_APPEND: CInt = 0o2000;

#[cfg(unix)]
mod mode_constants {
    use super::ModeT;
    pub const S_IRUSR: ModeT = 0o400;
    pub const S_IWUSR: ModeT = 0o200;
    pub const S_IXUSR: ModeT = 0o100;
    pub const S_IRGRP: ModeT = 0o040;
    pub const S_IWGRP: ModeT = 0o020;
    pub const S_IXGRP: ModeT = 0o010;
    pub const S_IROTH: ModeT = 0o004;
    pub const S_IWOTH: ModeT = 0o002;
    pub const S_IXOTH: ModeT = 0o001;
}

neobit! {
    /// File open flags compatible with C.
    pub struct OpenFlags: CInt {
        const RDONLY   = O_RDONLY;
        const WRONLY   = O_WRONLY;
        const RDWR     = O_RDWR;
        const CREAT    = O_CREAT;
        const TRUNC    = O_TRUNC;
        const APPEND   = O_APPEND;
    }
}

#[cfg(unix)]
neobit! {
    /// File permission flags compatible with C (Unix only).
    pub struct FileMode: ModeT {
        const RUSR = mode_constants::S_IRUSR;
        const WUSR = mode_constants::S_IWUSR;
        const XUSR = mode_constants::S_IXUSR;
        const RGRP = mode_constants::S_IRGRP;
        const WGRP = mode_constants::S_IWGRP;
        const XGRP = mode_constants::S_IXGRP;
        const ROTH = mode_constants::S_IROTH;
        const WOTH = mode_constants::S_IWOTH;
        const XOTH = mode_constants::S_IXOTH;
    }
}

fn main() {
    // Building flags for open() syscall
    let flags = OpenFlags::WRONLY | OpenFlags::CREAT | OpenFlags::TRUNC;

    println!("Open flags: {:?}", flags);
    println!("Raw flags: {:#x}", flags.bits());

    #[cfg(unix)]
    {
        let mode = FileMode::RUSR | FileMode::WUSR | FileMode::RGRP | FileMode::ROTH;
        println!("File mode: {:?}", mode);
        println!("Raw mode: {:#o}", mode.bits());

        // Checking mode
        println!("Mode contains WUSR? {}", mode.contains(FileMode::WUSR));
    }

    #[cfg(not(unix))]
    {
        println!("File mode operations are only available on Unix platforms");
    }

    // Checking flags
    println!(
        "\nFlags contain CREAT? {}",
        flags.contains(OpenFlags::CREAT)
    );

    // Modifying flags
    let mut flags = flags;
    flags.insert(OpenFlags::APPEND);
    flags.remove(OpenFlags::TRUNC);
    println!("Modified flags: {:?}", flags);
}
