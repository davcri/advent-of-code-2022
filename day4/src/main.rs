use std::{collections::HashSet, fs, hash::Hash};

fn main() {
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    // let mut priorities_sum = 0;

    let mut fully_contained_assignments = 0;
    for (idx, line) in lines.enumerate() {
        println!("<--");
        println!("{}", line);

        let pair = line.split_once(",").unwrap();
        let first_elf_section_ids = pair.0;
        let second_elf_section_idx = pair.1;

        let first_elf_sections = first_elf_section_ids.split_once("-").unwrap();
        let first_elf_start: u32 = first_elf_sections.0.parse().unwrap();
        let first_elf_end: u32 = first_elf_sections.1.parse().unwrap();

        let second_elf_sections = second_elf_section_idx.split_once("-").unwrap();
        let second_elf_start: u32 = second_elf_sections.0.parse().unwrap();
        let second_elf_end: u32 = second_elf_sections.1.parse().unwrap();

        println!(
            "{} {} | {} {}",
            first_elf_start, first_elf_end, second_elf_start, second_elf_end
        );

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

        println!("-->");
    }
    println!(
        "Fully contained assignments = {}",
        fully_contained_assignments
    );
}
