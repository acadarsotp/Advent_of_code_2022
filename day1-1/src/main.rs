use day1_1::read_input;

fn main() {
    if let Ok(elf_list) = read_input() {
        let max = elf_list
            .iter()
            .map(|elf| elf.food_list.iter().sum::<i32>())
            .max()
            .unwrap();

        println!("{max}");
    }
}
