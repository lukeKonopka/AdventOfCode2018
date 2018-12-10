#[macro_use]
extern crate nom;

mod parser;
mod world;

use chrono::Timelike;
use slurp::read_all_lines;
use std::io::Error;
use crate::parser::{Log, LogPayload, parse_log};
use crate::world::{WorldState, SleepStatus, SleepTable};


fn load_input(path: &str) -> Result<Vec<Log>, Error> {
    let lines = read_all_lines(path)?;
    Result::Ok(lines.iter().map(|line| parse_log(line).unwrap()).collect())
}

fn get_world_state(init_world: WorldState, logs: Vec<Log>) -> WorldState {
    logs.iter().fold::<WorldState, _>(init_world, |mut world, log| {
        let current_time = if log.timestamp.hour() == 0 {
            log.timestamp.minute() as u16
        } else {
            0
        };

        match log.payload {
            LogPayload::BeginsShift(id) => {
                world.current_guard_id = id as u16;
                world.status = SleepStatus::Awake;
            },
            LogPayload::FallsAsleep => {
                world.status = SleepStatus::Asleep;
            }
            LogPayload::WakesUp => {
                for minute in world.last_time..current_time {
                    world.sleep_table.count_sleep(world.current_guard_id as u32, minute as usize);
                    world.status = SleepStatus::Awake;
                }
            }
        }
        world.last_time = current_time;
        world
    })
}

fn strategy_one(final_world: &WorldState) -> u32 {
    let (final_guard_id, _) = final_world.sleep_table.get_all_guards()
        .map(|guard_id| (guard_id, final_world.sleep_table.get_guard_row(guard_id)
            .fold(0, |slept, minute| slept+minute)))
        .max_by(|(_, time_a), (_, time_b)| time_a.cmp(time_b)).unwrap();
    
    let (most_slept_min, _) = final_world.sleep_table.get_guard_row(final_guard_id).enumerate().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    *final_guard_id * most_slept_min as u32
}

fn strategy_two(final_world: &WorldState) -> u32 {
    let (most_slept_minute_idx, _) = final_world.sleep_table.table.iter().enumerate().max_by(|v1, v2| v1.1.cmp(v2.1)).unwrap();
    let most_slept_guard_idx = most_slept_minute_idx / 60;
    let most_slept_minute = most_slept_minute_idx % 60;
    let (guard_id, _) = final_world.sleep_table.guard_map.iter().find(|(_, &idx)| idx == most_slept_guard_idx).unwrap();
    most_slept_minute as u32 * guard_id
}

fn main() {
    let mut logs = load_input("./input").unwrap();
    logs.sort_by(|log_a, log_b| log_a.timestamp.partial_cmp(&log_b.timestamp).unwrap_or(std::cmp::Ordering::Equal));

    let init_world = WorldState {
        sleep_table: SleepTable::default(),
        current_guard_id: 0,
        status: SleepStatus::Awake,
        last_time: 0,
    };

    let final_world = get_world_state(init_world, logs);

    println!("{:?}", final_world.sleep_table);

    println!("Strategy one: {}", strategy_one(&final_world));
    println!("Strategy two: {}", strategy_two(&final_world));
}
