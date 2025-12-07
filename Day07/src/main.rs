use std::fs;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    // fn add(&self, other: &Position) -> Position {
    //     Position {
    //         row: self.row + other.row,
    //         col: self.col + other.col,
    //     }
    // }

    fn char_at(&self, field: &Vec<&str>) -> char {
        // field[self.row as usize].chars().nth(self.col as usize).unwrap_or('\0')
        let line = field.get(self.row as usize).unwrap_or(&"\0");
        line.chars().nth(self.col as usize).unwrap_or('\0')
    }

    fn is_in(&self, dequeue: &VecDeque<Position>) -> bool {
        for pos in dequeue {
            if pos.row == self.row && pos.col == self.col {
                return true;
            }
        }
        false
    }

    fn down(&self) -> Position
    {
        Position {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left(&self) -> Position
    {
        Position {
            row: self.row,
            col: self.col - 1
        }
    }

    fn right(&self) -> Position
    {
        Position {
            row: self.row,
            col: self.col + 1,
        }
    }
}

fn locate_start(lines: &Vec<&str>) -> Option<(usize, usize)> {
    for (i, line) in lines.iter().enumerate() {
        if let Some(j) = line.find('S') {
            return Some((i, j));
        }
    }
    None
}

fn simulate_beams(field: &Vec<&str>, start: Position) {
    let mut positions: VecDeque<Position> = VecDeque::new();
    positions.push_back(start);
    let mut splits: i128 = 0;

    while !positions.is_empty() {
        let pos = positions.pop_front().unwrap();
        // println!("At position: ({}, {}) in field {}", pos.row, pos.col, pos.char_at(field));


        if pos.down().char_at(field) == '^' {
            let left = pos.down().left();
            let right = pos.down().right();
            // let down = pos.down();
            // println!("Splitting beam ({},{}) to ({},{}), ({},{})", down.row, down.col, left.row, left.col, right.row, right.col);

            if !left.is_in(&positions) && !pos.left().is_in(&positions) {
                positions.push_back(left);
            }
            if !right.is_in(&positions) && !pos.right().is_in(&positions) {
                positions.push_back(right);
            }

            splits += 1;
        }
        else {
            let d = pos.down();
            if d.char_at(field) == '\0' {
                // println!("Beam exited the field at ({}, {})", d.row, d.col);
                continue;
            }
            positions.push_back(pos.down());
        }
        // println!("Beams remaining: {:?}", positions/* .len() */);
    }

    println!("Total splits: {}", splits);
}

fn pt1()
{
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("Cannot read the file");
    contents.pop();

    let lines: Vec<&str> = contents.split('\n').collect();
    if let Some((row, col)) = locate_start(&lines) {
        let start = Position { row: row as i32, col: col as i32 };
        simulate_beams(&lines, start);
    }
}

fn pt2() {
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("Cannot read the file");
    contents.pop();

    let manifold: Vec<&str> = contents.trim().lines().collect();

    if manifold.is_empty() {
        return;
    }

    let mut beams: HashMap<usize, i64> = HashMap::new();

    // Find starting position 'S'
    if let Some(start_pos) = manifold[0].find('S') {
        beams.insert(start_pos, 1);
    }

    let mut p1 = 0;

    for row in &manifold[1..] {
        let mut new_beams: HashMap<usize, i64> = HashMap::new();

        for (&x, &c) in &beams {
            // println!("Processing position {} with {} beams", x, c);

            if x < row.len() && row.chars().nth(x) == Some('^') {
                *new_beams.entry(x.saturating_sub(1)).or_insert(0) += c;
                *new_beams.entry(x + 1).or_insert(0) += c;
                p1 += 1;
            } else {
                *new_beams.entry(x).or_insert(0) += c;
            }
        }

        beams = new_beams;
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", beams.values().sum::<i64>());
}

fn main()
{
    pt1();
    pt2();

}
