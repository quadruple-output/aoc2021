use ::eyre::Result;
use ::itertools::Itertools;
use ::std::io::BufRead;
use ::tools::numbers_from_lines;

pub fn solve_a(input: &mut dyn BufRead) -> Result<usize> {
    Ok(numbers_from_lines(input, 10)?
        .iter()
        .tuple_windows()
        .filter(|(prev, next)| prev < next)
        .count())
}

pub fn solve_b(input: &mut dyn BufRead) -> Result<usize> {
    Ok(numbers_from_lines(input, 10)?
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(prev, next)| prev < next)
        .count())
}
