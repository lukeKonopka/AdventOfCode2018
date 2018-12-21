use itertools::Itertools;

use std::fmt::Debug;
use std::ops::Not;

#[derive(PartialEq, Clone, Copy)]
enum Polarity {
    Upper,
    Lower
}

impl From<char> for Polarity {
  fn from(c: char) -> Polarity {
    if c >= 'a' && c <= 'z' { Polarity::Lower } else { Polarity::Upper }
  }
}

impl Not for Polarity {
    type Output = Polarity;
    fn not(self) -> Self {
        match self {
            Polarity::Upper => Polarity::Lower,
            Polarity::Lower => Polarity::Upper,
        }
    }
}

#[derive(Clone, Copy)]
pub struct PolymerUnit {
    letter: char,
    polarity: Polarity
}

impl Debug for PolymerUnit {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    match &self.polarity {
      Polarity::Upper => write!(f, "{}", self.letter.to_uppercase()),
      Polarity::Lower => write!(f, "{}", self.letter.to_lowercase()),
    }
  }
}

impl PolymerUnit {
    fn will_react(&self, other_unit: &PolymerUnit) -> bool {
        self.letter == other_unit.letter && self.polarity == !other_unit.polarity.clone()
    }

    fn create(letter: char) -> Self {
      PolymerUnit {
        letter: letter.to_lowercase().next().unwrap(),
        polarity: letter.into(),
      }
    }
}

pub struct Polymer {
  list: Vec<PolymerUnit>
}

impl Debug for Polymer {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    let str_vec: Vec<String> = self.list.iter().map(|u| format!("{:?}", u)).collect();
    write!(f, "/");
    write!(f, "{:?}", str_vec.join(""));
    write!(f, "/")
  }
}

impl From<String> for Polymer {
  fn from(s: String) -> Self {
    let mut polymer = Polymer::create();
    for c in s.chars().into_iter() {
        polymer.push(c);
    }
    polymer
  }
}

impl Polymer {
  pub fn create() -> Self {
    Polymer { list: vec![] }
  }

  pub fn push(&mut self, letter: char) {
    self.list.push(PolymerUnit::create(letter))
  }

  pub fn len(&self) -> usize {
    self.list.len()
  }

  pub fn react(&self) -> Self {
    let new_vec: Vec<PolymerUnit> = self.list.iter().peekable()
      .batching(|it| {
        let first_unit = it.next();
        let second_unit = it.peek();
        match (first_unit, second_unit) {
          (Some(a), Some(b)) => {
            if a.will_react(b) {
              it.next();
              Some(vec![])
            } else {
              Some(vec![a])
            }
          },
          (Some(a), None) => Some(vec![a]),
          _ => {
            it.next();
            None
          }
        }
      })
      .flatten()
      .map(|u| u.clone())
      .collect();
    Polymer { list: new_vec }
  }

  pub fn filter(&self, letter: &char) -> Self {
    let new_list: Vec<PolymerUnit> = self.list.iter()
      .filter(|u| u.letter != *letter)
      .map(|u| u.clone())
      .collect();
    Polymer { list: new_list }
  }

  pub fn will_react(&self) -> bool {
    self.list.iter().tuple_windows().any(|(a, b)| a.will_react(b))
  }
}
