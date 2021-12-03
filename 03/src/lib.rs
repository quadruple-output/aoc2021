use eyre::{eyre, Result};
use std::io::BufRead;

pub fn solve_a(input: &mut dyn BufRead) -> Result<()> {
    let mut bit_counts = [0; 16]; // would work with any length >= 12
    let mut bit_mask = 0;
    let number_count = tools::numbers_from_lines(input, 2)? // parses numbers with radix 2
        .into_iter()
        .inspect(|&number| {
            bit_counts
                .iter_mut()
                .enumerate()
                .for_each(|(bit_pos, bit_count)| *bit_count += (number >> bit_pos) & 1);
            bit_mask |= number;
        })
        .count();
    let gamma = bit_counts
        .iter()
        .enumerate()
        .fold(0, |accu, (bit_pos, bit_count)| {
            accu | ((bit_count * 2 / number_count) << bit_pos)
        });
    let epsilon = !gamma & bit_mask; // fails if a bit exists that is not set in *any* number
    println!("Power Consumption: {}", gamma * epsilon);
    Ok(())
}

pub fn solve_b(_input: &mut dyn BufRead) -> Result<()> {
    Err(eyre!("not implemented"))
}
