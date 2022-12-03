use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn rec_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel + rec_fuel(fuel)
    } else {
        0
    }
}

fn run(input: &str) -> i32 {
    input
        .split_whitespace()
        .map(|w| {
            let mass: i32 = w.parse().unwrap();
            rec_fuel(mass)
        })
        .sum()
}
