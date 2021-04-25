fn main() {
    let mut counter: usize = 0;
    let v: Vec<String> = include_str!("input.txt")
        .lines()
        .map(|s| s.to_owned())
        .collect();

    for string in v {
        let tokens: Vec<String> = string.trim().split(' ').map(|s| s.to_owned()).collect();

        let positions: Vec<usize> = tokens[0]
            .split('-')
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();

        let ch = tokens[1].chars().nth(0).unwrap();

        if ch == tokens[2].chars().nth(positions[0]).unwrap()
            && ch == tokens[2].chars().nth(positions[1]).unwrap()
        {
            continue;
        } else if ch == tokens[2].chars().nth(positions[0]).unwrap()
            || ch == tokens[2].chars().nth(positions[1]).unwrap()
        {
            counter += 1;
        }
    }

    println!("{}", counter);
}
