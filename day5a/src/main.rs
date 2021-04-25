fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .lines()
            .map(|s| {
                (i32::from_str_radix(str_to_binary(&s[0..7]).as_str(), 2).unwrap() * 8)
                    + i32::from_str_radix(str_to_binary(&s[7..s.len()]).as_str(), 2).unwrap()
            })
            .max()
            .unwrap()
    );
}

fn str_to_binary(s: &str) -> String {
    let mut string = String::new();

    for ch in s.chars() {
        if ch == 'F' || ch == 'L' {
            string.push('0');
        } else {
            string.push('1');
        }
    }
    string
}
