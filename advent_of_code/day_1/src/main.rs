use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn pt_1(input: Vec<i32>) -> i32 {
    input[..]
        .iter()
        .cartesian_product(input[..].iter())
        .filter(|(l, r)| **l + **r == 2020)
        .map(|(l, r)| l * r)
        .next()
        .unwrap()
}

fn pt_2(input: Vec<i32>) -> i32 {
    input[..]
        .iter()
        .cartesian_product(input[..].iter())
        .cartesian_product(input[..].iter())
        .filter(|((l, r), z)| **l + **r + **z == 2020)
        .map(|((l, r), z)| l * r * z)
        .next()
        .unwrap()
}

fn main() {
    let filename = "/home/jdsouza/github/rust-advent-of-code/advent_of_code/src/input/day_1.txt";
    // Open the file and read into vector of ints
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let num: i32 = line.unwrap().parse::<i32>().unwrap();
        input.push(num); // push acquired integer to the vector
    }

    let input_2 = input.clone();

    let result_pt_1 = pt_1(input);

    let result_pt_2 = pt_2(input_2);

    println!("Day 1 pt 1 result: {}", result_pt_1);

    println!("Day 1 pt 2 result: {}", result_pt_2);
}
