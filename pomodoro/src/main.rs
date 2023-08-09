use std::thread::sleep;
use std::time::Duration;
use awesomewm_naughty::{Notification, send};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Pomodoro"), help = "Task name")]
    name: String,

    #[arg(short, long, default_value_t = 25, help = "Work time(in minutes)")]
    work_time: u64,

    #[arg(short, long, default_value_t = 5, help = "Short break time(in minutes)")]
    short_break_time: u64,

    #[arg(short, long, default_value_t = 30, help = "Long break time(in minutes)")]
    long_break_time: u64,

    #[arg(short, long, default_value_t = 4, help = "Short breaks until long break")]
    cycles: u64,

    #[arg(short, long, default_value_t = 1, help = "How many times the cycle will be run")]
    repeat: u64
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.repeat {
        Notifications::starting_cycle(&args);

        for _ in 0..args.cycles {
            sleep(Duration::from_secs(args.work_time * 60));

            Notifications::starting_short_break(&args);

            sleep(Duration::from_secs(args.short_break_time * 60));

           Notifications::starting_cycle(&args);
        }

        sleep(Duration::from_secs(args.work_time * 60));

        Notifications::starting_long_break(&args);

        sleep(Duration::from_secs(args.long_break_time * 60));
    }

    Notifications::ending_work(&args);
}

struct Notifications{}

impl Notifications {
    pub fn starting_cycle(args: &Args) {
        let notification = Notification::new()
            .title(args.name.clone())
            .text("Starting cycle - back to work".to_string())
            .border_color("#ff0000".to_string())
            .border_width(3)
            .position("top_middle".to_string())
            .build();

        let _ = send(notification);
    }

    pub fn starting_short_break(args: &Args) {
        let notification = Notification::new()
            .title(args.name.clone())
            .text("Take a short break".to_string())
            .border_color("#0000ff".to_string())
            .border_width(3)
            .position("top_middle".to_string())
            .build();

        let _ = send(notification);
    }

    pub fn starting_long_break(args: &Args) {
        let notification = Notification::new()
            .title(args.name.clone())
            .text("Take a long break".to_string())
            .border_color("#0000ff".to_string())
            .border_width(3)
            .position("top_middle".to_string())
            .build();

        let _ = send(notification);
    }

    pub fn ending_work(args: &Args) {
        let notification = Notification::new()
            .title(args.name.clone())
            .text("Work ended".to_string())
            .border_color("#00ff00".to_string())
            .border_width(3)
            .position("top_middle".to_string())
            .build();

        let _ = send(notification);
    }
}