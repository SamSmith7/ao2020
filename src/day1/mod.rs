use std::fs::File;
use std::collections::HashSet;
use std::io::prelude::*;


fn parse_input() -> Vec<i32> {

    let mut f = File::open("./src/day1/input.txt").expect("File Not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error Reading File");

    contents.split("\n")
        .filter(|value| value != &"")
        .map(|value| {
            let num = value.parse::<i32>();
            match num {
                Ok(v) => v,
                Err(_e) => 0
            }
        })
        .collect::<Vec<i32>>()
}


pub fn part1() -> String<> {

    let input = parse_input();
    let mut set = HashSet::new();

    for val in input.iter() {
        set.insert(val);
    }

    let target = 2020;

    for idx in 0..input.len() {
        let val = input[idx];
        if set.contains(&(target - val)) {
            return (val * (target - val)).to_string()
        }
    }
    panic!("Matching value not found");
}


pub fn part2() -> String<> {

    let input = parse_input();
    let mut set = HashSet::new();

    for val in input.iter() {
        set.insert(val);
    }

    let target = 2020;

    for i in 0..input.len() {
        let val1 = input[i];
        for j in 1..input.len() {
            let val2 = input[j];
            let remainder = target - (val1 + val2);
            if set.contains(&remainder) {
                return (val1 * val2 * remainder).to_string()
            }
        }
    }
    panic!("Matching value not found");
}
