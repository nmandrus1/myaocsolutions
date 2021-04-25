const LETTERS: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];
fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .split("\n\n")
            .map(|s| { LETTERS.iter().filter(|&letter| s.contains(letter)).count() })
            .sum::<usize>()
    );
}
