use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let mut listing = false;
    let mut directory_and_size: HashMap<String, usize> = HashMap::new();
    let mut cwd: Vec<&str> = Vec::new();

    for line in input.lines() {
        // println!("{} {}", idx, line);
        let cmd_chars: Vec<char> = line.chars().collect();

        if listing {
            let args = line.split_whitespace().collect::<Vec<&str>>();
            let first_arg: &str = args[0]; // file size
            let second_arg = args[1]; // file name

            if let Ok(file_size) = first_arg.parse::<usize>() {
                let mut acc = String::from("");
                for d in cwd.clone() {
                    acc.push_str("/");
                    acc.push_str(d);
                    directory_and_size
                        .entry(acc.clone())
                        .and_modify(|total_size| *total_size += file_size)
                        .or_insert(file_size);
                }
            }
        }

        if cmd_chars[0] == '$' {
            listing = false;
            // println!("command found {}{}", cmd_chars[2], cmd_chars[3]);
            let cmd = [cmd_chars[2], cmd_chars[3]];

            // cd
            if cmd_chars[2] == 'c' && cmd_chars[3] == 'd' {
                let cmd_args: Vec<&str> = line.split_whitespace().collect();
                let mut dir_name = cmd_args[2];

                if dir_name != ".." {
                    cwd.push(dir_name);
                } else {
                    cwd.pop();
                }
            }

            // ls
            if cmd_chars[2] == 'l' && cmd_chars[3] == 's' {
                listing = true;
            }
        }
    }

    let mut total_big_dir_sum = 0;
    for (k, v) in directory_and_size {
        println!("{k} {v}");

        if v <= 100000 {
            total_big_dir_sum += v;
        }
    }
    println!("Total sum is {}", total_big_dir_sum);
}
