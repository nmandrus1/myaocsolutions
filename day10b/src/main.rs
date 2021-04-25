fn main() {
    let mut vec: Vec<usize> = include_str!("../input_test.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    vec.sort();

    println!(
        "{}",
        vec.iter().enumerate().fold(0, |acc, (i, num)| {
            println!("{}", acc);
            acc + vec[i..vec.len()]
                .iter()
                .filter(|elem| **elem - *num < 4)
                .count()
        })
    );
}
