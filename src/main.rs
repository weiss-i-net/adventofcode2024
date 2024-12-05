use chrono::TimeZone;
use clap::Parser;
use seq_macro::seq;
use std::fs::File;
use std::io::Read;
use std::io::Write;

mod days;
mod util;

static AOC_YEAR: i32 = 2024;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long, default_value = get_latest_day().to_string(), group = "'day", num_args = 1..)]
    day: Vec<i32>,
    #[arg(short, long, group = "'day")]
    all: bool,
    #[arg(short, long, group = "input")]
    input_string: Option<String>,
    #[arg(short, long, group = "input")]
    from_stdin: bool,
}

fn get_latest_day() -> i32 {
    let now = chrono::Utc::now();
    let start = chrono::Utc
        .with_ymd_and_hms(AOC_YEAR, 12, 1, 5, 0, 0)
        .unwrap();
    let end = chrono::Utc
        .with_ymd_and_hms(AOC_YEAR, 12, 25, 5, 0, 0)
        .unwrap();

    if now < start {
        0
    } else if now > end {
        25
    } else {
        ((now - start).num_days() + 1) as i32
    }
}

pub fn download_input(day: i32) {
    let url = format!("https://adventofcode.com/{}/day/{}/input", AOC_YEAR, day);
    let client = reqwest::blocking::Client::new();
    let session = dotenv::var("AOC_SESSION").unwrap();
    let response = client
        .get(&url)
        .header("cookie", format!("session={}", session))
        .send()
        .expect("Failed to send request");

    let input = response.text().expect("Failed to get response body");
    let path = format!("input/day_{:02}.txt", day);
    let mut file = File::create_new(path).expect("Failed to create file");
    file.write_all(input.as_bytes())
        .expect("Failed to write to file");
}

fn get_solver(day: i32, part: i32) -> fn(&str) -> String {
    seq!(N in 01..=25 {
        match day {
            #(
                N => match part {
                    1 => days::day_~N::part_1,
                    2 => days::day_~N::part_2,
                    _ => panic!("Invalid part"),
                }
            )*
            _ => panic!("Invalid day"),
        }
    })
}

fn main() {
    let args = Arguments::parse();

    // Download input for all days that have not been downloaded yet.
    for day in 1..=get_latest_day() {
        if std::fs::exists(format!("input/day_{:02}.txt", day)).unwrap() {
            continue;
        }
        println!("downloading input for day {}...", day);
        download_input(day);
    }
    let mut days = if !args.all {
        args.day
    } else {
        (1..=get_latest_day()).collect()
    };
    days.sort();

    let has_special_input = args.input_string.is_some() || args.from_stdin;
    let inputs = if days.len() == 1 && has_special_input {
        if args.from_stdin {
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            vec![input]
        } else {
            vec![args.input_string.unwrap()]
        }
    } else {
        days.iter()
            .map(|day| {
                let mut input = String::new();
                File::open(format!("input/day_{:02}.txt", day))
                    .unwrap()
                    .read_to_string(&mut input)
                    .unwrap();
                input
            })
            .collect()
    };

    for (day, input) in days.into_iter().zip(inputs.into_iter()) {
        // Run the solver for the given day and part.
        println!("running day {}...", day);
        for part in [1, 2] {
            let answer = get_solver(day, part)(&input);
            println!("  {} (part {})", answer, part);
        }
    }
}
