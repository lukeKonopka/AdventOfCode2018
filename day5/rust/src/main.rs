extern crate itertools;
extern crate rayon;

mod polymer;
use rayon::prelude::*;

use std::io::Read;
use std::fs::File;
use crate::polymer::Polymer;

fn load_polymer(path: &str) -> Result<Polymer, std::io::Error> {
    let mut file = File::open(path)?;
    let mut file_str = String::new();
    file.read_to_string(&mut file_str)?;
    Result::Ok(file_str.into())
}

fn get_reacted_len(polymer_to_react: Polymer) -> usize {
    let mut polymer: Polymer = polymer_to_react;
    while polymer.will_react() {
        polymer = polymer.react();
    }
    polymer.len()
}

fn part_1() -> usize {
    let loaded_polymer = load_polymer("./input").unwrap();
    get_reacted_len(loaded_polymer)
}

fn part_2() -> usize {
    let loaded_polymer = load_polymer("./input").unwrap();
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let longest = letters.par_iter().map(|letter| {
        let filtered_polymer = loaded_polymer.filter(letter);
        let reacted_len = get_reacted_len(filtered_polymer);
        reacted_len
    }).min();
    longest.unwrap_or(0)
}

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}
