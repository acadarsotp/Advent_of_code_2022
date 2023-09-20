/*
* number of spaces until first -> start stack -> split ' '?
*
* read the input to the stacks by pushing each crate and then reversing
*
* impl move fn to pop and push accordingly
*
* get the top crates
*/

use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Stacks {
    pub stack_vec: Vec<Vec<String>>,
}

#[derive(Debug)]
pub struct Instructions {
    pub instructions_vec: Vec<Vec<u32>>,
}

pub fn get_stacks() -> Result<Stacks, Box<dyn Error>> {
    let f = fs::read_to_string("input.txt").expect("input provided by problem");
    let mut stacks = Stacks {
        stack_vec: Vec::new(),
    };

    for line in f.lines() {
        // check for end of stacks
        if line.chars().nth(1) == Some('1') {
            break;
        }

        let leading_blanks = line.chars().take_while(|c| c.is_whitespace()).count();
        let starting_stack = leading_blanks / 4;
        let line_crates: Vec<String> = line.split_whitespace().map(|c| c.to_string()).collect();

        let needed_stacks = starting_stack + line_crates.len();

        //check if the number of stacks is enough
        while stacks.stack_vec.len() < needed_stacks {
            stacks.stack_vec.push(Vec::new());
        }

        let mut push_counter = 0;
        for crate_item in line_crates {
            stacks.stack_vec[starting_stack + push_counter].push(crate_item);
            push_counter += 1;
        }
    }

    // reverse the stacks items so top will be last
    for stack in &mut stacks.stack_vec {
        stack.reverse();
    }

    Ok(stacks)
}

// refactor getting both instrucions and stacks to same function? file is being read 2 times
pub fn get_instuctions() -> Result<Instructions, Box<dyn Error>> {
    let f = fs::read_to_string("input.txt").expect("input provided by problem");
    let mut instructions = Instructions {
        instructions_vec: Vec::new(),
    };

    for line in f.lines() {
        // check if this is a instruction line
        if line.chars().nth(0) == Some('m') {
            let line_instructions: Vec<u32> = line
                .split_whitespace()
                .filter_map(|i| i.parse::<u32>().ok())
                .collect();
            instructions.instructions_vec.push(line_instructions);
        }
    }

    Ok(instructions)
}

pub fn crane(stacks: &mut Stacks, instructions: Instructions) {
    for instruction in instructions.instructions_vec {
        let qty = instruction[0];
        let source = (instruction[1] - 1) as usize;
        let target = (instruction[2] - 1) as usize;

        for _ in 0..qty {
            let moved_crate_option = stacks.stack_vec[source].pop();

            if let Some(moved_crate) = moved_crate_option {
                stacks.stack_vec[target].push(moved_crate);
            }
        }
    }
}
