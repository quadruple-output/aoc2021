use ::clap::Parser;
use ::std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::Path,
};

#[derive(Parser)]
struct Options {
    /// Sets the day for which the solution shall be calculated
    #[clap(short, long)]
    day: usize,
    /// The path to the folder containing the input data files.
    /// The name of the file is expected to be the day as a two-digit number,
    /// padded with leading zeros.
    /// Use '-' to read from STDIN.
    #[clap(short, long, default_value = "input")]
    input_folder: String,
}

fn main() {
    let options = Options::parse();
    if options.input_folder == "-" {
        solve(options.day, &mut BufReader::new(stdin()));
    } else {
        let filepath = Path::new(&options.input_folder).join(format!("{:02}", options.day));
        match File::open(&filepath) {
            Ok(file) => solve(options.day, &mut BufReader::new(file)),
            Err(err) => eprintln!("Cannot open {:?}: {}", filepath, err),
        }
    };
}

fn solve(day: usize, input: &mut dyn BufRead) {
    match day {
        1 => day_01::solve(input),
        _ => {
            eprintln!("Day {} is not implemented.", day);
            Ok(())
        }
    }
    .unwrap_or_else(|err| eprintln!("{}", err))
}
