//! Composite flags and common patterns.
//!
//! Demonstrates creating composite constants and common flag patterns.

use neobit::neobit;

neobit! {
    /// File system permissions (Unix-style)
    pub struct Mode: u16 {
        // Owner permissions
        const OWNER_READ    = 0o400;
        const OWNER_WRITE   = 0o200;
        const OWNER_EXECUTE = 0o100;

        // Group permissions
        const GROUP_READ    = 0o040;
        const GROUP_WRITE   = 0o020;
        const GROUP_EXECUTE = 0o010;

        // Other permissions
        const OTHER_READ    = 0o004;
        const OTHER_WRITE   = 0o002;
        const OTHER_EXECUTE = 0o001;

        // Special bits
        const SETUID        = 0o4000;
        const SETGID        = 0o2000;
        const STICKY        = 0o1000;
    }
}

// Define common composite patterns outside the macro
impl Mode {
    // Common permission sets
    pub const OWNER_RWX: Self = Self::OWNER_READ
        .union(Self::OWNER_WRITE)
        .union(Self::OWNER_EXECUTE);

    pub const OWNER_RW: Self = Self::OWNER_READ.union(Self::OWNER_WRITE);

    pub const GROUP_RWX: Self = Self::GROUP_READ
        .union(Self::GROUP_WRITE)
        .union(Self::GROUP_EXECUTE);

    pub const GROUP_RW: Self = Self::GROUP_READ.union(Self::GROUP_WRITE);

    pub const GROUP_RX: Self = Self::GROUP_READ.union(Self::GROUP_EXECUTE);

    pub const OTHER_RX: Self = Self::OTHER_READ.union(Self::OTHER_EXECUTE);

    // Standard chmod patterns
    pub const MODE_0755: Self = Self::OWNER_RWX.union(Self::GROUP_RX).union(Self::OTHER_RX); // rwxr-xr-x

    pub const MODE_0644: Self = Self::OWNER_RW
        .union(Self::GROUP_READ)
        .union(Self::OTHER_READ); // rw-r--r--

    pub const MODE_0600: Self = Self::OWNER_RW; // rw-------

    pub const MODE_0777: Self = Self::OWNER_RWX
        .union(Self::GROUP_RWX)
        .union(Self::OTHER_READ)
        .union(Self::OTHER_WRITE)
        .union(Self::OTHER_EXECUTE); // rwxrwxrwx

    /// Format as traditional Unix permission string
    pub fn as_permission_string(&self) -> String {
        let mut s = String::with_capacity(10);

        // File type (always '-' for this example)
        s.push('-');

        // Owner
        s.push(if self.contains(Self::OWNER_READ) {
            'r'
        } else {
            '-'
        });
        s.push(if self.contains(Self::OWNER_WRITE) {
            'w'
        } else {
            '-'
        });
        s.push(if self.contains(Self::OWNER_EXECUTE) {
            if self.contains(Self::SETUID) {
                's'
            } else {
                'x'
            }
        } else {
            if self.contains(Self::SETUID) {
                'S'
            } else {
                '-'
            }
        });

        // Group
        s.push(if self.contains(Self::GROUP_READ) {
            'r'
        } else {
            '-'
        });
        s.push(if self.contains(Self::GROUP_WRITE) {
            'w'
        } else {
            '-'
        });
        s.push(if self.contains(Self::GROUP_EXECUTE) {
            if self.contains(Self::SETGID) {
                's'
            } else {
                'x'
            }
        } else {
            if self.contains(Self::SETGID) {
                'S'
            } else {
                '-'
            }
        });

        // Other
        s.push(if self.contains(Self::OTHER_READ) {
            'r'
        } else {
            '-'
        });
        s.push(if self.contains(Self::OTHER_WRITE) {
            'w'
        } else {
            '-'
        });
        s.push(if self.contains(Self::OTHER_EXECUTE) {
            if self.contains(Self::STICKY) {
                't'
            } else {
                'x'
            }
        } else {
            if self.contains(Self::STICKY) {
                'T'
            } else {
                '-'
            }
        });

        s
    }
}

neobit! {
    /// Request handling options
    pub struct RequestOptions: u32 {
        const CACHE         = 1 << 0;
        const COMPRESS      = 1 << 1;
        const ENCRYPT       = 1 << 2;
        const AUTHENTICATE  = 1 << 3;
        const VALIDATE      = 1 << 4;
        const LOG           = 1 << 5;
        const RETRY         = 1 << 6;
        const TIMEOUT       = 1 << 7;
    }
}

impl RequestOptions {
    // Common request profiles
    pub const PUBLIC_API: Self = Self::CACHE
        .union(Self::COMPRESS)
        .union(Self::VALIDATE)
        .union(Self::LOG);

