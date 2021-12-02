use eyre::{eyre, Result};
use std::io::BufRead;

pub fn solve_a(input: &mut dyn BufRead) -> Result<()> {
    let mut position: usize = 0;
    let mut depth: usize = 0;
    for line in input.lines() {
        let line = line?;
        let mut tokens = line.split_whitespace();
        let command = tokens.next().unwrap();
        let value = tokens.next().unwrap();
        let value = value.parse::<usize>()?;
        match command {
            "forward" => position += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => return Err(eyre!("unknown command '{}'", command)),
        }
    }
    println!("Product of Position and Depth: {}", position * depth);
    Ok(())
}

pub fn solve_b(input: &mut dyn BufRead) -> Result<()> {
    let mut position: usize = 0;
    let mut depth: usize = 0;
    let mut aim: isize = 0;
    for line in input.lines() {
        let line = line?;
        let mut tokens = line.split_whitespace();
        let command = tokens.next().unwrap();
        let value = tokens.next().unwrap();
        let x = value.parse::<usize>()?;
        match command {
            "down" => aim += x as isize,
            "up" => aim -= x as isize,
            "forward" => {
                position += x;
                depth += (aim * x as isize) as usize;
            }
            _ => return Err(eyre!("unknown command '{}'", command)),
        }
    }
    println!("Product of Position and Depth: {}", position * depth);
    Ok(())
}
