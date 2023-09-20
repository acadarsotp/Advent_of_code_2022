use day1_2::read_input;

fn main() {
    if let Ok(elf_list) = read_input() {
        let mut elf_cals_list = elf_list
            .iter()
            .map(|elf| elf.food_list.iter().sum::<i32>())
            .collect::<Vec<_>>();
        // cannot sort an iterator, get the vector and sort it
        elf_cals_list.sort();

        // now that its sorted, take the top 3 and sum them
        let max = elf_cals_list.iter().rev().take(3).sum::<i32>();

        println!("{max}");
    }
}
