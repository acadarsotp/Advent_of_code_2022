/*
* A-X paper
* B-Y rock
* C-Z scissors
* score-> shape selected {
*   X = 1,
*   y = 2,
*   z = 2,
* } + outcome {
*   win = 6,
*   lose = 0,
*   draw = 3
* }
* struct Round {
*   rival,
*   me,
*   score,
* }
*/

use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Round {
    rival: char,
    own: char,
    pub score: i32,
}

impl Round {
    pub fn calc_score(&mut self) {
        let shape_score = match self.own {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("invalid input"),
        };
        let own_to_int = match self.own {
            'X' => 0,
            'Y' => 1,
            'Z' => 2,
            _ => panic!("invalid input"),
        };
        let rival_to_int = match self.rival {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!("invalid input"),
        };

        self.score = shape_score
            + match (own_to_int - rival_to_int + 3) % 3 {
                2 => 0,
                1 => 6,
                0 => 3,
                _ => panic!("invalid input"),
            };
    }
}

pub fn read_input() -> Result<Vec<Round>, Box<dyn Error>> {
    let f = fs::read_to_string("input.txt")?;
    let mut rounds = Vec::new();

    for line in f.lines() {
        let get_round: Vec<char> = line.chars().collect();
        let mut new_round = Round {
            rival: get_round[0],
            own: get_round[2],
            score: 0,
        };
        new_round.calc_score();
        rounds.push(new_round);
    }
    Ok(rounds)
}
