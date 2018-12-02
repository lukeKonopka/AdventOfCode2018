use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn load_input() -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open("./src/input")?;
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();
    let ids: Vec<String> = file_string.split("\n").map(|s| String::from(s)).collect();
    Result::Ok(ids)
}

fn count_repeats(id: String) -> impl Iterator<Item = (char, usize)> {
    id.chars()
        .map(|c| (c, c))
        .into_group_map()
        .into_iter()
        .map(|(k, g)| (k, g.into_iter().count()))
}

fn parse_id(id: String) -> (bool, bool) {
    count_repeats(id).fold((false, false), |(has2, has3), (_, count)| {
        (has2 || count == 2, has3 || count == 3)
    })
}


fn diff(a: String, b: String) -> String {
    a.chars()
        .zip(b.chars())
        .fold(vec![], |difference, (char_a, char_b)| {
            if char_a == char_b {
                difference
            } else {
                [&difference[..], &[char_a]].concat()
            }
        }).iter()
        .collect()
}


fn part_1(input: Vec<String>) -> u32 {
    let (twos_count, threes_count) =
        input
            .into_iter()
            .map(|id| parse_id(id))
            .fold((0, 0), |(twos, threes), (has2, has3)| {
                (
                    if has2 { twos + 1 } else { twos },
                    if has3 { threes + 1 } else { threes },
                )
            });
    twos_count * threes_count
}

fn part_2(input: Vec<String>) -> Option<String> {
    let iter_clone = input.clone().into_iter();
    input
        .into_iter()
        .cartesian_product(iter_clone)
        .map(|(id_a, id_b)| (id_a.clone(), diff(id_a, id_b)))
        .filter(|(_, diff)| diff.len() == 1)
        .map(|(id, diff)| {
            id.chars()
                .filter(|c| c != &(diff.as_bytes()[0] as char))
                .collect()
        }).next()
}

fn main() {
    let ids = load_input().unwrap();
    println!("First part:\n\tChecksum: {}", part_1(ids.clone()));
    println!("Second part:\n\tDiff: {}", part_2(ids.clone()).unwrap());
}

#[test]
fn part_1_test() {
    let input: Vec<String> = vec![
        "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    ].into_iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(12, part_1(input));
}

#[test]
fn part_2_test() {
    let input: Vec<String> = vec![
        "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ].into_iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(Option::Some(String::from("fgij")), part_2(input));
}
