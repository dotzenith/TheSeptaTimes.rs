mod api;
mod data;
mod parse;
mod print;

use crate::data::{Arrival, NextToArrive, Stations, TrainSchedule};
use crate::print::Print;
use clap::{arg, command, Command};

fn main() {
    let stations = Stations::new();

    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("next")
                .about("Search for the next train going from an origin to a destination")
                .arg(arg!(from: [STATION]))
                .arg(arg!(to: [STATION]))
                .arg(
                    arg!(--count <NUM>)
                        .value_parser(clap::value_parser!(u8))
                        .default_value("5")
                        .default_missing_value("5")
                        .require_equals(false),
                ),
        )
        .subcommand(
            Command::new("arrivals")
                .about("Find the next arrivals at a given train station")
                .arg(arg!(station: [STATION]))
                .arg(
                    arg!(--count <NUM>)
                        .value_parser(clap::value_parser!(u8))
                        .default_value("5")
                        .default_missing_value("5")
                        .require_equals(false),
                ),
        )
        .subcommand(
            Command::new("train")
                .about("Track a given train")
                .arg(arg!(number: [TRAIN_NUM])),
        )
        .subcommand(Command::new("stations").about("Get all valid station names"))
        .subcommand(Command::new("refresh").about("Refresh the cache for station names"))
        .get_matches();

    match matches.subcommand() {
        Some(("next", sub_matches)) => {
            let from = sub_matches.get_one::<String>("from").expect("required");
            let to = sub_matches.get_one::<String>("to").expect("required");
            let count = *sub_matches.get_one::<u8>("count").unwrap();
            match (stations.fuzzy_search(from), stations.fuzzy_search(to)) {
                (Err(_), _) | (_, Err(_)) => {
                    eprintln!("Invalid station, please use `tst stations` for all valid station names");
                    std::process::exit(1)
                }
                (Ok(matching_start), Ok(matching_end)) => {
                    NextToArrive::print(NextToArrive::get(matching_start, matching_end, count));
                }
            }
        }
        Some(("arrivals", sub_matches)) => {
            let station = sub_matches.get_one::<String>("station").expect("required");
            let count = *sub_matches.get_one::<u8>("count").unwrap();
            match stations.fuzzy_search(station) {
                Ok(matching_station) => Arrival::print(Arrival::get(matching_station, count)),
                Err(_) => {
                    eprintln!("Invalid station, please use `tst stations` for all valid station names");
                    std::process::exit(1)
                }
            }
        }
        Some(("train", sub_matches)) => {
            let train_num = sub_matches.get_one::<String>("number").expect("required");
            TrainSchedule::print(TrainSchedule::get(train_num));
        }
        Some(("stations", _)) => {
            for station in stations.get_stations().iter() {
                println!("{station}");
            }
        }
        Some(("refresh", _)) => match Stations::refresh() {
            Ok(_) => println!("Successfully updated the cache for station names"),
            Err(_) => println!("Unable to update the cache for station names"),
        },
        _ => unreachable!(),
    }
}
