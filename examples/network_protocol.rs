//! Network protocol parsing example.
//!
//! Demonstrates using neobit for protocol flags in a TCP-like header.

use neobit::neobit;

neobit! {
    /// TCP flags from a packet header
    pub struct TcpFlags: u8 {
        const FIN = 0b0000_0001;  // Finish
        const SYN = 0b0000_0010;  // Synchronize
        const RST = 0b0000_0100;  // Reset
        const PSH = 0b0000_1000;  // Push
        const ACK = 0b0001_0000;  // Acknowledgment
        const URG = 0b0010_0000;  // Urgent
        const ECE = 0b0100_0000;  // ECN Echo
        const CWR = 0b1000_0000;  // Congestion Window Reduced
    }
}

neobit! {
    /// HTTP request method flags (custom protocol)
    pub struct HttpMethods: u16 {
        const GET     = 1 << 0;
        const POST    = 1 << 1;
        const PUT     = 1 << 2;
        const DELETE  = 1 << 3;
        const PATCH   = 1 << 4;
        const HEAD    = 1 << 5;
        const OPTIONS = 1 << 6;
    }
}

/// Simulated TCP packet
struct TcpPacket {
    flags: TcpFlags,
    #[allow(dead_code)]
    data: Vec<u8>,
}

impl TcpPacket {
    fn is_syn_ack(&self) -> bool {
        self.flags == (TcpFlags::SYN | TcpFlags::ACK)
    }

    fn is_connection_close(&self) -> bool {
        self.flags.contains(TcpFlags::FIN) || self.flags.contains(TcpFlags::RST)
    }
}

fn main() {
    println!("=== TCP Protocol Flags ===\n");

    // Three-way handshake
    let syn = TcpFlags::SYN;
    println!("1. Client sends SYN: {:?}", syn);

    let syn_ack = TcpFlags::SYN | TcpFlags::ACK;
    println!("2. Server responds SYN-ACK: {:?}", syn_ack);

    let ack = TcpFlags::ACK;
    println!("3. Client sends ACK: {:?}", ack);

    // Data transfer
    let psh_ack = TcpFlags::PSH | TcpFlags::ACK;
    println!("\n4. Data transfer PSH-ACK: {:?}", psh_ack);

    // Connection close
    let fin_ack = TcpFlags::FIN | TcpFlags::ACK;
    println!("5. Close connection FIN-ACK: {:?}", fin_ack);

    // Parsing raw packet flags
    println!("\n=== Parsing Raw Flags ===\n");

    let raw_flags: u8 = 0b0001_0010; // SYN + ACK
    let parsed = TcpFlags::from_bits_retain(raw_flags);
    println!("Raw 0b0001_0010 parsed as: {:?}", parsed);

    // Validate flags
    match TcpFlags::from_bits(raw_flags) {
        Some(flags) => println!("Valid flags: {:?}", flags),
        None => println!("Invalid flags contain unknown bits"),
    }

    // Unknown bits handling
    let invalid_raw: u8 = 0xFF; // All bits set
    let retained = TcpFlags::from_bits_retain(invalid_raw);
    println!("\nRetaining all bits 0xFF: {:?}", retained);

    match TcpFlags::from_bits(invalid_raw) {
        Some(flags) => println!("Valid: {:?}", flags),
        None => println!("Invalid: contains unknown bits"),
    }

    // Packet inspection
    println!("\n=== Packet Inspection ===\n");

    let packet = TcpPacket {
        flags: TcpFlags::SYN | TcpFlags::ACK,
        data: vec![],
    };

    println!("Packet flags: {:?}", packet.flags);
    println!("Is SYN-ACK? {}", packet.is_syn_ack());
    println!("Is closing? {}", packet.is_connection_close());

    // HTTP methods example
    println!("\n=== HTTP Methods ===\n");

    // Safe methods (idempotent)
    let safe_methods = HttpMethods::GET | HttpMethods::HEAD | HttpMethods::OPTIONS;
    println!("Safe methods: {:?}", safe_methods);

    // Unsafe methods
    let unsafe_methods =
        HttpMethods::POST | HttpMethods::PUT | HttpMethods::DELETE | HttpMethods::PATCH;
    println!("Unsafe methods: {:?}", unsafe_methods);

    // Check if method is allowed
    let allowed = HttpMethods::GET | HttpMethods::POST;
    println!("\nAllowed methods: {:?}", allowed);
    println!("GET allowed? {}", allowed.contains(HttpMethods::GET));
    println!("DELETE allowed? {}", allowed.contains(HttpMethods::DELETE));

    // All methods
    let all_methods = HttpMethods::all();
    println!("\nAll HTTP methods: {:?}", all_methods);
}
