use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn floor(directions: &str) -> i32 {

    let mut open = 0;
    let mut close = 0;

    for c in directions.chars() {
        match c {
            '(' => open += 1,
            ')' => close += 1,
            _ => {}
        }
    }

    return open - close;
}

fn first_basement(directions: &str) -> usize {

    let mut open = 0;
    let mut close = 0;
    let mut first = 0;

    for (i, c) in directions.chars().enumerate() {
        match c {
            '(' => open += 1,
            ')' => close += 1,
            _ => {}
        }
        if open - close == -1 {
            first = i;
            return i + 1;
        }
    }

    return first;
}


fn f1() {
    println!("f1!");

    let path = Path::new("input-1.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open file because: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("could not read file because: {}", Error::description(&why)),
        Ok(_) => {
            println!("Floor: {}; First basement: {}",
                     floor(&s),
                     first_basement(&s))
        }
    }

}


fn main() {
    f1();
}
