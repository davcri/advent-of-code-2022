use std::env;
use std::fs;

// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html
// 71502 max calories

fn main() {
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let mut elves = Vec::new();
    let mut totalCaloriesForElf = 0;
    let mut idx = 0;
    let mut maxCalories: i32 = 0;

    for line in lines {
        if line == "" {
            idx += 1;
            // print!("Elf {} has {} calories.\n\n", idx, totalCaloriesForElf);
            elves.push(totalCaloriesForElf);
            maxCalories = if totalCaloriesForElf > maxCalories {
                totalCaloriesForElf
            } else {
                maxCalories
            };

            totalCaloriesForElf = 0;
        } else {
            let calories = line.parse::<i32>().unwrap();
            // print!("{}\n", calories);
            totalCaloriesForElf += calories;
        }
    }

    if (totalCaloriesForElf != 0) {
        idx += 1;
        // print!("Elf {} has {} calories.\n\n", idx, totalCaloriesForElf);
        elves.push(totalCaloriesForElf);
        totalCaloriesForElf = 0;

        maxCalories = if totalCaloriesForElf > maxCalories {
            totalCaloriesForElf
        } else {
            maxCalories
        };
    }
    // println!("MAX CALORIES = {}", maxCalories);

    elves.sort();
    elves.reverse();
    let mut topThreeCaloriesSum = 0;
    for i in 0..3 {
        topThreeCaloriesSum += elves[i];
        println!("{} {}", i + 1, elves[i]);
    }
    println!("topThreeCaloriesSum {}", topThreeCaloriesSum)
    // println!("MAX CALORIES = {}", maxCalories);
}
