use std::fs;
use std::env;

// This is gonna get awfully repetitious...
pub mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num = &args[1];
    let filename = format!("input/{}", day_num);
    let input = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("{filename} not found"));

    match day_num.as_str() {
        // So is this, but it seems harder to avoid.
        "1" => day1::day1(&input),
        _ => println!("no code for {day_num} yet")
    }
}
