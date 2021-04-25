struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn go(&mut self) -> &Position {
        self.x += 3;
        self.y += 1;
        self
    }
}

fn main() {
    let mut pos = Position { x: 0, y: 0 };
    println!(
        "{}",
        include_str!("input.txt")
            .lines()
            .skip(1)
            .filter(|s| {
                pos.go();
                s.to_owned().chars().nth(pos.x % s.len()).unwrap() == '#'
            })
            .count()
    );
}
