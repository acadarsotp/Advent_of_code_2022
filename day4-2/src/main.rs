use day4_2::read_input;

fn main() {
    if let Ok(pair_vec) = read_input() {
        let result: u32 = pair_vec.iter().map(|p| p.overlap as u32).sum();
        println!("{result}");
    }
}
