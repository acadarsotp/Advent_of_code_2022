/*
* struct ElfPair {
*   range1
*   range2
* }
*
* read the input into a vector of ElfPair
* algo to check if contained
* new field in struct or outside counter to check for result
*/

use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct ElfPair {
    range1: RangeInclusive<u32>,
    range2: RangeInclusive<u32>,
    pub overlap: bool,
}

impl ElfPair {
    fn new(r1: RangeInclusive<u32>, r2: RangeInclusive<u32>) -> ElfPair {
        ElfPair {
            range1: r1,
            range2: r2,
            overlap: false,
        }
    }

    // refactor this function
    fn check_overlap(&mut self) {
        /*
        if (self.range1.start() <= self.range2.start() || self.range1.end() >= self.range2.end())
            && (self.range2.start() <= self.range1.start()
                || self.range2.end() >= self.range1.end())
        */
        for item in self.range1.clone() {
            if self.range2.contains(&item) {
                self.overlap = true;
            }
        }
    }
}

pub fn read_input() -> Result<Vec<ElfPair>, Box<dyn Error>> {
    let f = fs::read_to_string("input.txt").expect("input file provided by problem");
    let mut pair_list = Vec::new();

    for pair in f.lines() {
        let mut pair_divider = pair.split(',');

        let r1 = pair_divider.next().expect("input file provided by problem");
        let mut r1_divider = r1.split('-');
        let rg1_start = r1_divider.next().unwrap().parse().unwrap();
        let rg1_end = r1_divider.next().unwrap().parse().unwrap();
        let r1_rng = rg1_start..=rg1_end;

        let r2 = pair_divider.next().expect("input file provided by problem");
        let mut r2_divider = r2.split('-');
        let rg2_start = r2_divider.next().unwrap().parse().unwrap();
        let rg2_end = r2_divider.next().unwrap().parse().unwrap();
        let r2_rng = rg2_start..=rg2_end;

        let mut new_pair = ElfPair::new(r1_rng, r2_rng);
        new_pair.check_overlap();

        pair_list.push(new_pair);
    }

    Ok(pair_list)
}
