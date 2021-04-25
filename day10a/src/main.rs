fn main() {
    let mut vec: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();
    vec.sort();

    let num = vec
        .windows(2)
        .fold((1usize, 1usize), |acc, i| match i[1] - i[0] {
            1 => (acc.0 + 1, acc.1),
            _ => (acc.0, acc.1 + 1),
        });

    println!("{}", num.0 * num.1);
}
