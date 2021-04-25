#[derive(Debug)]
pub enum Tile {
    Empty,
    Occupied,
    Floor,
}
#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug)]
pub struct Cell {
    pub point: Point,
    pub status: Tile,
    pub neighbors: Vec<Cell>,
}

impl Cell {
    fn new_floor(pos: Point) -> Self {
        Cell {
            point: pos,
            status: Tile::Floor,
            neighbors: Vec::with_capacity(8),
        }
    }
    fn new_seat(pos: Point) -> Self {
        Cell {
            point: pos,
            status: Tile::Empty,
            neighbors: Vec::with_capacity(8),
        }
    }
}

#[derive(Debug)]
pub struct Floor {
    pub cells: Vec<Vec<Cell>>,
    pub diffs: u16,
    pub rows: u8,
    pub cols: u8,
}

impl Floor {
    pub fn surrounding_cells(&self, pos: &Point) -> Vec<Cell> {
        // if idx is outside bounds dont add cell
        let max_idx = (self.cols * self.cols) - 1;
        let moore_cells: Vec<Cell> = Vec::with_capacity(8);

        vec![]
    }

    pub fn add_border(mut self) -> Self {
        let cols = self.cols as usize;
        self.cells.iter_mut().enumerate().for_each(|(i, vec)| {
            vec.insert(0, Cell::new_floor(Point { x: 0, y: i + 1 }));
            vec.push(Cell::new_floor(Point { x: cols, y: i }));
        });

        let top_row = (0..self.cols)
            .map(|x| {
                Cell::new_floor(Point {
                    x: x as usize,
                    y: 0,
                })
            })
            .collect::<Vec<Cell>>();

        let bottom_row = (0..self.cols)
            .map(|x| {
                Cell::new_floor(Point {
                    x: x as usize,
                    y: self.rows as usize,
                })
            })
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
                            'L' => Cell::new_seat(Point { x: x + 1, y: y + 1 }),
                            '.' => Cell::new_floor(Point { x: x + 1, y: y + 1 }),
                            _ => panic!("Unexpected Char: {}", c),
                        })
                        .collect::<Vec<Cell>>()
                })
                .collect::<_>(),
            diffs: 0,
            rows: (input.lines().count()) as u8,
            cols: ((input.chars().count() - input.lines().count()) / input.lines().count()) as u8,
        }
    }
}