    pub const SECURE_API: Self = Self::ENCRYPT
        .union(Self::AUTHENTICATE)
        .union(Self::VALIDATE)
        .union(Self::LOG);

    pub const INTERNAL_API: Self = Self::CACHE.union(Self::COMPRESS);

    pub const CRITICAL_REQUEST: Self = Self::AUTHENTICATE
        .union(Self::VALIDATE)
        .union(Self::LOG)
        .union(Self::RETRY)
        .union(Self::TIMEOUT);
}

fn main() {
    println!("=== Unix File Permissions ===\n");

    // Common permission patterns
    println!(
        "Standard file (0644):      {} ({:04o})",
        Mode::MODE_0644.as_permission_string(),
        Mode::MODE_0644.bits()
    );

    println!(
        "Executable (0755):         {} ({:04o})",
        Mode::MODE_0755.as_permission_string(),
        Mode::MODE_0755.bits()
    );

    println!(
        "Private file (0600):       {} ({:04o})",
        Mode::MODE_0600.as_permission_string(),
        Mode::MODE_0600.bits()
    );

    println!(
        "Full access (0777):        {} ({:04o})",
        Mode::MODE_0777.as_permission_string(),
        Mode::MODE_0777.bits()
    );

    // Build custom permissions
    println!("\n=== Custom Permissions ===\n");

    let script = Mode::OWNER_RWX | Mode::GROUP_RX;
    println!(
        "Script file:               {} ({:04o})",
        script.as_permission_string(),
        script.bits()
    );

    let config = Mode::OWNER_RW | Mode::GROUP_READ;
    println!(
        "Config file:               {} ({:04o})",
        config.as_permission_string(),
        config.bits()
    );

    // Special permissions
    println!("\n=== Special Permissions ===\n");

    let setuid = Mode::MODE_0755 | Mode::SETUID;
    println!(
        "Setuid executable:         {} ({:04o})",
        setuid.as_permission_string(),
        setuid.bits()
    );

    let setgid = Mode::MODE_0755 | Mode::SETGID;
    println!(
        "Setgid directory:          {} ({:04o})",
        setgid.as_permission_string(),
        setgid.bits()
    );

    let sticky = Mode::MODE_0777 | Mode::STICKY;
    println!(
        "Sticky directory (/tmp):   {} ({:04o})",
        sticky.as_permission_string(),
        sticky.bits()
    );

    // Modify permissions
    println!("\n=== Permission Modification ===\n");

    let mut mode = Mode::MODE_0644;
    println!(
        "Start:                     {} ({:04o})",
        mode.as_permission_string(),
        mode.bits()
    );

    // chmod +x (add execute for owner)
    mode.insert(Mode::OWNER_EXECUTE);
    println!(
        "After chmod +x:            {} ({:04o})",
        mode.as_permission_string(),
        mode.bits()
    );

    // Add group write
    mode.insert(Mode::GROUP_WRITE);
    println!(
        "After chmod g+w:           {} ({:04o})",
        mode.as_permission_string(),
        mode.bits()
    );

    // Remove other permissions
    mode.remove(Mode::OTHER_READ);
    println!(
        "After chmod o-r:           {} ({:04o})",
        mode.as_permission_string(),
        mode.bits()
    );

    println!("\n=== Request Options ===\n");

    println!("Public API:      {:?}", RequestOptions::PUBLIC_API);
    println!("Secure API:      {:?}", RequestOptions::SECURE_API);
    println!("Internal API:    {:?}", RequestOptions::INTERNAL_API);
    println!("Critical:        {:?}", RequestOptions::CRITICAL_REQUEST);

    // Customize based on context
    println!("\n=== Custom Request Configuration ===\n");

    let mut opts = RequestOptions::PUBLIC_API;
    println!("Base (public):   {:?}", opts);

    // Upgrade to secure
    opts.insert(RequestOptions::ENCRYPT | RequestOptions::AUTHENTICATE);
    println!("Upgraded:        {:?}", opts);

    // Disable caching for fresh data
    opts.remove(RequestOptions::CACHE);
    println!("No cache:        {:?}", opts);

    // Check configuration
    println!("\n=== Checking Configuration ===\n");
    println!("Is encrypted? {}", opts.contains(RequestOptions::ENCRYPT));
    println!("Is cached? {}", opts.contains(RequestOptions::CACHE));
    println!(
        "Is secure? {}",
        opts.contains(RequestOptions::ENCRYPT.union(RequestOptions::AUTHENTICATE))
    );

    println!("\n=== Composite Const Evaluation ===\n");

    // These are all computed at compile time
    const _CHECK1: Mode = Mode::MODE_0755;
    const _CHECK2: RequestOptions = RequestOptions::SECURE_API;

    println!("All composite constants evaluated at compile time! ✓");
}
