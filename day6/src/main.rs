use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = include_str!("input.txt");

    let v: Vec<char> = input.chars().collect();
    let array_len = v.len();
    let mut idx = 0;
    let window_size = 14;

    for ch in &v {
        if idx + window_size >= array_len {
            break;
        }
        let mut window_chars: Vec<char> = Vec::with_capacity(window_size);
        for window_idx in 0..window_size {
            let cur_char = v[idx + window_idx];
            window_chars.push(cur_char);
            print!("{} ", cur_char);
        }
        println!();

        // https://stackoverflow.com/questions/39803237/build-hashset-from-a-vector-in-rust
        let set: HashSet<char> = HashSet::from_iter(window_chars); // TODO: do not allocate memory in a for loop, this is waste of memory
        if set.len() == window_size {
            println!("Found {}", idx + window_size);
            break;
        }
        idx += 1;
    }

    println!();
}
