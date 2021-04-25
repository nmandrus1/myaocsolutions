fn main() {
    let mut counter: u32 = 0;
    let v: Vec<String> = include_str!("input.txt")
        .lines()
        .map(|s| s.to_owned())
        .collect();

    for string in v {
        let tokens: Vec<String> = string.trim().split(' ').map(|s| s.to_owned()).collect();
        //println!("{:?}", tokens);

        let bounds: Vec<usize> = tokens[0]
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let ch = tokens[1].chars().nth(0).unwrap();

        if tokens[2].matches(ch).count() >= bounds[0] && tokens[2].matches(ch).count() <= bounds[1]
        {
            counter += 1;
        }
    }

    println!("{}", counter);
}

// nwwwwwqwwwwwtww
