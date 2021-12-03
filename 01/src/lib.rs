use ::eyre::Result;
use ::itertools::Itertools;
use ::std::io::BufRead;
use ::tools::numbers_from_lines;

pub fn solve_a(input: &mut dyn BufRead) -> Result<()> {
    println!(
        "Increasing measurements: {}",
        numbers_from_lines(input, 10)?
            .iter()
            .tuple_windows()
            .filter(|(prev, next)| prev < next)
            .count()
    );
    Ok(())
}

pub fn solve_b(input: &mut dyn BufRead) -> Result<()> {
    println!(
        "Increasing sliding averages: {}",
        numbers_from_lines(input, 10)?
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(prev, next)| prev < next)
            .count()
    );
    Ok(())
}
