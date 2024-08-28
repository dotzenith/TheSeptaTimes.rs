mod endpoints;
mod stations;
mod traits;

use crate::endpoints::{
    Arrivals, NextToArrive, ScheduleDirection, ScheduleMode, ScheduleOuter, SeptaPlusPlusManager, TrainSchedule,
};
use crate::stations::Stations;
use crate::traits::{PrettyPrint, PrettyPrintWithMode};
use clap::{arg, command, value_parser, Command};

pub const URL: &str = "https://www3.septa.org/api";

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
                        .value_parser(value_parser!(u8))
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
                        .value_parser(value_parser!(u8))
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
        .subcommand(
            Command::new("extra")
                .about("All of the extra endpoints added by SepatPlusPlus")
                .subcommand(
                    Command::new("schedule")
                        .about("Get Schedule from one station to another on a given line")
                        .arg(arg!(line: [LINE]).help("The Regional Rail Line Code (e.g, TRE)"))
                        .arg(arg!(orig: [ORIG]).help("Starting Station (e.g, Trenton)"))
                        .arg(arg!(dest: [DEST]).help("Ending Station (e.g, Gray 30th Street)"))
                        .arg(arg!(direction: [inbound_or_outbound]).value_parser(value_parser!(ScheduleDirection)))
                        .arg(arg!(mode: [weekend_or_weekday]).value_parser(value_parser!(ScheduleMode))),
                )
                .subcommand(
                    Command::new("lines").about("Get all of the lines supported by the extra schedules endpoint"),
                )
                .subcommand(
                    Command::new("stations")
                        .about("Get all of the stations for a given line")
                        .arg(arg!(line: [LINE]).help("The Regional Rail Line Code (e.g, TRE)"))
                        .arg(
                            arg!(--direction <inbound_or_outbound>)
                                .value_parser(value_parser!(ScheduleDirection))
                                .default_value("inbound")
                                .default_missing_value("inbound")
                                .require_equals(false),
                        ),
                ),
        )
        .subcommand(
            Command::new("refresh")
                .about("Manually refresh the cache for station names (note: tst automatically refreshes every week)"),
        )
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
                    match NextToArrive::get(matching_start, matching_end, count) {
                        Ok(next) => next.print(),
                        Err(err) => {
                            eprintln!("An error occurred while getting next trains: {:?}", err);
                            std::process::exit(1)
                        }
                    }
                }
            }
        }
        Some(("arrivals", sub_matches)) => {
            let station = sub_matches.get_one::<String>("station").expect("required");
            let count = *sub_matches.get_one::<u8>("count").unwrap();
            match stations.fuzzy_search(station) {
                Ok(matching_station) => match Arrivals::get(matching_station, count) {
                    Ok(arr) => arr.print(),
                    Err(err) => {
                        eprintln!("An error occurred while getting arrivals: {:?}", err);
                        std::process::exit(1)
                    }
                },
                Err(_) => {
                    eprintln!("Invalid station, please use `tst stations` for all valid station names");
                    std::process::exit(1)
                }
            }
        }
        Some(("train", sub_matches)) => {
            let train_num = sub_matches.get_one::<String>("number").expect("required");
            match TrainSchedule::get(train_num) {
                Ok(train) => train.print(),
                Err(err) => {
                    eprintln!("An error occurred while getting train schedule: {:?}", err);
                    std::process::exit(1)
                }
            }
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
        Some(("extra", extra_matches)) => {
            let manager = match SeptaPlusPlusManager::new() {
                Ok(man) => man,
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(1)
                }
            };
            match extra_matches.subcommand() {
                Some(("schedule", sub_matches)) => {
                    let line = sub_matches.get_one::<String>("line").expect("required");
                    let orig = sub_matches.get_one::<String>("orig").expect("required");
                    let dest = sub_matches.get_one::<String>("dest").expect("required");
                    let direction = sub_matches.get_one::<ScheduleDirection>("direction").expect("required");
                    let mode = sub_matches.get_one::<ScheduleMode>("mode").expect("required");

                    match (
                        manager.fuzzy_match_station_for_line(line, orig, direction),
                        manager.fuzzy_match_station_for_line(line, dest, direction),
                    ) {
                        (Err(_), _) | (_, Err(_)) => {
                            eprintln!("Invalid station, please use `tst extra stations [LINE]` for all valid station names for a given line");
                            std::process::exit(1)
                        }
                        (Ok(matching_orig), Ok(matching_dest)) => {
                            match ScheduleOuter::get(line, direction, &matching_orig, &matching_dest) {
                                Ok(schedule) => schedule.print(mode),
                                Err(_) => {
                                    eprintln!(
                                        "An error occurred while getting train schedule, please check your inputs"
                                    );
                                    std::process::exit(1)
                                }
                            }
                        }
                    }
                }
                Some(("lines", _)) => match manager.get_lines() {
                    Ok(lines) => lines.print(),
                    Err(_) => {
                        eprintln!("An error occurred while getting lines, please check your SeptaPlusPlus URL");
                        std::process::exit(1)
                    }
                },

                Some(("stations", sub_matches)) => {
                    let line = sub_matches.get_one::<String>("line").expect("required");
                    let direction = sub_matches.get_one::<ScheduleDirection>("direction").unwrap();

                    match manager.get_stations_for_line(line, direction) {
                        Ok(stations) => {
                            for station in stations.iter() {
                                println!("{station}");
                            }
                        }
                        Err(_) => {
                            eprintln!("An error occurred while getting station, please check your SeptaPlusPlus URL and inputs");
                            std::process::exit(1)
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
