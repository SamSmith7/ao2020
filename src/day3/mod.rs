use std::fs::File;
use std::io::prelude::*;


fn parse_input() -> Vec<Vec<char>> {

    let mut f = File::open("./src/day3/input.txt").expect("File Not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error Reading File");

    contents.split("\n")
        .filter(|value| value != &"")
        .map(|value| {
            value.chars().collect()
        })
        .collect()
}

fn calculate_trees_hit(x_diff: usize, y_diff: usize, input: &Vec<Vec<char>>) -> usize {
    let width = input[0].len();
    let mut x = 0;
    let mut tree_count = 0;

    for y in 1..(input.len() / y_diff) {
        x = (x + x_diff) % width;
        let val = input[y * y_diff][x];
        if val == '#' {
            tree_count += 1;
        }
    }

    tree_count
}


pub fn part1() -> String<> {
    let input = parse_input();

    calculate_trees_hit(3, 1, &input).to_string()
}


pub fn part2() -> String<> {

    let input = parse_input();

    let trials: [[usize; 2]; 5] = [
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2]
    ];

    let mut result: Vec<usize> = vec![];

    for [x_diff, y_diff] in trials.iter() {
        result.push(calculate_trees_hit(*x_diff, *y_diff, &input));
    }

    let mut res = result[0];
    for val in 1..result.len() {
        res *= result[val];
    }

    res.to_string()
}
