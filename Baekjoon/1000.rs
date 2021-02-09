use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let sum:u32 = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().expect("Not a number"))
        .sum();

    println!("{}", sum);
    Ok(())
}
