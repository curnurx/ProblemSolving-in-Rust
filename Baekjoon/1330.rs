use std::io;
use std::cmp::Ordering;

struct Config {
    a: i32,
    b: i32,
}

impl Config {
    fn new(input: &str) -> Config {
        let mut input = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Not a number"));

        Config {
            a: input.next().unwrap(),
            b: input.next().unwrap(),
        }
    }
}

fn solve(cfg: Config) {
    let (a, b) = (cfg.a, cfg.b);
    let result = match a.cmp(&b) {
        Ordering::Less => "<",
        Ordering::Equal => "==",
        Ordering::Greater => ">",
    };
    println!("{}", result);
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let cfg = Config::new(&input);
    Ok(solve(cfg))
}
