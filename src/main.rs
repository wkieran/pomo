use std::io::{self, Write};
use std::time::{Duration, Instant};

// TODO:
// - Add start/stop arguments from user
// - overwrite current line rather than adding a new line each second

fn print_time(d: u64) {
    let seconds = d % 60;

    // 4 hours 36 min 2 seconds = (4 * 60 * 60) + (36 * 60) + 2 seconds
    let minutes = d / 60;
    let hours = d / 60 / 60;

    let mut sec_str = seconds.to_string();
    if seconds < 10 {
        sec_str = "0".to_string() + &sec_str;
    }

    let mut min_str = minutes.to_string();
    if minutes < 10 {
        min_str = "0".to_string() + &min_str;
    }

    let mut hrs_str = hours.to_string();
    if hours < 10 {
        hrs_str = "0".to_string() + &hrs_str;
    }

    io::stdout().flush().unwrap();
    println!("{}:{}:{}", hrs_str, min_str, sec_str);
}

fn main() {
    const WORK_TIME: Duration = Duration::from_mins(60);
    const BREAK_TIME: Duration = Duration::from_mins(10);

    let mut t = Instant::now();
    let mut work_time: bool = true;

    loop {
        println!("Starting work time");

        let start_work_time = Instant::now();
        let mut last = t.duration_since(start_work_time).as_secs();
        while work_time && t.duration_since(start_work_time) < WORK_TIME {
            t = Instant::now();
            let d = t.duration_since(start_work_time).as_secs();
            if last != d {
                print_time(d);
                last = d;
            }
        }
        work_time = false;

        println!("Starting break time");

        let start_break_time = Instant::now();
        last = t.duration_since(start_break_time).as_secs();
        while !work_time && t.duration_since(start_break_time) < BREAK_TIME {
            t = Instant::now();
            let d = t.duration_since(start_break_time).as_secs();
            if last != d {
                print_time(d);
                last = d;
            }
        }
        work_time = true;
    }
}
