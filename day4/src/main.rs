use std::{cmp, fs};

fn main() {
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let mut fully_contained_assignments = 0;
    let mut overlapping_assignments = 0;

    for line in lines {
        let pair = line.split_once(",").unwrap();
        let first_elf_section_ids = pair.0;
        let second_elf_section_idx = pair.1;

        let first_elf_sections = first_elf_section_ids.split_once("-").unwrap();
        let first_elf_start: u32 = first_elf_sections.0.parse().unwrap();
        let first_elf_end: u32 = first_elf_sections.1.parse().unwrap();

        let second_elf_sections = second_elf_section_idx.split_once("-").unwrap();
        let second_elf_start: u32 = second_elf_sections.0.parse().unwrap();
        let second_elf_end: u32 = second_elf_sections.1.parse().unwrap();

        let first_elf_sections_count = first_elf_end - first_elf_start + 1;
        let second_elf_sections_count = second_elf_end - second_elf_start + 1;

        if first_elf_sections_count > second_elf_sections_count {
            if first_elf_start <= second_elf_start && first_elf_end >= second_elf_end {
                fully_contained_assignments += 1;
            }
        } else {
            if second_elf_start <= first_elf_start && second_elf_end >= first_elf_end {
                fully_contained_assignments += 1;
            }
        }

        let bigger_start = cmp::max(first_elf_start, second_elf_start);
        let smaller_end = cmp::min(first_elf_end, second_elf_end);
        if bigger_start <= smaller_end {
            overlapping_assignments += 1;
        }
    }
    println!(
        "Fully contained assignments = {}",
        fully_contained_assignments
    );
    println!("Overlapping assignments = {}", overlapping_assignments);
}
