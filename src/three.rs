
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp::min;


fn unique_houses(moves: &str) -> usize {
    use std::collections::BTreeSet;

    let mut houses: BTreeSet<(i32, i32)> = BTreeSet::new();
    houses.insert((0, 0));

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    for c in moves.chars() {
        match c {
            '<' => i -= 1,
            '>' => i += 1,
            'v' => j -= 1,
            '^' => j += 1,
            _ => panic!("Invalid move: {}", c),
        }
        houses.insert((i, j));
    }

    return houses.len();
}

fn unique_houses_robo(moves: &str) -> usize {
    use std::collections::BTreeSet;

    let mut houses: BTreeSet<(i32, i32)> = BTreeSet::new();
    houses.insert((0, 0));

    let mut positions = [(0, 0), (0, 0)];

    let mut current_index = 0;
    for c in moves.chars() {
        let (ref mut i, ref mut j) = positions[current_index];
        match c {
            '<' => *i -= 1,
            '>' => *i += 1,
            'v' => *j -= 1,
            '^' => *j += 1,
            _ => panic!("Invalid move: {}", c),
        }
        houses.insert((*i, *j));
        current_index = (current_index + 1) % 2;
    }

    return houses.len();
}

pub fn solve() {
    let path = Path::new("input-3.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open file because: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("could not read file because: {}", Error::description(&why)),
        Ok(_) => {
            println!("Total houses: {}", unique_houses(&s));
            println!("Total houses with robosanta: {}", unique_houses_robo(&s));
        }
    }

}

#[test]
fn test_three() {
    assert_eq!(unique_houses("^>v<"), 4);
    assert_eq!(unique_houses_robo("^>v<"), 3);

    assert_eq!(unique_houses("^v^v^v^v^v"), 2);
    assert_eq!(unique_houses_robo("^v^v^v^v^v"), 11);
}
