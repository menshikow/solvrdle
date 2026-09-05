const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = solvrdle::algorithms::Naive::new();
        solvrdle::play(answer, guesser);
    }
}
