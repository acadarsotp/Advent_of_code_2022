use day2_1::read_input;

fn main() {
    if let Ok(round_list) = read_input() {
        let total: i32 = round_list.iter().map(|round| round.score).sum();
        println!("{total}");
    }
}
