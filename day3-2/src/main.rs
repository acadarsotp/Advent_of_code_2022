use day3_2::{extract_badges, read_input};

fn main() {
    if let Ok(elf_groups) = read_input() {
        let badges = extract_badges(elf_groups);

        let result: u32 = badges.iter().sum();
        println!("{result}");
    }
}
