use std::io;
use std::io::prelude::*;

struct Config {
    a: i32,
    b: i32,
}

impl Config {
    fn new(input: &str) -> Vec<Config> {
        let mut input = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let mut cfgs = Vec::new();
        loop {
            let cfg = Config {
                a: input.next().unwrap(),
                b: input.next().unwrap(),
            };
            if cfg.a == 0 && cfg.b == 0 {
                break;
            }
            cfgs.push(cfg);
        }
        cfgs
    }
}

fn solve(cfg: Config) {
    let (a, b) = (cfg.a, cfg.b);
    println!("{}", a + b);
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let cfgs = Config::new(&input);
    for cfg in cfgs {
        solve(cfg);
    }
    Ok(())
}
