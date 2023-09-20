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

pub fn read_input() -> Result<Vec<u32>, Box<dyn Error>> {
    let f = fs::read_to_string("input.txt").expect("file should be available");

    let mut duplicates: Vec<u32> = Vec::new();

    for rucksack in f.lines() {
        let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
        for item in comp1.chars() {
            if comp2.contains(item) {
                duplicates.push(ascii_to_priority(item));
                break;
            }
        }
    }

    Ok(duplicates)
}

fn ascii_to_priority(c: char) -> u32 {
    let ascii_val = c as u32;
    let offset: u32 = match c.is_ascii_lowercase() {
        true => 96,
        false => 38,
    };
    ascii_val - offset
}
