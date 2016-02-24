use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp::max;
use regex::Regex;


fn extract_indices(line: &str) -> (usize, usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }
    let mut vec: Vec<usize> = Vec::new();

    for caps in RE.captures_iter(line) {
        let x = caps.at(1).unwrap();
        vec.push(x.parse::<usize>().unwrap());
    }

    assert_eq!(vec.len(), 4);

    return (vec[0], vec[1], vec[2], vec[3]);
}


fn matrix_to_index(x: usize, y: usize) -> usize {
    return x * 1000 + y;
}

fn operate_on_range<T: Clone, Op>(grid: &mut Vec<T>,
                                  x1: usize,
                                  y1: usize,
                                  x2: usize,
                                  y2: usize,
                                  op: Op)
    where Op: Fn(T) -> T
{
    for column in x1..x2 + 1 {
        for row in y1..y2 + 1 {
            let index = matrix_to_index(column, row);
            grid[index] = op(grid[index].clone());
        }
    }
}


fn process_line_1(line: &str, grid: &mut Vec<bool>) {
    let (x1, y1, x2, y2) = extract_indices(line);
    if line.contains("toggle") {
        operate_on_range(grid, x1, y1, x2, y2, |value: bool| !value);
    } else if line.contains("turn on") {
        operate_on_range(grid, x1, y1, x2, y2, |_| true);
    } else if line.contains("turn off") {
        operate_on_range(grid, x1, y1, x2, y2, |_| false);
    }
}


fn process_line_2(line: &str, grid: &mut Vec<i32>) {
    let (x1, y1, x2, y2) = extract_indices(line);
    if line.contains("toggle") {
        operate_on_range(grid, x1, y1, x2, y2, |value: i32| value + 2);
    } else if line.contains("turn on") {
        operate_on_range(grid, x1, y1, x2, y2, |value: i32| value + 1);
    } else if line.contains("turn off") {
        operate_on_range(grid, x1, y1, x2, y2, |value: i32| max(value - 1, 0));
    }
}

pub fn solve() {
    let path = Path::new("input-6.txt");

    let file = match File::open(&path) {
        Err(why) => panic!("could not open file because: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let file = BufReader::new(file);

    let mut grid_1: Vec<bool> = vec![false; 1000*1000];
    let mut grid_2: Vec<i32> = vec![0 as i32; 1000*1000];

    for line in file.lines() {
        match line {
            Err(why) => panic!("could not read line because: {}", Error::description(&why)),
            Ok(line) => {
                process_line_1(&line, &mut grid_1);
                process_line_2(&line, &mut grid_2);
            }
        }
    }

    let on = grid_1.iter().filter(|&x| *x == true).count();
    println!("Total number of on lights: {}", on);

    let mut brightness: i32 = 0;
    for value in grid_2 {
        brightness += value;
    }
    println!("Total brightness: {}", brightness);
}


#[test]
fn test_six() {}
