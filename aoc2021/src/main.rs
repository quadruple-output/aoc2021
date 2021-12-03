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
        (3, false) => day_03::solve_a(input),
        (3, true) => day_03::solve_b(input),
        (4, false) => day_04::solve_a(input),
        (4, true) => day_04::solve_b(input),
        (5, false) => day_05::solve_a(input),
        (5, true) => day_05::solve_b(input),
        (6, false) => day_06::solve_a(input),
        (6, true) => day_06::solve_b(input),
        (7, false) => day_07::solve_a(input),
        (7, true) => day_07::solve_b(input),
        (8, false) => day_08::solve_a(input),
        (8, true) => day_08::solve_b(input),
        (9, false) => day_09::solve_a(input),
        (9, true) => day_09::solve_b(input),
        (10, false) => day_10::solve_a(input),
        (10, true) => day_10::solve_b(input),
        (11, false) => day_11::solve_a(input),
        (11, true) => day_11::solve_b(input),
        (12, false) => day_12::solve_a(input),
        (12, true) => day_12::solve_b(input),
        (13, false) => day_13::solve_a(input),
        (13, true) => day_13::solve_b(input),
        (14, false) => day_14::solve_a(input),
        (14, true) => day_14::solve_b(input),
        (15, false) => day_15::solve_a(input),
        (15, true) => day_15::solve_b(input),
        (16, false) => day_16::solve_a(input),
        (16, true) => day_16::solve_b(input),
        (17, false) => day_17::solve_a(input),
        (17, true) => day_17::solve_b(input),
        (18, false) => day_18::solve_a(input),
        (18, true) => day_18::solve_b(input),
        (19, false) => day_19::solve_a(input),
        (19, true) => day_19::solve_b(input),
        (20, false) => day_20::solve_a(input),
        (20, true) => day_20::solve_b(input),
        (21, false) => day_21::solve_a(input),
        (21, true) => day_21::solve_b(input),
        (22, false) => day_22::solve_a(input),
        (22, true) => day_22::solve_b(input),
        (23, false) => day_23::solve_a(input),
        (23, true) => day_23::solve_b(input),
        (24, false) => day_24::solve_a(input),
        (24, true) => day_24::solve_b(input),
        (25, false) => day_25::solve_a(input),
        (25, true) => day_25::solve_b(input),
        _ => {
            eprintln!("Day {} is not implemented.", day);
            Ok(())
        }
    }
    .unwrap_or_else(|err| eprintln!("{}", err))
}
