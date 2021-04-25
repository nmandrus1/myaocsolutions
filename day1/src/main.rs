use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let mut v: Vec<u32> = read("input.txt").expect("Failed to open file");
    v.sort();

    println!("{}", find_solution1(&v));
    println!("{}", find_solution2(&v));
}

fn read(path: &str) -> Result<Vec<u32>, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let mut v = Vec::new();

    for line in br.lines() {
        let n: u32 = line?.trim().parse().unwrap();
        v.push(n);
    }

    Ok(v)
}

/*** MY SOLUTIONS ***/

fn find_solution2(v: &Vec<u32>) -> u32 {
    for index in 0..v.len() {
        for value1 in index + 1..v.len() {
            for value2 in value1 + 1..v.len() {
                if &v[index] + &v[value1] + &v[value2] == 2020 {
                    return &v[index] * &v[value1] * &v[value2];
                }
            }
        }
    }
    0
}

fn find_solution1(v: &Vec<u32>) -> u32 {
    for index in v {
        for value in v {
            if index + value == 2020 {
                return index * value;
            }
        }
    }

    0
}

/*** BETTER SOLUTIONS ***/
