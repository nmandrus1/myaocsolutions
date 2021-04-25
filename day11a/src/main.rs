mod lib;
use lib::{Cell, Floor, Point, Tile};

fn main() {
    let input = include_str!("input_test.txt");
    let mut floor = Floor::new_from_in(input).add_border();

    //println!("{}x{}", floor.rows, floor.cols);
    println!("{:#?}", floor);
}

// test
