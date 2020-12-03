use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Record {
    letter: char,
    min: usize,
    max: usize,
    password: String
}

impl Record {
    fn new(letter: char, min: usize, max: usize, password: &str) -> Record {
        Record {
            letter: letter,
            min: min,
            max: max,
            password: password.to_string()
        }
    }
    fn is_valid(&self) -> bool {
        let count = self.password.chars().fold(0, |acc, chr| {
            if chr == self.letter {
                return acc + 1;
            }
            return acc;
         });
         count >= self.min && count <= self.max
    }
    fn is_valid_v2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();

        let pos1 = chars.len() >= self.min - 1 && chars[self.min - 1] == self.letter;
        let pos2 = chars.len() >= self.max - 1 && chars[self.max - 1] == self.letter;

        return pos1 != pos2
    }
}

fn parse_input() -> Vec<Record> {

    let mut f = File::open("./src/day2/input.txt").expect("File Not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error Reading File");

    contents.split("\n")
        .filter(|value| value != &"")
        .map(|value| {
            let split: Vec<&str> = value.split(":").collect();
            let password = split[1].trim();
            let split2: Vec<&str> = split[0].split(" ").collect();
            let letter = split2[1].trim().chars().collect::<Vec<char>>()[0];
            let range: Vec<usize> = split2[0].split("-").map(|val| {
                let num = val.parse::<usize>();
                return match num {
                    Ok(v) => v,
                    Err(_e) => 0
                }
            }).collect();

            Record::new(letter, range[0], range[1], password)
        })
        .collect()
}


pub fn part1() -> String<> {

    let input = parse_input();

    input.iter().fold(0, |acc, record| {
        if record.is_valid() {
            return acc + 1;
        }
        return acc
    }).to_string()
}


pub fn part2() -> String<> {

    let input = parse_input();

    input.iter().fold(0, |acc, record| {
        if record.is_valid_v2() {
            return acc + 1;
        }
        return acc
    }).to_string()
}
