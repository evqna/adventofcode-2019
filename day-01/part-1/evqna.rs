use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> i32 {
    input
        .split_whitespace()
        .map(|w| {
            let mass: i32 = w.parse().unwrap();
            mass / 3 - 2
        })
        .sum()
}
