use std::fmt;

fn main() {
    // let input_file = include_str!("sample.txt");
    let input_file = include_str!("input.txt");
    let (instructions, total_cycles_required) = parse_instructions(input_file);
    let out1 = solve_part_1(instructions, total_cycles_required);

    println!("{}", out1);
}

fn solve_part_1(instructions: Vec<Op>, total_cycles_required: i32) -> i32 {
    let mut ops_completion_queue: Vec<(Op, i32)> = Vec::new();
    for (i, op) in instructions.iter().enumerate() {
        match op.code {
            OpCode::NOOP => {
                ops_completion_queue.push((op.clone(), 1));
            }
            OpCode::ADDX => {
                ops_completion_queue.push((op.clone(), 2));
            }
        }
    }

    let mut cpu = CPU { registerX: 1 };
    let mut some_of_signal_strengths = 0;

    let mut ops_iter = ops_completion_queue.iter_mut();
    let mut cur_cycle = 0;
    loop {
        let op_res = ops_iter.next();
        if op_res.is_none() {
            break;
        } else {
            let (op, required_cycles) = op_res.unwrap();
            while *required_cycles > 0 {
                cur_cycle += 1;
                *required_cycles -= 1;
                // println!("Cycle {}", cur_cycle);
                if (cur_cycle == 20) || ((cur_cycle + 20) % 40 == 0) {
                    let signal_strength = cur_cycle * cpu.registerX;
                    println!(
                        "check signal strentgh at cycle {} = {}",
                        cur_cycle, signal_strength
                    );
                    some_of_signal_strengths += signal_strength;
                }
            }
            // operation completed
            match op.code {
                OpCode::NOOP => {}
                OpCode::ADDX => {
                    cpu.registerX += op.arg.unwrap();
                }
            }
            // println!(
            //     "Cycle {} | Op {} completed. CPU [x = {}]",
            //     cur_cycle, op, cpu.registerX
            // );
        }
    }

    return some_of_signal_strengths;
}

fn parse_instructions(input_file: &str) -> (Vec<Op>, i32) {
    let mut ops = Vec::new();
    let mut total_cycles_required = 0;
    for line in input_file.lines() {
        let mut splits = line.split_whitespace();
        let split0 = splits.next().unwrap();
        let opcode = OpCode::from_str(split0);
        let split1 = splits.next();
        if split1.is_some() {
            let arg = split1.unwrap().parse::<i32>().unwrap();
            let op = Op {
                code: opcode,
                arg: Some(arg),
            };
            total_cycles_required += 2;
            ops.push(op);
        } else {
            let op = Op {
                code: opcode,
                arg: None,
            };
            total_cycles_required += 1;
            ops.push(op);
        }
    }
    return (ops, total_cycles_required);
}
struct CPU {
    registerX: i32,
}

#[derive(Clone)]
enum OpCode {
    NOOP,
    ADDX,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut op_str = "";
        match self {
            Self::NOOP => op_str = "noop",
            Self::ADDX => op_str = "adds",
            _ => panic!(),
        }
        return write!(f, "{}", op_str);
    }
}

impl OpCode {
    pub(crate) fn from_str(arg: &str) -> OpCode {
        match arg {
            "noop" => OpCode::NOOP,
            "addx" => OpCode::ADDX,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
struct Op {
    code: OpCode,
    arg: Option<i32>,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.arg.is_some() {
            return write!(f, "{} {}", self.code, self.arg.unwrap());
        } else {
            return write!(f, "{}", self.code);
        }
    }
}
