use aoc_day::AoCDay;
use clap::Parser;
use std::path::PathBuf;

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
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<u8> for Day {
    type Error = InvalidDayError;

    fn try_from(day: u8) -> Result<Self, Self::Error> {
        match day {
            1 => Ok(Day::One),
            2 => Ok(Day::Two),
            3 => Ok(Day::Three),
            5 => Ok(Day::Five),
            6 => Ok(Day::Six),
            7 => Ok(Day::Seven),
            8 => Ok(Day::Eight),
            9 => Ok(Day::Nine),
            _ => Err(InvalidDayError(day)),
        }
    }
}

impl From<Day> for u8 {
    fn from(day: Day) -> Self {
        (day as u8) + 1
    }
}

impl From<&Day> for u8 {
    fn from(day: &Day) -> Self {
        (*day as u8) + 1
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Day:\t{:02}", u8::from(*self))
    }
}

impl Day {
    fn get_input_path(&self) -> PathBuf {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../inputs")
            .canonicalize()
            .unwrap();

        let day_num: u8 = self.into();
        let txt_name = format!("day{:02}", day_num);

        root.join(&txt_name).with_extension("txt")
    }

    fn get_aoc_day(&self) -> Box<dyn AoCDay> {
        match self {
            Self::One => Box::new(day01::Day01::default()),
            Self::Two => Box::new(day02::Day02::default()),
            Self::Three => Box::new(day03::Day03::default()),
            Self::Five => Box::new(day05::Day05::default()),
            Self::Six => Box::new(day06::Day06::default()),
            Self::Nine => Box::new(day09::Day09::default()),
            _ => panic!("Day not implemented yet"),
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
            2 => Ok(Self::Two),
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
    fn run(&self) -> anyhow::Result<()> {
        println!("{}", self);

        let mut aoc_day = self.day.get_aoc_day();
        let input_path = self.day.get_input_path();
        aoc_day.load_input(&input_path)?;

        match self.part {
            Part::One => {
                aoc_day.part1();
            }
            Part::Two => {
                aoc_day.part2();
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for RunConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Advent of Code 2024\n\
            ==========\
            \n{}\
            \n{}\
            \n\
            ==========",
            self.day, self.part
        )
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
        }
    };

    let part: Part = match cli.part.try_into() {
        Ok(part) => part,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let run_config = RunConfig { day, part };

    match run_config.run() {
        Ok(()) => std::process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(2);
        }
    }
}
