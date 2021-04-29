use std::{thread, time};

mod lib;
use lib::{Floor, Tile};

fn main() {
    let input = include_str!("input_test.txt");
    //let input = include_str!("input.txt");
    let mut floor = Floor::new_from_in(input).add_border();

    animate(&mut floor);
}

fn solution(floor: &mut Floor) {
    while floor.diffs != 0 {
        floor.update();
    }
    println!(
        "{}",
        floor
            .cells
            .iter()
            .flatten()
            .filter(|c| c.status == Tile::Occupied)
            .count()
    )
}

fn animate(floor: &mut Floor) {
    while floor.diffs != 0 {
        thread::sleep(time::Duration::from_millis(50));
        floor.update();
        thread::sleep(time::Duration::from_millis(50));
        floor.draw();
    }
}
