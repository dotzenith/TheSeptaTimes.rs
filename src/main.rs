mod api;
mod parse;
mod data;
mod print;

use crate::data::{Arrival, NextToArrive, TrainSchedule, Stations};
use crate::print::Print;
use std::env;

fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();
    let str_argv: Vec<&str> = argv.iter().map(String::as_str).collect();

    let stations = Stations::new();

    match str_argv.as_slice() {
        ["-h" | "--help", ..] => {
            println!("Usage: tst [OPTIONS] COMMAND [ARGS]...\n\n\
                Options:\n  \
                --help  Show this message and exit.\n\n\
                Commands:\n  \
                  arrivals  Find the next arrivals at a given train station\n  \
                  next      Search for the next train going from an origin to a destination\n  \
                  stations  Get all valid station names\n  \
                  train     Track a given train using it's number\n");
            std::process::exit(0);
        },
        ["arrivals", station] => {
            if !stations.exists(station) {
                eprintln!("Invalid station, please use `tst stations` for all valid station names");
                std::process::exit(1);
            }
            Arrival::print(Arrival::get(station, 5));
        },
        ["arrivals", station, num] => {
            if !stations.exists(station) {
                eprintln!("Invalid station, please use `tst stations` for all valid station names");
                std::process::exit(1);
            }
            Arrival::print(Arrival::get(station, num.parse::<u8>().unwrap_or(5)));
        },
        ["next", start, end] => {
            if !stations.exists(start) || !stations.exists(end) {
                eprintln!("Invalid station, please use `tst stations` for all valid station names");
                std::process::exit(1);
            }
            NextToArrive::print(NextToArrive::get(start, end, 5));
        },
        ["next", start, end, num] => {
            if !stations.exists(start) || !stations.exists(end) {
                eprintln!("Invalid station, please use `tst stations` for all valid station names");
                std::process::exit(1);
            }
            NextToArrive::print(NextToArrive::get(start, end, num.parse::<u8>().unwrap_or(5)));
        },
        ["train", train_num] => {
            TrainSchedule::print(TrainSchedule::get(train_num));
        },
        ["stations"] => {
            for station in stations.stations().iter() {
                println!("{station}");
            }
            std::process::exit(0);
        }
        [..] => {
            println!("Invalid command: use --help or -h for usage details");
            std::process::exit(1);
        }
    }
}
