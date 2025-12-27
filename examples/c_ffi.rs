//! C FFI example demonstrating signed integer bitflags with libc.
//!
//! This example shows how neobit can be used with libc types
//! for compatibility with C libraries.

use neobit::neobit;

neobit! {
    /// File open flags compatible with libc.
    pub struct OpenFlags: libc::c_int {
        const RDONLY   = libc::O_RDONLY;
        const WRONLY   = libc::O_WRONLY;
        const RDWR     = libc::O_RDWR;
        const CREAT    = libc::O_CREAT;
        const TRUNC    = libc::O_TRUNC;
        const APPEND   = libc::O_APPEND;
    }
}

#[cfg(unix)]
neobit! {
    /// File permission flags compatible with libc (Unix only).
    pub struct FileMode: libc::mode_t {
        const RUSR = libc::S_IRUSR;
        const WUSR = libc::S_IWUSR;
        const XUSR = libc::S_IXUSR;
        const RGRP = libc::S_IRGRP;
        const WGRP = libc::S_IWGRP;
        const XGRP = libc::S_IXGRP;
        const ROTH = libc::S_IROTH;
        const WOTH = libc::S_IWOTH;
        const XOTH = libc::S_IXOTH;
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

        // Example: calling libc::open (unsafe)
        let path = std::ffi::CString::new("/tmp/neobit_test.txt").unwrap();
        let fd = unsafe { libc::open(path.as_ptr(), flags.bits(), mode.bits()) };

        if fd >= 0 {
            println!("File opened successfully, fd = {}", fd);
            unsafe { libc::close(fd) };
        } else {
            println!("Failed to open file");
        }

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
