use day4_1::read_input;

fn main() {
    if let Ok(pair_vec) = read_input() {
        let result: u32 = pair_vec.iter().map(|p| p.fully_contains as u32).sum();
        println!("{result}");
    }
}
