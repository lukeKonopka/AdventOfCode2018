use std::collections::HashMap;
use core::fmt::{Result as FormatResult, Formatter, Debug};

pub struct SleepTable {
    pub table: [u16; 23*60],
    pub guard_map: HashMap<u32, usize>,
}

impl Debug for SleepTable {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "        \t");
        for minute in 0..60 {
            write!(f, "{:<2} ", minute);
        }
        writeln!(f, "\n");
        for guard_idx in 0..23 {
            let formatted_guard_idx = format!("[{:<2}] -> \t", guard_idx);
            write!(f, "{}", formatted_guard_idx);
            for minute in 0..60 {
                let idx = guard_idx*60 + minute;
                let minutes_slept = self.table[idx];
                let formatted_minutes_slept = format!("{:<2} ", minutes_slept);
                write!(f, "{}", formatted_minutes_slept);
            }
            writeln!(f, "");
        }
        write!(f, "")
    }
}

impl Default for SleepTable {
    fn default() -> Self {
        SleepTable {
            table: [0; 23*60],
            guard_map: HashMap::new()
        }
    }
}

impl SleepTable {
    pub fn get_guard_row(&self, id: &u32) -> impl Iterator<Item=&u16> {
        let row_idx = self.guard_map.get(&id).unwrap();
        self.table[row_idx*60..(row_idx*60+60)].iter()
    }

    pub fn get_all_guards(&self) -> impl Iterator<Item=&u32> {
        self.guard_map.keys()
    }

    pub fn count_sleep(&mut self, id: u32, time: usize) {
        let guard_row = self.guard_map.get(&id);
        match guard_row {
            Some(row_idx) => {
                let idx = row_idx * 60 + time;
                self.table[idx] += 1;
            },
            None => {
                let new_idx = self.guard_map.iter().count();
                self.guard_map.insert(id, new_idx);
                self.count_sleep(id, time);
            }
        }
    }
}

#[derive(Debug)]
pub enum SleepStatus {
    Asleep,
    Awake
}

#[derive(Debug)]
pub struct WorldState {
    pub sleep_table: SleepTable,
    pub current_guard_id: u16,
    pub status: SleepStatus,
    pub last_time: u16
}