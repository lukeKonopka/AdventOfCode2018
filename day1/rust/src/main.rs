use std::fs::File;
use std::collections::HashSet;
use std::io::prelude::*;

fn load_input() -> Result<Vec<i32>, std::io::Error> {
    let mut input_file = File::open("./src/input")?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();
    let freq_changes: Vec<i32> = input_string.split("\n").map(
        |freq_change| freq_change.parse::<i32>().unwrap()
    ).collect();
    Result::Ok(freq_changes)
}

fn part_1() -> i32 {
    let freq_changes = load_input().unwrap();
    let final_frequency = freq_changes.into_iter().fold(0, |acc, freq| acc + freq);
    final_frequency
}

fn part_2() -> i32 {
    let freq_changes = load_input().unwrap();
    let mut freq_reached = HashSet::new();
    let mut freq_loop = freq_changes.iter().cycle();
    let mut current_freq = 0;
    loop {
        let freq_change = freq_loop.next().unwrap();
        current_freq = current_freq + freq_change;
        if freq_reached.contains(&current_freq) {
            break
        } else {
            freq_reached.insert(current_freq);
        }
    }
    current_freq
}

fn main() {
    println!("Part 1:\n\tFinal frequency: {}", part_1());
    println!("Part 2:\n\tFirst repeat: {}", part_2());
}
