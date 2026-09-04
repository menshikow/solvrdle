fn main() {
    println!("Hello, world!");
}

fn play<G: Guesser>(answer: &'static str, guesser: G) {
    // Play six rounds where it invokes the guesser each round.
}
