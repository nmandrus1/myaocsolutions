#![feature(str_split_once)]

use std::collections::HashMap;
const REQ_FIELDS: [&str; 7] = ["byr", "eyr", "iyr", "hgt", "hcl", "ecl", "pid"];
const ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .split("\n\n")
            .map(|fields| fields
                .split_ascii_whitespace()
                .map(|field| field.split_once(":").unwrap())
                .collect::<HashMap<_, _>>())
            .filter(|passport| REQ_FIELDS.iter().all(|item| passport.contains_key(item)))
            .filter(|passport| passport.iter().all(|(f, v)| validate_field(f, v)))
            .count()
    )
}

fn validate_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => 1920 <= value.parse::<usize>().unwrap() && value.parse::<usize>().unwrap() <= 2002,
        "eyr" => 2020 <= value.parse::<usize>().unwrap() && value.parse::<usize>().unwrap() <= 2030,
        "iyr" => 2010 <= value.parse::<usize>().unwrap() && value.parse::<usize>().unwrap() <= 2020,
        "hgt" => {
            if value.contains("cm") {
                println!("{}", value);
                150 <= value
                    .trim_matches(char::is_alphabetic)
                    .parse::<usize>()
                    .unwrap()
                    && value
                        .trim_matches(char::is_alphabetic)
                        .parse::<usize>()
                        .unwrap()
                        <= 193
            } else if value.contains("in") {
                59 <= value
                    .trim_matches(char::is_alphabetic)
                    .parse::<usize>()
                    .unwrap()
                    && value
                        .trim_matches(char::is_alphabetic)
                        .parse::<usize>()
                        .unwrap()
                        <= 76
            } else {
                false
            }
        }
        "hcl" => value.starts_with('#') && value.len() == 7,
        "ecl" => ECL.iter().any(|ecl| value.contains(ecl)),
        "pid" => value.len() == 9,
        "cid" => true,
        _ => panic!("Unexpected Value"),
    }
}
