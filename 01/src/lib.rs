use ::eyre::{eyre, Result};
use ::std::io::BufRead;

pub fn solve(input: &mut dyn BufRead) -> Result<()> {
    let mut lines = input.lines();
    if let Some(first_line) = lines.next() {
        let mut count = 0;
        let mut previous_measurement = first_line?.parse::<usize>()?;
        for line in lines {
            let this_measurement = line?.parse::<usize>()?;
            if this_measurement > previous_measurement {
                count += 1;
            }
            previous_measurement = this_measurement;
        }
        println!("{}", count);
        Ok(())
    } else {
        Err(eyre!("no input lines"))
    }
}
