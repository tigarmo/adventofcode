
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp::min;

fn sqft(line: &str, accum: &mut i32, bow_accum: &mut i32) {
    let vec: Vec<&str> = line.split("x").collect();


    if vec.len() != 3 {
        panic!("Could not tokenize line properly: {}", line);
    }

    let w = vec[0].parse::<i32>().unwrap();
    let h = vec[1].parse::<i32>().unwrap();
    let d = vec[2].parse::<i32>().unwrap();

    let wh = w * h;
    let hd = h * d;
    let dw = d * w;

    // let min_area = *vec![wh, hd, dw].iter().min().unwrap();
    let min_area = min(min(wh, hd), dw);

    *accum += 2 * wh + 2 * hd + 2 * dw + min_area;

    let min_per = *vec![w + h, h + d, d + w].iter().min().unwrap() * 2;
    *bow_accum += min_per + w * h * d;
}


pub fn solve() {
    let path = Path::new("input-2.txt");

    let file = match File::open(&path) {
        Err(why) => panic!("could not open file because: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let file = BufReader::new(file);

    let mut accum: i32 = 0;
    let mut bow_accum: i32 = 0;

    for line in file.lines() {
        sqft(&line.unwrap(), &mut accum, &mut bow_accum);
    }

    println!("Total area needed: {}", accum);
    println!("Total bow length needed: {}", bow_accum);
}


#[test]
fn test_two() {

    let values = [("2x3x4", 58, 34), ("1x1x10", 43, 14)];

    for i in values.iter() {
        let (line, expected_accum, expected_bow_accum) = *i;

        let mut accum: i32 = 0;
        let mut bow_accum: i32 = 0;

        sqft(line, &mut accum, &mut bow_accum);

        assert_eq!(accum, expected_accum);
        assert_eq!(bow_accum, expected_bow_accum);
    }

}
