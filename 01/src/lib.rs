use ::eyre::{eyre, Result};
use ::itertools::Itertools;
use ::std::io::BufRead;

pub fn solve_a(input: &mut dyn BufRead) -> Result<()> {
    let count_increasing_pairs = numbers_from_lines(input)?
        .iter()
        .tuple_windows::<(_, _)>()
        .filter(|(prev, next)| prev < next)
        .count();
    println!("{}", count_increasing_pairs);
    Ok(())
}

pub fn solve_b(input: &mut dyn BufRead) -> Result<()> {
    let count_increasing_window_avrg = numbers_from_lines(input)?
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|w| w.0 + w.1 + w.2)
        .tuple_windows::<(_, _)>()
        .filter(|(prev, next)| prev < next)
        .count();
    println!("{}", count_increasing_window_avrg);
    Ok(())
}

fn numbers_from_lines(input: &mut dyn BufRead) -> Result<Vec<usize>> {
    let mut numbers = Vec::new();
    for number_or_err in input.lines().enumerate().map(|(n, maybe_line)| {
        let line_count = n + 1;
        maybe_line
            .map_err(|io_err| eyre!("error reading line {}: {}", line_count, io_err))
            .and_then(|line| {
                line.parse::<usize>().map_err(|parse_err| {
                    eyre!("error parsing line {} as usize: {}", line_count, parse_err)
                })
            })
    }) {
        numbers.push(number_or_err?);
    }
    Ok(numbers)
}
