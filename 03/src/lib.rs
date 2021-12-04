use eyre::{eyre, Result};
use std::io::BufRead;

pub fn solve_a(input: &mut dyn BufRead) -> Result<()> {
    let numbers = tools::numbers_from_lines(input, 2)?;
    let mut bit_counters = [0; usize::BITS as usize]; // be prepared for numbers of "any size"
    let number_of_bits = max_number_of_bits(&numbers);
    let number_count = numbers
        .iter()
        .inspect(|&number| {
            bit_counters
                .iter_mut()
                .take(number_of_bits as usize)
                .enumerate()
                .for_each(|(bit_pos, bit_count)| *bit_count += (number >> bit_pos) & 1);
        })
        .count();

    let gamma = bit_counters
        .iter()
        .take(number_of_bits as usize)
        .enumerate()
        .fold(0, |accu, (bit_pos, bit_count)| {
            accu | ((bit_count * 2 / number_count) << bit_pos)
        });

    let bit_mask = (1 << number_of_bits) - 1;
    let epsilon = !gamma & bit_mask; // fails if a bit exists that is not set in *any* number

    println!("Power Consumption: {}", gamma * epsilon);
    Ok(())
}

pub fn solve_b(input: &mut dyn BufRead) -> Result<()> {
    let numbers = tools::numbers_from_lines(input, 2)?;
    let number_of_bits = max_number_of_bits(&numbers);

    let oxygen_generator_rating = find_rating(numbers.clone(), false, number_of_bits)?;
    let co2_scrubber_rating = find_rating(numbers, true, number_of_bits)?;
    println!(
        "Life support rating: {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
    Ok(())
}

fn find_rating(mut candidates: Vec<usize>, invert: bool, number_of_bits: u32) -> Result<usize> {
    for bit_pos in (0..number_of_bits).rev() {
        let bit_mask = 1 << bit_pos;
        let bit_count = candidates
            .iter()
            .filter(|&candidate| candidate & bit_mask != 0)
            .count();
        let criterion = (bit_count * 2 / candidates.len()) << bit_pos;
        candidates.retain(|candidate| (candidate & bit_mask == criterion) ^ invert);
        if candidates.len() == 1 {
            return Ok(candidates[0]);
        }
    }
    Err(eyre!("could not find rating"))
}

fn max_number_of_bits(numbers: &[usize]) -> u32 {
    let largest_number = numbers.iter().max().cloned().unwrap_or(0);
    (0..usize::BITS)
        .rev()
        .find(|bit_pos| largest_number & (1 << bit_pos) != 0)
        .unwrap_or(0)
        + 1
}
