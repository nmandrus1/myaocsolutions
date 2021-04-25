use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let mut bags: HashSet<String> = ["shiny gold".to_string()].iter().cloned().collect();
    let mut temp: usize = 0;

    loop {
        temp = bags.len();
        bags = make_hashset(&bags, input);

        if temp == bags.len() {
            break;
        }
    }

    println!("{}", temp - 1);
}

fn make_hashset(bags: &HashSet<String>, input: &str) -> HashSet<String> {
    bags.union(&bags_that_hold_these(&bags, input))
        .map(|s| s.to_string())
        .collect::<HashSet<String>>()
}

fn bags_that_hold_these<'a>(bags_hashset: &HashSet<String>, input: &'a str) -> HashSet<String> {
    let mut return_hashset = HashSet::new();

    bags_hashset.iter().for_each(|bag| {
        input
            .lines()
            .filter(|line| line.contains(bag) && !line.starts_with(bag))
            .map(|line| make_bag_from_line(line))
            .for_each(|bag_from_line| {
                return_hashset.insert(bag_from_line);
                ()
            })
    });

    return_hashset
}

fn make_bag_from_line(line: &str) -> String {
    let v: Vec<&str> = line.split_ascii_whitespace().collect();
    format!("{} {}", v[0], v[1].trim_end_matches('s'))
}
