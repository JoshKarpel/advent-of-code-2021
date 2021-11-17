#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::error::Error;
use std::process::exit;

use clap::{App, Arg};
use itertools::Itertools;

use advent_of_code_2021::{
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11, day_12,
    day_13, day_14, day_15, day_16, day_17, day_18, day_19, day_20, day_21, day_22, day_23, day_24,
    day_25,
};

lazy_static! {
    static ref SOLVERS: HashMap<&'static str, fn() -> ()> = {
        let mut solvers = HashMap::new();

        solvers.insert("01", day_01::solve as fn() -> ());
        solvers.insert("02", day_02::solve as fn() -> ());
        solvers.insert("03", day_03::solve as fn() -> ());
        solvers.insert("04", day_04::solve as fn() -> ());
        solvers.insert("05", day_05::solve as fn() -> ());
        solvers.insert("06", day_06::solve as fn() -> ());
        solvers.insert("07", day_07::solve as fn() -> ());
        solvers.insert("08", day_08::solve as fn() -> ());
        solvers.insert("09", day_09::solve as fn() -> ());
        solvers.insert("10", day_10::solve as fn() -> ());
        solvers.insert("11", day_11::solve as fn() -> ());
        solvers.insert("12", day_12::solve as fn() -> ());
        solvers.insert("13", day_13::solve as fn() -> ());
        solvers.insert("14", day_14::solve as fn() -> ());
        solvers.insert("15", day_15::solve as fn() -> ());
        solvers.insert("16", day_16::solve as fn() -> ());
        solvers.insert("17", day_17::solve as fn() -> ());
        solvers.insert("18", day_18::solve as fn() -> ());
        solvers.insert("19", day_19::solve as fn() -> ());
        solvers.insert("20", day_20::solve as fn() -> ());
        solvers.insert("21", day_21::solve as fn() -> ());
        solvers.insert("22", day_22::solve as fn() -> ());
        solvers.insert("23", day_23::solve as fn() -> ());
        solvers.insert("24", day_24::solve as fn() -> ());
        solvers.insert("25", day_25::solve as fn() -> ());

        solvers
    };
}

fn run_solver(day: &str) {
    if let Some(solver) = SOLVERS.get(day) {
        println!("== Day {} =====", day);
        println!(
            "== https://adventofcode.com/2021/day/{}",
            day.trim_start_matches('0')
        );
        solver();
    } else {
        println!("Unknown day: {}", day);
        exit(1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Advent of Code 2021")
        .version("0.1.0")
        .author("Josh Karpel <josh.karpel@gmail.com>")
        .about("Josh's solutions for Advent of Code 2021.")
        .arg(
            Arg::with_name("DAY")
                .help("The day to run the solver for.")
                .index(1),
        )
        .get_matches();

    if let Some(day) = matches.value_of("DAY").map(|d| format!("{:0>2}", d)) {
        run_solver(&day)
    } else {
        SOLVERS.keys().sorted().for_each(|day| {
            run_solver(day);
            println!();
        })
    }

    Ok(())
}
