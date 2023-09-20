use day3_1::read_input;

fn main() {
    if let Ok(duplicates) = read_input() {
        let result: u32 = duplicates.iter().sum();
        println!("{result}");
    }
}
