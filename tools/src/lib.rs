use ::eyre::{eyre, Result};
use ::std::io::BufRead;

pub fn numbers_from_lines(input: &mut dyn BufRead, radix: u32) -> Result<Vec<usize>> {
    let mut numbers = Vec::new();
    for number_or_err in input.lines().enumerate().map(|(n, maybe_line)| {
        let line_count = n + 1;
        maybe_line
            .map_err(|io_err| eyre!("error reading line {}: {}", line_count, io_err))
            .and_then(|line| {
                usize::from_str_radix(&line, radix).map_err(|parse_err| {
                    eyre!("error parsing line {} as usize: {}", line_count, parse_err)
                })
            })
    }) {
        numbers.push(number_or_err?);
    }
    Ok(numbers)
}
