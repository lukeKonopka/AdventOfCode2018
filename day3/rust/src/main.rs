#[macro_use]
extern crate nom;

mod parser;
mod rect;
mod canvas;

use self::{parser::parse, rect::Rect, canvas::Canvas};
use std::fs::File;
use std::io::prelude::*;

fn load_input(path: &str) -> Result<Vec<Rect>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();
    let rects: Vec<Rect> = file_string.split("\n").map(|s| parse(s).unwrap()).collect();
    Result::Ok(rects)
}

fn main() {
    let rects = load_input("./input").unwrap();
    let mut canvas = Canvas::create(1000);
    for rect in rects.iter() {
        canvas.paint(rect);
    }
    
    let no_overlap_rect = rects.iter().find(|rect| !canvas.is_overlap(rect));

    println!("Area: {}", canvas.get_overlap_area());
    match no_overlap_rect {
        Some(rect) => println!("No overlap rect id: {}", rect.id),
        None => println!("Every rect overlaps")
    }
}
