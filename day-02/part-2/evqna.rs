use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run_program(program: &[usize], noun: usize, verb: usize) -> usize {
    let mut mem = program.to_owned();   // Copy data into new Vec
    mem[1] = noun;
    mem[2] = verb;

    let mut ip: usize = 0;
    loop {
        let op = mem[ip];
        match op {
            1 => {
                let (a, b, c) = (mem[ip + 1], mem[ip + 2], mem[ip + 3]);
                mem[c] = mem[a] + mem[b];
                ip += 4;
            }
            2 => {
                let (a, b, c) = (mem[ip + 1], mem[ip + 2], mem[ip + 3]);
                mem[c] = mem[a] * mem[b];
                ip += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode {} at {}", op, ip),
        }
    }

    mem[0]
}

fn run(input: &str) -> usize {
    let mem: Vec<usize> = input
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    
    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(&mem, noun, verb) == 19690720 {
                return 100 * noun + verb
            }
        }
    }
    panic!("No match found");
}
