use std::io;
use std::io::prelude::*;

struct Config {
    a: u32,
    b: u32,
}

impl Config {
    fn new(input: &str) -> Config {
        let mut input = input
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("Not a number"));
        
        Config {
            a: input.next().unwrap(),
            b: input.next().unwrap(),
        }
    }
}

fn solve(cfg: Config) {
    let (a, mut b) = (cfg.a, cfg.b);

    let mut vec_b: Vec<u32> = Vec::new();

    while b != 0 {
        vec_b.push(b % 10);
        b /= 10;
    }
    
    let sum: u32 = (0..3)
        .zip(vec_b)
        .map(|(p, b)| {
            println!("{}", a * b);
            10u32.pow(p) * a * b
        })
        .sum();

    println!("{}", sum);
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let cfg = Config::new(&input);
    solve(cfg);
    Ok(())
}
