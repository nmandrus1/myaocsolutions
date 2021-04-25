const REQ_FIELDS: [&str; 7] = ["byr", "eyr", "iyr", "hgt", "hcl", "ecl", "pid"];
fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .split("\n\n")
            .filter(|s| { REQ_FIELDS.iter().all(|i| s.contains(i)) })
            .count()
    );
}
