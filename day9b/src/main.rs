fn main() {
    let nums: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    'outer: for (i, _) in nums.iter().enumerate() {
        let mut acc = 0;
        let mut temp_v = vec![];
        'inner: for num in nums[i..nums.len()].iter() {
            acc += num;
            temp_v.push(num);
            if acc > 556543474 {
                break 'inner;
            }

            if acc == 556543474 {
                println!(
                    "{}",
                    **temp_v.iter().max().unwrap() + **temp_v.iter().min().unwrap()
                );
                break 'outer;
            }
        }
    }
}
