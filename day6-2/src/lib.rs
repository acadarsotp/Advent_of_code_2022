/*
* start of packet -> 4 diferent chars
* report number of chars from buf start to end of start marker index of last + 1
*/

use std::error::Error;
use std::fs::read_to_string;

pub fn read_input() -> Result<usize, Box<dyn Error>> {
    let f = read_to_string("input.txt").expect("input provided by problem");

    let char_vec: Vec<char> = f.chars().collect();
    let mut buf = Vec::new();

    for (i, char) in char_vec.iter().enumerate() {
        if buf.len() < 14 {
            buf.push(char);
            continue;
        }

        if check_buf(&buf) {
            return Ok(i);
        } else {
            buf.rotate_left(1);
            buf[13] = char;
        }
    }

    //custom Err
    Err(todo!())
}

// usar un vector buffer, si no está lleno push hasta buf.len = 4, a partir de ahí
// comprobar si todos los componentes son distintos, si no rotate_left y cambiar

fn check_buf(v: &Vec<&char>) -> bool {
    let mut unique = Vec::new();
    for item in v {
        if !unique.contains(item) {
            unique.push(*item);
        }
    }

    v.len() == unique.len()
}
