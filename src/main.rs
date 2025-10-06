mod septa;
mod septum;
mod stations;
mod traits;

use crate::septa::{Arrivals, NextToArrive, TrainSchedule};
use crate::septum::{ScheduleDirection, ScheduleMode, ScheduleOuter, SeptumMisc};
use crate::stations::StationsManager;
use crate::traits::{PrettyPrint, PrettyPrintWithMode};
use clap::{Parser, Subcommand};

pub const URL: &str = "https://www3.septa.org/api";

#[derive(Parser)]
#[command(name = "tst")]
#[command(about, version, author)]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for the next train going from an origin to a destination
    Next {
        /// Starting station
        from: String,

        /// Destination station
        to: String,

        /// Number of results to return
        #[arg(long, default_value = "5")]
        count: u8,
    },

    /// Find the next arrivals at a given train station
    Arrivals {
        /// Station name
        station: String,

        /// Number of results to return
        #[arg(long, default_value = "5")]
        count: u8,
    },

    /// Track a given train
    Train {
        /// Train number
        number: String,
    },

    /// Get all valid station names
    Stations,

    /// All of the extra endpoints added by Septum
    Extra {
        #[command(subcommand)]
        command: ExtraCommands,
    },
}

#[derive(Subcommand)]
enum ExtraCommands {
    /// Get Schedule from one station to another on a given line
    Schedule {
        /// The Regional Rail Line Code (e.g, TRE)
        line: String,

        /// Starting Station (e.g, Trenton)
        orig: String,

        /// Ending Station (e.g, Gray 30th Street)
        dest: String,

        /// Direction (inbound or outbound)
        direction: ScheduleDirection,

        /// Mode (weekend or weekday)
        mode: ScheduleMode,
    },

    /// Get all of the lines supported by the extra schedules endpoint
    Lines,

    /// Get all of the stations for a given line
    Stations {
        /// The Regional Rail Line Code (e.g, TRE)
        line: String,

        /// Direction (inbound or outbound)
        #[arg(long, default_value = "inbound")]
        direction: ScheduleDirection,
    },
}

fn main() {
    let stations = StationsManager::new();
    let cli = Cli::parse();

    match cli.command {
        Commands::Next { from, to, count } => match (stations.fuzzy_search(&from), stations.fuzzy_search(&to)) {
            (Err(_), _) | (_, Err(_)) => {
                eprintln!("Invalid station, please use `tst stations` for all valid station names");
                std::process::exit(1)
            }
            (Ok(matching_start), Ok(matching_end)) => match NextToArrive::get(matching_start, matching_end, count) {
                Ok(next) => next.print(),
                Err(err) => {
                    eprintln!("An error occurred while getting next trains: {:?}", err);
                    std::process::exit(1)
                }
            },
        },
        Commands::Arrivals { station, count } => match stations.fuzzy_search(&station) {
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
        },
        Commands::Train { number } => match TrainSchedule::get(&number) {
            Ok(train) => train.print(),
            Err(err) => {
                eprintln!("An error occurred while getting train schedule: {:?}", err);
                std::process::exit(1)
            }
        },
        Commands::Stations => {
            for station in stations.get_stations().iter() {
                println!("{station}");
            }
        }
        Commands::Extra { command } => {
            let manager = match SeptumMisc::new() {
                Ok(man) => man,
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(1)
                }
            };

            match command {
                ExtraCommands::Schedule {
                    line,
                    orig,
                    dest,
                    direction,
                    mode,
                } => {
                    match (
                        manager.fuzzy_match_station_for_line(&line, &orig, &direction),
                        manager.fuzzy_match_station_for_line(&line, &dest, &direction),
                    ) {
                        (Err(_), _) | (_, Err(_)) => {
                            eprintln!(
                                "Invalid station, please use `tst extra stations [LINE]` for all valid station names for a given line"
                            );
                            std::process::exit(1)
                        }
                        (Ok(matching_orig), Ok(matching_dest)) => {
                            match ScheduleOuter::get(&line, &direction, &matching_orig, &matching_dest) {
                                Ok(schedule) => schedule.print(&mode),
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
                ExtraCommands::Lines => match manager.get_lines() {
                    Ok(lines) => lines.print(),
                    Err(_) => {
                        eprintln!("An error occurred while getting lines, please check your Septum URL");
                        std::process::exit(1)
                    }
                },
                ExtraCommands::Stations { line, direction } => match manager.get_stations_for_line(&line, &direction) {
                    Ok(stations) => {
                        for station in stations.iter() {
                            println!("{station}");
                        }
                    }
                    Err(_) => {
                        eprintln!("An error occurred while getting station, please check your Septum URL and inputs");
                        std::process::exit(1)
                    }
                },
            }
        }
    }
}
