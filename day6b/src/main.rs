const LETTERS: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];
fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .split("\n\n")
            .map(|string| {
                LETTERS
                    .iter()
                    .filter(|&ch| string.lines().all(|s| s.contains(ch)))
                    .count()
            })
            .sum::<usize>()
    );
}
