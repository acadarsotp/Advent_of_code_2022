/*
* solve -> wich elf has the most calories, print his calories
* group data -> vector of calories for each elf
* struct elf { list: Vec<i32>, sum: i32 }
* closure to get the max?
*/
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Elf {
    pub food_list: Vec<i32>,
}

pub fn read_input() -> Result<Vec<Elf>, Box<dyn Error>> {
    // Create the elf list
    let mut elf_list = Vec::new();
    // Insert the first elf because the input does not start with empty line
    elf_list.push(Elf {
        food_list: Vec::new(),
    });

    let f = fs::read_to_string("input.txt")?;
    for line in f.lines() {
        if line.is_empty() {
            // go to the next elf
            elf_list.push(Elf {
                food_list: Vec::new(),
            })
        } else {
            // add food to the elfs inventory
            elf_list
                .last_mut()
                .expect("the vector should never be empty")
                .food_list
                .push(line.parse::<i32>().expect("input should be parseable"));
        }
    }
    Ok(elf_list)
}
