use std::collections::HashMap;
use std::fs;

fn getItemPriority(item: char) -> u32 {
    if item.is_uppercase() {
        return item as u32 - 64 + 26;
    } else {
        return item as u32 - 64 - 32;
    }
}

fn main() {
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut priorities_sum = 0;

    for bag_content in lines {
        let mut items_count = 0;
        for _ in bag_content.chars() {
            items_count += 1;
        }
        assert_eq!(items_count % 2, 0, "itemsCount {} is not even", items_count);

        // find the item that appears in both compartments
        let mut first_compartment = HashMap::new();
        let mut second_compartment = HashMap::new();
        let mut item_in_both_compartments: Option<char> = None;

        for (idx, item) in bag_content.chars().enumerate() {
            if idx < items_count / 2 {
                first_compartment
                    .entry(item)
                    .and_modify(|item_count| *item_count += 1)
                    .or_insert(1);
            } else {
                if first_compartment.contains_key(&item) {
                    item_in_both_compartments = Some(item);
                    break;
                }
                second_compartment
                    .entry(item)
                    .and_modify(|item_count| *item_count += 1)
                    .or_insert(1);
            }
        }

        if let Some(found_item) = item_in_both_compartments {
            let item_priority = getItemPriority(found_item);
            priorities_sum += item_priority;
            println!("Item in both compartments = {}", found_item);
            println!("Item priority {}", item_priority);
        } else {
            panic!("ERROR: item not found.");
        }

        println!("PRIORITIES SUM = {}", priorities_sum);
    }
}
