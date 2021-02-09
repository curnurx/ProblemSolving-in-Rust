use std::io;
use std::io::prelude::*;

struct Config {
    a: u32,
    b: u32,
}

impl Config {
    fn new(input: &str) -> Vec<Config> {
        let mut input = input
            .split_whitespace();

        let t: u32 = input.next().unwrap().parse().unwrap();

        let mut input = input
            .map(|x| x.parse::<u32>().unwrap());

        let mut cfgs: Vec<Config> = Vec::new();
        for _ in 0..t {
            cfgs.push(Config { 
                a: input.next().unwrap(), 
                b: input.next().unwrap(),
            });
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
