use day5_1::{crane, get_instuctions, get_stacks};

fn main() {
    let mut stacks = get_stacks().unwrap();
    let instructions = get_instuctions().unwrap();

    crane(&mut stacks, instructions);

    for stack in stacks.stack_vec {
        println!("{}", stack.last().unwrap());
    }
}
