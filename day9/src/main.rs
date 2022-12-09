use core::fmt;
use std::str::FromStr;

fn main() {
    let input_file = include_str!("input.txt");
    let steps = parse_input(input_file);
    let walked_cells = solve_part_1(steps);
    println!("{} ", walked_cells);
}

fn solve_part_1(steps: Vec<Step>) -> usize {
    let mut coords_tail: Vec<Coord> = Vec::new();
    let mut head_coord = Coord { x: 0, y: 0 }; // current coord
    let mut tail_coord = Coord { x: 0, y: 0 }; // current coord

    // record starting position
    coords_tail.push(Coord {
        x: head_coord.x,
        y: head_coord.y,
    });

    for s in steps {
        // println!("---{s}");
        for _ in 0..s.count {
            // move head
            let mut new_head_coord = Coord {
                x: head_coord.x,
                y: head_coord.y,
            };
            match s.dir {
                Dir::Left => {
                    new_head_coord.x -= 1;
                }
                Dir::Right => {
                    new_head_coord.x += 1;
                }
                Dir::Up => {
                    new_head_coord.y -= 1;
                }
                Dir::Down => {
                    new_head_coord.y += 1;
                }
            }
            head_coord.x = new_head_coord.x;
            head_coord.y = new_head_coord.y;

            let diff = Coord {
                x: head_coord.x - tail_coord.x,
                y: head_coord.y - tail_coord.y,
            };
            // If the head is ever two steps directly up, down, left, or right
            // from the tail, the tail must also move one step in that direction
            // so it remains close enough:
            if diff.y == 0 {
                if diff.x.abs() == 2 {
                    tail_coord.x += diff.x.signum();
                }
            } else if diff.x == 0 {
                if diff.y.abs() == 2 {
                    tail_coord.y += diff.y.signum();
                }
            }
            // Otherwise, if the head and tail aren't touching and aren't in the
            // same row or column, the tail always moves one step diagonally to
            // keep up:
            if diff.x.abs() + diff.y.abs() > 2
                && tail_coord.x != head_coord.x
                && tail_coord.y != head_coord.y
            {
                tail_coord.x += diff.x.signum();
                tail_coord.y += diff.y.signum();
            }

            // println!("h{} t{}", head_coord, tail_coord);

            if !coords_tail.contains(&tail_coord) {
                coords_tail.push(Coord {
                    x: tail_coord.x,
                    y: tail_coord.y,
                });
            }
        }
    }

    return coords_tail.len();
}

#[derive(PartialEq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str_repr = "";
        str_repr = match self {
            Dir::Left => "L",
            Dir::Right => "R",
            Dir::Up => "U",
            Dir::Down => "D",
            _ => "_ERR_",
        };
        return write!(f, "{}", str_repr);
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "R" => Ok(Dir::Right),
            "L" => Ok(Dir::Left),
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            _ => Err(()),
        };
    }
}

struct Step {
    dir: Dir,
    count: u32,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Step {} {} times", self.dir, self.count);
    }
}

fn parse_input(input_file: &str) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();

    for line in input_file.lines() {
        let mut line_args = line.split_whitespace();
        let dir = line_args.next().unwrap();
        let steps_count = line_args.next().unwrap().parse::<u32>().unwrap();
        steps.push(Step {
            dir: Dir::from_str(dir).unwrap(),
            count: steps_count,
        })
    }

    return steps;
}
