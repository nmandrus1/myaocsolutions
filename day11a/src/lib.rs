extern crate termion;
use termion::{clear, color, cursor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Occupied,
    Floor,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub point: Point,
    pub status: Tile,
    pub marked: bool,
}

impl Cell {
    fn new_floor(x: usize, y: usize) -> Self {
        Cell {
            point: Point { x, y },
            status: Tile::Floor,
            marked: false,
        }
    }
    fn new_seat(x: usize, y: usize) -> Self {
        Cell {
            point: Point { x, y },
            status: Tile::Empty,
            marked: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Floor {
    pub cells: Vec<Vec<Cell>>,
    pub diffs: u16,
    pub rows: u8,
    pub cols: u8,
}

impl Floor {
    pub fn draw(&mut self) {
        println!("{}", clear::All);
        self.cells.iter().for_each(|cell_vec| {
            cell_vec.iter().for_each(|cell| match cell.status {
                Tile::Empty => print!(
                    "{}{}L",
                    cursor::Goto(cell.point.x as u16 + 1, cell.point.y as u16 + 1),
                    color::Fg(color::Green)
                ),
                Tile::Floor => print!(
                    "{}{}.",
                    cursor::Goto(cell.point.x as u16 + 1, cell.point.y as u16 + 1),
                    color::Fg(color::LightWhite)
                ),
                Tile::Occupied => print!(
                    "{}{}#",
                    cursor::Goto(cell.point.x as u16 + 1, cell.point.y as u16 + 1),
                    color::Fg(color::Red)
                ),
            });
            println!("");
        });
    }

    pub fn update(&mut self) {
        self.diffs = 0;
        let (rows, cols) = (self.rows, self.cols);
        (1..(rows - 1) as usize)
            .flat_map(|y| (1..(cols - 1) as usize).map(move |x| (x, y)))
            .for_each(|(x, y)| {
                if self.surrounding_cells(&self.cells[y][x]) == 0
                    && self.cells[y][x].status == Tile::Empty
                {
                    self.cells[y][x].marked = true;
                    self.diffs += 1;
                } else if self.surrounding_cells(&self.cells[y][x]) > 3
                    && self.cells[y][x].status == Tile::Occupied
                {
                    self.cells[y][x].marked = true;
                    self.diffs += 1;
                }
            });

        self.cells
            .iter_mut()
            .flatten()
            .filter(|c| c.marked)
            .for_each(|c| match c.status {
                Tile::Occupied => {
                    c.status = Tile::Empty;
                    c.marked = false
                }
                Tile::Empty => {
                    c.status = Tile::Occupied;
                    c.marked = false
                }
                Tile::Floor => (),
            });
    }

    pub fn surrounding_cells(&self, cell: &Cell) -> u8 {
        ((cell.point.y - 1)..=(cell.point.y + 1))
            .flat_map(|y| ((cell.point.x - 1)..=(cell.point.x + 1)).map(move |x| &self.cells[y][x]))
            .filter(move |c| c.status == Tile::Occupied && *c != cell)
            .count() as u8
    }

    pub fn add_border(mut self) -> Self {
        self.cols += 2;
        self.rows += 2;
        let cols = self.cols as usize;
        self.cells.iter_mut().enumerate().for_each(|(i, vec)| {
            vec.insert(0, Cell::new_floor(0, i + 1));
            vec.push(Cell::new_floor(cols - 1, i + 1));
        });

        let top_row = (0..self.cols)
            .map(|x| Cell::new_floor(x as usize, 0))
            .collect::<Vec<Cell>>();

        let bottom_row = (0..self.cols)
            .map(|x| Cell::new_floor(x as usize, (self.rows - 1) as usize))
            .collect::<Vec<Cell>>();

        self.cells.insert(0, top_row);
        self.cells.push(bottom_row);
        self
    }

    pub fn new_from_in(input: &str) -> Self {
        Floor {
            cells: input
                .lines()
                .enumerate()
                .map(|(y, s)| {
                    s.chars()
                        .enumerate()
                        .map(move |(x, c)| match c {
                            'L' => Cell::new_seat(x + 1, y + 1),
                            '.' => Cell::new_floor(x + 1, y + 1),
                            _ => panic!("Unexpected Char: {}", c),
                        })
                        .collect::<Vec<Cell>>()
                })
                .collect::<_>(),
            diffs: u16::MAX,
            rows: (input.lines().count()) as u8,
            cols: ((input.chars().count() - input.lines().count()) / input.lines().count()) as u8,
        }
    }
}
