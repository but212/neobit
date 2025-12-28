//! Example demonstrating operators and methods
//!
//! Shows all available operators and their const fn equivalents.

use neobit::neobit;

neobit! {
    pub struct Flags: u8 {
        const A = 0b001;
        const B = 0b010;
        const C = 0b100;
    }
}

fn main() {
    println!("=== Operators ===");

    // Union (|)
    let a_or_b = Flags::A | Flags::B;
    println!("A | B = {:?}", a_or_b);

    // Intersection (&)
    let ab = Flags::A | Flags::B;
    let a_and_ab = Flags::A & ab;
    println!("A & (A | B) = {:?}", a_and_ab);

    // Symmetric difference (^)
    let a_xor_b = Flags::A ^ Flags::B;
    println!("A ^ B = {:?}", a_xor_b);

    // Complement (!)
    let not_a = !Flags::A;
    println!("!A = {:?}", not_a);

    // Difference (-)
    let all_minus_a = Flags::all() - Flags::A;
    println!("All - A = {:?}", all_minus_a);

    println!("\n=== Methods (const fn equivalents) ===");

    // These can be used in const contexts!
    const UNION_AB: Flags = Flags::A.union(Flags::B);
    const INTERSECTION: Flags = Flags::A.intersection(UNION_AB);
    const SYMMETRIC_DIFF: Flags = Flags::A.symmetric_difference(Flags::B);
    const COMPLEMENT: Flags = Flags::A.complement();
    const DIFFERENCE: Flags = Flags::all().difference(Flags::A);

    println!("A.union(B) = {:?}", UNION_AB);
    println!("A.intersection(A | B) = {:?}", INTERSECTION);
    println!("A.symmetric_difference(B) = {:?}", SYMMETRIC_DIFF);
    println!("A.complement() = {:?}", COMPLEMENT);
    println!("All.difference(A) = {:?}", DIFFERENCE);

    println!("\n=== Assignment operators ===");

    let mut flags = Flags::A;
    println!("Start: {:?}", flags);

    flags |= Flags::B;
    println!("After |= B: {:?}", flags);

    flags &= Flags::A | Flags::B;
    println!("After &= (A | B): {:?}", flags);

    flags ^= Flags::C;
    println!("After ^= C: {:?}", flags);

    flags -= Flags::A;
    println!("After -= A: {:?}", flags);

    println!("\n=== Other methods ===");

    let flags = Flags::A | Flags::B;

    // Access methods
    println!("bits(): {}", flags.bits());
    println!("is_empty(): {}", flags.is_empty());
    println!("contains(A): {}", flags.contains(Flags::A));
    println!("contains(C): {}", flags.contains(Flags::C));
    println!("intersects(C): {}", flags.intersects(Flags::C));
    println!(
        "intersects(A | C): {}",
        flags.intersects(Flags::A | Flags::C)
    );

    // Mutation methods
    let mut flags = Flags::empty();
    flags.insert(Flags::A);
    println!("After insert(A): {:?}", flags);

    flags.remove(Flags::A);
    flags.insert(Flags::B);
    println!("After remove(A), insert(B): {:?}", flags);

    flags.toggle(Flags::A);
    println!("After toggle(A): {:?}", flags);

    // Conditional set
    flags.set(Flags::C, true);
    println!("After set(C, true): {:?}", flags);

    flags.set(Flags::B, false);
    println!("After set(B, false): {:?}", flags);

    println!("\nAll examples passed!");
}
