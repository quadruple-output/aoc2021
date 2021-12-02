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
    /// use this option to calculate the second puzzle of the day
    #[clap(short)]
    b: bool,
}

fn main() {
    let options = Options::parse();
    if options.input_folder == "-" {
        solve(options.day, options.b, &mut BufReader::new(stdin()));
    } else {
        let filepath = Path::new(&options.input_folder).join(format!("{:02}", options.day));
        match File::open(&filepath) {
            Ok(file) => solve(options.day, options.b, &mut BufReader::new(file)),
            Err(err) => eprintln!("Cannot open {:?}: {}", filepath, err),
        }
    };
}

fn solve(day: usize, part2: bool, input: &mut dyn BufRead) {
    match (day, part2) {
        (1, false) => day_01::solve_a(input),
        (1, true) => day_01::solve_b(input),
        (2, false) => day_02::solve_a(input),
        (2, true) => day_02::solve_b(input),
        _ => {
            eprintln!("Day {} is not implemented.", day);
            Ok(())
        }
    }
    .unwrap_or_else(|err| eprintln!("{}", err))
}
