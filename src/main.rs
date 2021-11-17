#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::error::Error;
use std::process::exit;

use clap::{App, Arg};

use advent_of_code_2021::day_01;
use advent_of_code_2021::day_02;

lazy_static! {
    static ref SOLVERS: HashMap<&'static str, fn() -> ()> = {
        let mut solvers = HashMap::new();

        solvers.insert("01", day_01::solve as fn() -> ());
        solvers.insert("02", day_02::solve as fn() -> ());

        solvers
    };
}

fn run_solver(day: &str) {
    if let Some(solver) = SOLVERS.get(day) {
        println!("====== Day {}", day);
        solver()
    } else {
        println!("Unknown day: {}", day);
        exit(1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Advent of Code 2021")
        .version("0.1.0")
        .author("Josh Karpel <josh.karpel@gmail.com>")
        .about("Josh's solutions for Advent of Code 2021")
        .arg(
            Arg::with_name("DAY")
                .help("The day to run the solver for.")
                .index(1),
        )
        .get_matches();

    if let Some(day) = matches.value_of("DAY").map(|d| format!("{:0>2}", d)) {
        run_solver(&day)
    } else {
        SOLVERS.keys().for_each(|day| run_solver(day))
    }

    Ok(())
}
