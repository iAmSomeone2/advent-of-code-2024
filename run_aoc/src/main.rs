use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct InvalidDayError(u8);

impl std::fmt::Display for InvalidDayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid day: {}", self.0)
    }
}

impl std::error::Error for InvalidDayError {}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Day {
    One
}

impl TryFrom<u8> for Day {
    type Error = InvalidDayError;

    fn try_from(day: u8) -> Result<Self, Self::Error> {
        match day {
            1 => Ok(Day::One),
            _ => Err(InvalidDayError(day)),
        }
    }
}

impl From<Day> for u8 {
    fn from(day: Day) -> Self {
        (day as u8) + 1
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Day:\t{:02}", u8::from(*self))
    }
}

impl Day {
    fn get_input_path(&self) -> PathBuf {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../inputs").canonicalize().unwrap();
        match self {
            Self::One => root.join("day01.txt")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct InvalidPartError(u8);

impl std::fmt::Display for InvalidPartError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid part: {}", self.0)
    }
}

impl std::error::Error for InvalidPartError {}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Part {
    One,
    Two,
}

impl TryFrom<u8> for Part {
    type Error = InvalidPartError;

    fn try_from(part: u8) -> Result<Self, Self::Error> {
        match part {
            1 => Ok(Self::One),
            _ => Err(InvalidPartError(part)),
        }
    }
}

impl From<Part> for u8 {
    fn from(part: Part) -> Self {
        (part as u8) + 1
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Part:\t{:>2}", u8::from(*self))
    }
}

struct RunConfig {
    day: Day,
    part: Part,
}

impl RunConfig {
    fn load_text_input(&self) -> std::io::Result<String> {
        let input_path = self.day.get_input_path();
        std::fs::read_to_string(input_path)
    }

    fn run(&self) -> std::io::Result<()> {
        println!("{}", self);

        let input = self.load_text_input()?;
        match self.day {
            Day::One => {
                match self.part {
                    Part::One => {
                        day01::part1(&input);
                    },
                    Part::Two => {
                        todo!();
                    }
                }
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for RunConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "==========\n{}\n{}\n==========", self.day, self.part)
    }
}

/// Advent of Code 2024 runner
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Which day to run
    day: u8,
    /// Which part to run
    #[arg(short, long, default_value = "1")]
    part: u8,
}

fn main() {
    let cli = Args::parse();

    let day: Day = match cli.day.try_into() {
        Ok(day) => day,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        },
    };

    let part: Part = match cli.part.try_into() {
        Ok(part) => part,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let run_config = RunConfig {
        day,
        part,
    };

    match run_config.run() {
        Ok(()) => std::process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(2);
        }
    }
}
