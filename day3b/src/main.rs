struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn go(&mut self, rise: usize, run: usize) -> &Position {
        self.x += run;
        self.y += rise;
        self
    }
}

fn sled(rise: usize, run: usize) -> usize {
    let mut pos = Position { x: 0, y: 0 };
    include_str!("input.txt")
        .lines()
        .skip(rise)
        .step_by(rise)
        .filter(|s| {
            pos.go(rise, run);
            s.to_owned().chars().nth(pos.x % s.len()).unwrap() == '#'
        })
        .count()
}

fn main() {
    println!(
        "{}",
        sled(1, 1) * sled(1, 3) * sled(1, 5) * sled(1, 7) * sled(2, 1)
    );
}
