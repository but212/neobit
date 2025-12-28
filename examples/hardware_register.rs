//! Hardware register manipulation example.
//!
//! Demonstrates using neobit for embedded systems and hardware control.

use neobit::neobit;

neobit! {
    /// GPIO pin configuration register (example STM32-like)
    pub struct GpioConfig: u32 {
        const MODE_INPUT   = 0b00 << 0;   // Bits 0-1: Mode
        const MODE_OUTPUT  = 0b01 << 0;
        const MODE_ALT_FN  = 0b10 << 0;
        const MODE_ANALOG  = 0b11 << 0;

        const PULL_NONE    = 0b00 << 2;   // Bits 2-3: Pull-up/down
        const PULL_UP      = 0b01 << 2;
        const PULL_DOWN    = 0b10 << 2;

        const SPEED_LOW    = 0b00 << 4;   // Bits 4-5: Speed
        const SPEED_MEDIUM = 0b01 << 4;
        const SPEED_HIGH   = 0b10 << 4;
        const SPEED_VHIGH  = 0b11 << 4;

        const OPEN_DRAIN   = 1 << 6;      // Bit 6: Output type (absence implies push-pull)
    }
}

neobit! {
    /// SPI control register flags
    pub struct SpiControl: u16 {
        const ENABLE       = 1 << 0;   // SPE: SPI Enable
        const MASTER       = 1 << 2;   // MSTR: Master selection
        const BIDIMODE     = 1 << 15;  // Bidirectional mode
        const BIDIOE       = 1 << 14;  // Output enable in bidirectional mode
        const CRCEN        = 1 << 13;  // CRC enable
        const CRCNEXT      = 1 << 12;  // CRC next
        const DFF          = 1 << 11;  // Data frame format
        const RXONLY       = 1 << 10;  // Receive only
        const SSM          = 1 << 9;   // Software slave management
        const SSI          = 1 << 8;   // Internal slave select
        const LSBFIRST     = 1 << 7;   // Frame format
        const BR_2         = 0b000 << 3;  // Baud rate divider
        const BR_4         = 0b001 << 3;
        const BR_8         = 0b010 << 3;
        const BR_16        = 0b011 << 3;
        const CPOL         = 1 << 1;   // Clock polarity
        const CPHA         = 1 << 0;   // Clock phase
    }
}

neobit! {
    /// DMA control register
    pub struct DmaControl: u32 {
        const ENABLE       = 1 << 0;
        const TCIE         = 1 << 1;   // Transfer complete interrupt enable
        const HTIE         = 1 << 2;   // Half transfer interrupt enable
        const TEIE         = 1 << 3;   // Transfer error interrupt enable
        const DIR_M2P      = 1 << 4;   // Memory-to-peripheral (absence implies peripheral-to-memory)
        const CIRC         = 1 << 5;   // Circular mode
        const PINC         = 1 << 6;   // Peripheral increment
        const MINC         = 1 << 7;   // Memory increment
        const PSIZE_16     = 0b01 << 8;
        const PSIZE_32     = 0b10 << 8;
        const MSIZE_16     = 0b01 << 10;
        const MSIZE_32     = 0b10 << 10;
    }
}

/// Simulate writing to a hardware register
fn write_register(name: &str, value: u32) {
    println!(
        "[HW] Writing to {}: {:#010x} ({:#034b})",
        name, value, value
    );
}

fn main() {
    println!("=== GPIO Configuration ===\n");

    // Configure GPIO as output with pull-up, high speed (push-pull is default, no flag needed)
    let gpio_output = GpioConfig::MODE_OUTPUT | GpioConfig::PULL_UP | GpioConfig::SPEED_HIGH;

    write_register("GPIOA_MODER", gpio_output.bits());
    println!("Config: {:?}\n", gpio_output);

    // Configure GPIO as input with pull-down
    let gpio_input = GpioConfig::MODE_INPUT | GpioConfig::PULL_DOWN;

    write_register("GPIOB_MODER", gpio_input.bits());
    println!("Config: {:?}\n", gpio_input);

    // Analog input (ADC)
    let gpio_analog = GpioConfig::MODE_ANALOG | GpioConfig::PULL_NONE;
    write_register("GPIOC_MODER", gpio_analog.bits());
    println!("Config: {:?}\n", gpio_analog);

    println!("\n=== SPI Configuration ===\n");

    // Configure SPI as master, MSB first, baud rate /8
    let spi_master = SpiControl::ENABLE
        | SpiControl::MASTER
        | SpiControl::SSM
        | SpiControl::SSI
        | SpiControl::BR_8;

    write_register("SPI1_CR1", spi_master.bits() as u32);
    println!("SPI Config: {:?}\n", spi_master);

    // SPI Mode 3 (CPOL=1, CPHA=1)
    let spi_mode3 = spi_master | SpiControl::CPOL | SpiControl::CPHA;
    write_register("SPI1_CR1", spi_mode3.bits() as u32);
    println!("SPI Mode 3: {:?}\n", spi_mode3);

    // Check specific configuration
    println!("Is master mode? {}", spi_mode3.contains(SpiControl::MASTER));
    println!("Is LSB first? {}", spi_mode3.contains(SpiControl::LSBFIRST));

    println!("\n=== DMA Configuration ===\n");

    // Configure DMA for memory-to-peripheral transfer
    // 32-bit size, memory increment, circular mode, interrupts enabled
    let dma_config = DmaControl::ENABLE
        | DmaControl::DIR_M2P
        | DmaControl::CIRC
        | DmaControl::MINC
        | DmaControl::PSIZE_32
        | DmaControl::MSIZE_32
        | DmaControl::TCIE
        | DmaControl::TEIE;

    write_register("DMA1_CCR1", dma_config.bits());
    println!("DMA Config: {:?}\n", dma_config);

    // Modify configuration: disable interrupts
    let mut dma_no_int = dma_config;
    dma_no_int.remove(DmaControl::TCIE | DmaControl::TEIE | DmaControl::HTIE);
    write_register("DMA1_CCR1", dma_no_int.bits());
    println!("DMA without interrupts: {:?}\n", dma_no_int);

    println!("\n=== Safe Register Read-Modify-Write ===\n");

    // Simulate reading from a register
    let current_spi = SpiControl::from_bits_retain(0x0305); // Simulated register value
    println!("Current SPI register: {:?}", current_spi);

    // Modify only specific bits (change baud rate) using a mask
    let mut new_spi = current_spi;
    // Clear all baud rate bits first (BR_2 is 0b000, so we need to clear the field)
    new_spi.remove(SpiControl::BR_4 | SpiControl::BR_8 | SpiControl::BR_16);
    new_spi.insert(SpiControl::BR_16);

    println!("Modified SPI (new baud rate): {:?}", new_spi);
    write_register("SPI1_CR1", new_spi.bits() as u32);

    println!("\n=== Unknown Bits Handling ===\n");

    // Hardware might have reserved or undocumented bits set
    let raw_register: u32 = 0x8000_0001; // Some unknown bits set

    // from_bits_retain preserves all bits (important for hardware!)
    let preserved = GpioConfig::from_bits_retain(raw_register);
    println!("Preserved all bits: {:#010x}", preserved.bits());

    // from_bits validates (returns None for unknown bits)
    match GpioConfig::from_bits(raw_register) {
        Some(config) => println!("Valid config: {:?}", config),
        None => {
            println!("Register contains unknown/reserved bits - preserved with from_bits_retain")
        }
    }
}
