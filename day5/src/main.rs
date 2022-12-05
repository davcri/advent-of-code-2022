const ROW_IDX_CRATES_STACK: usize = 8;
const CRATE_LANES_COUNT: usize = 9;

fn main() {
    let input = include_str!("input.txt");

    let mut lines = input.lines();
    let mut idx = 0;
    let mut lanes = vec![vec![]; CRATE_LANES_COUNT];
    let mut moves: Vec<usize> = Vec::new();

    while let Some(line) = lines.next() {
        if idx == ROW_IDX_CRATES_STACK {
        } else if idx > ROW_IDX_CRATES_STACK {
            // parsing moves
            if line != "" {
                for ch in line.split_whitespace() {
                    if ch != "" {
                        if let Ok(val) = ch.parse::<usize>() {
                            moves.push(val);
                        }
                    }
                }
            }
        } else if idx < ROW_IDX_CRATES_STACK {
            // parsing stacks
            let items = line.chars();
            let mut char_idx = 0;

            for c in items {
                let current_stack_idx = char_idx / 4;
                if char_idx != 0 && (char_idx) % 4 == 1 {
                    if c != ' ' {
                        lanes[current_stack_idx].push(c);
                    }
                    // print!("[{} {}] ", c, current_stack_idx);
                }
                char_idx += 1;
            }
            // while let Some(item) = items.next() {
            //     print!("[{char_idx} {item}] ");
            //     char_idx += 1;
            // }
            // println!();
        }

        idx += 1;
    }

    let mut reversed_lanes = vec![vec![]; CRATE_LANES_COUNT];
    for (i, line) in lanes.iter().enumerate() {
        let mut rev_lane = line.clone();
        rev_lane.reverse();
        reversed_lanes[i] = rev_lane;
    }

    assert!(moves.len() % 3 == 0, "moves array should be multiple of 3");
    for idx in (0..moves.len() / 3) {
        let moves_count = moves[idx * 3];
        let from_stack = moves[idx * 3 + 1] - 1;
        let to_stack = moves[idx * 3 + 2] - 1;

        let mut crates_group: Vec<char> = Vec::new();
        for _ in 0..moves_count {
            let val = reversed_lanes[from_stack].pop();
            crates_group.push(val.unwrap());
        }
        for _ in 0..crates_group.len() {
            reversed_lanes[to_stack].push(crates_group.pop().unwrap());
        }
    }

    for lane in reversed_lanes.clone().iter_mut() {
        print!("{}", lane.pop().unwrap());
    }
    println!();
}
