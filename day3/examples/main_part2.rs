use std::collections::{HashMap, HashSet};
use std::fs;

fn get_item_priority(item: char) -> u32 {
    if item.is_uppercase() {
        return item as u32 - 64 + 26;
    } else {
        return item as u32 - 64 - 32;
    }
}

// elves are in group of 3
// the badge is the only item type shared between 3 racksacks
// found the badge item type for each group (of three elves)
// calculate the sum of these item types

fn main() {
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let mut priorities_sum = 0;
    let mut group_count = 0;

    let mut a: HashSet<char> = HashSet::new();
    let mut b: HashSet<char> = HashSet::new();
    let mut c: HashSet<char> = HashSet::new();

    let mut idx = 0;
    for (bag_content) in lines {
        // skip empty lines
        if bag_content == "" {
            continue;
        }

        let current_elf_idx_in_group = idx % 3;

        for (item) in bag_content.chars() {
            if current_elf_idx_in_group == 0 {
                a.insert(item);
            } else if current_elf_idx_in_group == 1 {
                b.insert(item);
            } else if current_elf_idx_in_group == 2 {
                c.insert(item);
            }
        }

        if current_elf_idx_in_group == 2 {
            group_count += 1;
            println!("-- group {} --", group_count);
            let ac: HashSet<char> = a.iter().cloned().collect();
            let bc: HashSet<char> = b.iter().cloned().collect();
            let cc: HashSet<char> = c.iter().cloned().collect();
            // intersect items sets
            let mut items_in_common: HashSet<char>;
            items_in_common = &ac & &bc;
            items_in_common = &items_in_common & &cc;
            assert!(items_in_common.len() == 1);

            // reset
            a = HashSet::new();
            b = HashSet::new();
            c = HashSet::new();

            let mut badge_item: Option<char> = None;
            badge_item = Some(Vec::from_iter(items_in_common)[0]);

            if let Some(found_badge_item) = badge_item {
                let item_priority = get_item_priority(found_badge_item);
                println!("badge {} ", found_badge_item);
                priorities_sum += item_priority;
            } else {
                panic!("ERROR: badge item not found.");
            }
        }
        idx += 1;
    }
    println!("PRIORITIES SUM = {}", priorities_sum);
}
