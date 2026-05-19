use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

fn notify(msg: &str) {
    std::process::Command::new("notify-send")
        .args(["pomo", msg])
        .spawn()
        .ok();
}

fn print_time(d: u64, working: bool, round: u32) {
    let seconds = d % 60;
    let minutes = (d / 60) % 60;
    let hours = d / 3600;

    let label = if working { "STUDY" } else { "BREAK" };

    print!(
        "\r[Round {round}] {label} {:02}:{:02}:{:02}",
        hours, minutes, seconds
    );
    std::io::stdout().flush().unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let work_mins: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(60);
    let break_mins: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);

    let work_time = Duration::from_secs(work_mins * 60);
    let break_time = Duration::from_secs(break_mins * 60);

    let mut round = 1;

    loop {
        println!("Starting round {round} — work ({work_mins}/{break_mins}min)");

        let start = Instant::now();
        while start.elapsed() < work_time {
            print_time(start.elapsed().as_secs(), true, round);
            thread::sleep(Duration::from_millis(100));
        }

        notify("Work session complete — take a break!");
        println!("\nBreak time ({break_mins}min)");

        let start = Instant::now();
        while start.elapsed() < break_time {
            print_time(start.elapsed().as_secs(), false, round);
            thread::sleep(Duration::from_millis(100));
        }

        notify("Break over — back to work!");
        println!();

        round += 1;
    }
}
