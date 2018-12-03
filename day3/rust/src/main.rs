#[macro_use]
extern crate nom;

mod parser;

use self::parser::parse;
// use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Rect {
  id: u16,
  top: u16,
  left: u16,
  width: u8,
  height: u8
}

fn load_input() -> Result<Vec<Rect>, std::io::Error> {
    let mut file = File::open("./input")?;
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();
    let rects: Vec<Rect> = file_string.split("\n").map(|s| parse(s).unwrap()).collect();
    Result::Ok(rects)
}

fn main() {
    let a = load_input();
}
