/*
* each line is a rucksack
* split the line in half for compartments
* check if compartments share an item
* ASCII priority by value
* sum of the priorities
*
* vector to store the char for each line
* for each line hashmap to check repetition
* if char repeated break and push to vector
* next line
*/

use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct ElfGroup {
    rucksack_list: Vec<String>,
}

impl ElfGroup {
    fn new() -> ElfGroup {
        ElfGroup {
            rucksack_list: Vec::new(),
        }
    }
}

pub fn read_input() -> Result<Vec<ElfGroup>, Box<dyn Error>> {
    let f = fs::read_to_string("input.txt").expect("file should be available");

    // create vector of the different groups and start the first group
    let mut elf_groups: Vec<ElfGroup> = Vec::new();
    elf_groups.push(ElfGroup::new());

    let mut counter = 0;

    for rucksack in f.lines() {
        // when an elf group is completed start the next one
        if counter == 3 {
            elf_groups.push(ElfGroup::new());
            counter = 0;
        }
        elf_groups
            .last_mut()
            .expect("vector should never be empty")
            .rucksack_list
            .push(rucksack.to_string());

        counter += 1;
    }
    Ok(elf_groups)
}

pub fn extract_badges(elf_groups: Vec<ElfGroup>) -> Vec<u32> {
    let mut badges = Vec::new();

    for group in elf_groups {
        for item in group.rucksack_list[0].chars() {
            if group.rucksack_list[1].contains(item) & group.rucksack_list[2].contains(item) {
                badges.push(ascii_to_priority(item));
                break;
            }
        }
    }

    badges
}

fn ascii_to_priority(c: char) -> u32 {
    let ascii_val = c as u32;
    let offset: u32 = match c.is_ascii_lowercase() {
        true => 96,
        false => 38,
    };
    ascii_val - offset
}
