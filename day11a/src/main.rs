extern crate termion;
use termion::clear;

use std::{thread, time};

mod lib;
use lib::{Cell, Floor, Point, Tile};

fn main() {
    let input = include_str!("input_test.txt");
    let mut floor = Floor::new_from_in(input).add_border();

    println!("{}", termion::clear::All);
    floor.draw();
    floor.update();
    thread::sleep(time::Duration::from_millis(200));
    println!("{}", termion::clear::All);
    floor.draw();

    //println!("{:#?}", floor);
}
