use std::collections::HashSet;
use std::iter;
use std::ops::{Add, AddAssign};
use std::path::Path;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

struct Instruction {
    direction: Point,
    steps: usize,
}

fn main() {
    // Read input file
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let (d, s) = line.split_once(' ').expect("doesnt contain a whitespace");
            let dir: Point = match d {
                "R" => Point { x: 1, y: 0 },
                "U" => Point { x: 0, y: -1 },
                "L" => Point { x: -1, y: 0 },
                "D" => Point { x: 0, y: 1 },
                _ => unreachable!(),
            };
            Instruction {
                direction: dir,
                steps: s.parse().unwrap(),
            }
        })
        .collect();

    // Iterate over all instructions
    let mut unique_positions = HashSet::new();
    let mut head_pos = Point { x: 0, y: 0 };
    let mut tail_pos = Point { x: 0, y: 0 };
    unique_positions.insert(tail_pos);
    for delta in instructions
        .into_iter()
        .flat_map(|instr| iter::repeat(instr.direction).take(instr.steps))
    {
        head_pos += delta;
        tail_pos = compute_tail_position(head_pos, tail_pos);
        unique_positions.insert(tail_pos);
    }

    // Output result
    println!("Solution 1: {}", unique_positions.len());
}

fn compute_tail_position(head: Point, tail: Point) -> Point {
    if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
        tail
    } else {
        let dx = (head.x - tail.x).signum();
        let dy = (head.y - tail.y).signum();
        Point {
            x: tail.x + dx,
            y: tail.y + dy,
        }
    }
}
