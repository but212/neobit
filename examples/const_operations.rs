//! Const operations example.
//!
//! Demonstrates using neobit in const contexts for compile-time flag operations.

use neobit::neobit;

neobit! {
    /// Feature flags for a software package
    pub struct Features: u32 {
        const LOGGING     = 1 << 0;
        const METRICS     = 1 << 1;
        const TRACING     = 1 << 2;
        const PROFILING   = 1 << 3;
        const DEBUGGING   = 1 << 4;
        const VALIDATION  = 1 << 5;
    }
}

// Define composite constants at compile time
const DEBUG_MODE: Features = Features::LOGGING
    .union(Features::DEBUGGING)
    .union(Features::VALIDATION);

const PRODUCTION_MODE: Features = Features::LOGGING.union(Features::METRICS);

const DEVELOPMENT_MODE: Features = Features::LOGGING
    .union(Features::DEBUGGING)
    .union(Features::VALIDATION)
    .union(Features::TRACING);

const PERFORMANCE_PROFILING: Features = Features::METRICS
    .union(Features::PROFILING)
    .union(Features::TRACING);

// All features enabled
const ALL_FEATURES: Features = Features::all();

// No features
const NO_FEATURES: Features = Features::empty();

neobit! {
    /// Hardware register flags
    pub struct RegisterFlags: u8 {
        const ENABLE  = 1 << 0;
        const RESET   = 1 << 1;
        const IRQ     = 1 << 2;
        const DMA     = 1 << 3;
    }
}

// Create const masks for hardware configuration
const HW_INIT: RegisterFlags = RegisterFlags::RESET;
const HW_NORMAL: RegisterFlags = RegisterFlags::ENABLE;
const HW_IRQ_MODE: RegisterFlags = RegisterFlags::ENABLE.union(RegisterFlags::IRQ);
const HW_DMA_MODE: RegisterFlags = RegisterFlags::ENABLE
    .union(RegisterFlags::DMA)
    .union(RegisterFlags::IRQ);

impl Features {
    /// Check if this is a production-safe configuration
    pub const fn is_production_safe(self) -> bool {
        // Production should not have debugging or profiling
        !self.intersects(Features::DEBUGGING.union(Features::PROFILING))
    }

    /// Check if observability is enabled
    pub const fn has_observability(self) -> bool {
        self.intersects(
            Features::LOGGING
                .union(Features::METRICS)
                .union(Features::TRACING),
        )
    }
}

fn main() {
    println!("=== Const Feature Configurations ===\n");

    println!("Debug mode:       {:?}", DEBUG_MODE);
    println!("Production mode:  {:?}", PRODUCTION_MODE);
    println!("Development mode: {:?}", DEVELOPMENT_MODE);
    println!("Profiling mode:   {:?}", PERFORMANCE_PROFILING);
    println!("All features:     {:?}", ALL_FEATURES);
    println!("No features:      {:?}", NO_FEATURES);

    println!("\n=== Feature Checks ===\n");

    println!(
        "Production is safe? {}",
        PRODUCTION_MODE.is_production_safe()
    );
    println!("Debug is safe? {}", DEBUG_MODE.is_production_safe());
    println!(
        "Production has observability? {}",
        PRODUCTION_MODE.has_observability()
    );

    println!("\n=== Hardware Register Configurations ===\n");

    println!("Init state:   {:?}", HW_INIT);
    println!("Normal state: {:?}", HW_NORMAL);
    println!("IRQ mode:     {:?}", HW_IRQ_MODE);
    println!("DMA mode:     {:?}", HW_DMA_MODE);

    // Runtime operations
    println!("\n=== Runtime Operations ===\n");

    let mut current_features = PRODUCTION_MODE;
    println!("Starting with: {:?}", current_features);

    // Add tracing for debugging an issue
    current_features.insert(Features::TRACING);
    println!("After adding tracing: {:?}", current_features);

    // Remove all debugging features
    let debug_features = Features::DEBUGGING | Features::VALIDATION;
    current_features.remove(debug_features);
    println!("After removing debug features: {:?}", current_features);

    // Toggle validation
    current_features.toggle(Features::VALIDATION);
    println!("After toggling validation: {:?}", current_features);

    println!("\n=== Const Operations ===\n");

    // Demonstrate const operations
    const INTERSECTION: Features = DEBUG_MODE.intersection(DEVELOPMENT_MODE);
    println!("Debug ∩ Development: {:?}", INTERSECTION);

    const DIFFERENCE: Features = DEVELOPMENT_MODE.difference(DEBUG_MODE);
    println!("Development - Debug: {:?}", DIFFERENCE);

    const SYMMETRIC_DIFF: Features = DEBUG_MODE.symmetric_difference(PRODUCTION_MODE);
    println!("Debug △ Production: {:?}", SYMMETRIC_DIFF);

    // Bitwise complement (all bits flipped)
    const COMPLEMENT: Features = DEBUG_MODE.complement();
    println!("¬Debug (bitwise): {:?}", COMPLEMENT);

    println!("\n=== Runtime Assertions ===");

    // These checks are performed at runtime (const assert! requires nightly)
    assert!(PRODUCTION_MODE.is_production_safe());
    assert!(PRODUCTION_MODE.has_observability());
    assert!(!NO_FEATURES.has_observability());

    println!("All assertions passed! ✓");
}
