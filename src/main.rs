mod api;
mod data;
mod parse;
mod print;

use crate::data::{Arrival, NextToArrive, Stations, TrainSchedule};
use crate::print::Print;
use std::env;

fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();
    let str_argv: Vec<&str> = argv.iter().map(String::as_str).collect();

    let stations = Stations::new();

    match str_argv.as_slice() {
        ["-h" | "--help", ..] => {
            println!(
                "Usage: tst [OPTIONS] COMMAND [ARGS]...\n\n\
                Options:\n  \
                --help  Show this message and exit.\n\n\
                Commands:\n  \
                  next      Search for the next train going from an origin to a destination\n  \
                  arrivals  Find the next arrivals at a given train station\n  \
                  train     Track a given train using it's number\n  \
                  stations  Get all valid station names\n  \
                  refresh   Refresh the cache for station names\n"
            );
        }
        ["arrivals", station] => match stations.fuzzy_search(station) {
            Ok(matching_station) => Arrival::print(Arrival::get(matching_station, 5)),
            Err(_) => {
                eprintln!("Invalid station, please use `tst stations` for all valid station names");
                std::process::exit(1)
            }
        },
        ["arrivals", station, num] => match stations.fuzzy_search(station) {
            Ok(matching_station) => Arrival::print(Arrival::get(matching_station, num.parse::<u8>().unwrap_or(5))),
            Err(_) => {
                eprintln!("Invalid station, please use `tst stations` for all valid station names");
                std::process::exit(1)
            }
        },
        ["next", start, end] => match (stations.fuzzy_search(start), stations.fuzzy_search(end)) {
            (Err(_), _) | (_, Err(_)) => {
                eprintln!("Invalid station, please use `tst stations` for all valid station names");
                std::process::exit(1)
            }
            (Ok(matching_start), Ok(matching_end)) => {
                NextToArrive::print(NextToArrive::get(matching_start, matching_end, 5));
            }
        },
        ["next", start, end, num] => match (stations.fuzzy_search(start), stations.fuzzy_search(end)) {
            (Err(_), _) | (_, Err(_)) => {
                eprintln!("Invalid station, please use `tst stations` for all valid station names");
                std::process::exit(1)
            }
            (Ok(matching_start), Ok(matching_end)) => {
                NextToArrive::print(NextToArrive::get(
                    matching_start,
                    matching_end,
                    num.parse::<u8>().unwrap_or(5),
                ));
            }
        },
        ["train", train_num] => {
            TrainSchedule::print(TrainSchedule::get(train_num));
        }
        ["stations"] => {
            for station in stations.get_stations().iter() {
                println!("{station}");
            }
        }
        ["refresh"] => match Stations::refresh() {
            Ok(_) => println!("Successfully updated the cache for station names"),
            Err(_) => println!("Unable to update the cache for station names"),
        },
        [..] => {
            println!("Invalid command: use --help or -h for usage details");
            std::process::exit(1);
        }
    }
}
