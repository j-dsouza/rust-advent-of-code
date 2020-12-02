use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

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

    let result = input[..]
        .iter()
        .cartesian_product(input[..].iter())
        .filter(|(l, r)| **l + **r == 2020)
        .map(|(l, r)| l * r)
        .next()
        .unwrap();

    println!("{}", result);
}
