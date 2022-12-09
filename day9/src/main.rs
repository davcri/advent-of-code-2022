use core::fmt;
use std::str::FromStr;

fn main() {
    let input_file = include_str!("sample.txt");
    let steps = parse_input(input_file);

    for s in &steps {
        println!("{s}");
    }
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
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
