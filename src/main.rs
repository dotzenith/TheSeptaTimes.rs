mod septa;
mod septum;
mod stations;
mod traits;
mod utils;

use crate::septa::{Arrivals, NextToArrive, TrainSchedule};
use crate::septum::{ScheduleDirection, ScheduleMode, ScheduleOuter, SeptumMisc};
use crate::stations::StationsManager;
use crate::traits::{PrettyPrint, PrettyPrintWithMode};
use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use colored::Colorize;
use std::io;

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
        #[arg(long, short, default_value = "5")]
        count: u8,
    },

    /// Find the next arrivals at a given train station
    Arrivals {
        /// Station name
        station: String,

        /// Number of results to return
        #[arg(long, short, default_value = "5")]
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

    /// Generate shell completions
    Completions {
        /// The shell to generate completions for
        shell: Shell,
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

        /// Trains going inbound
        #[arg(long, group = "direction")]
        inbound: bool,

        /// Trains going outbound
        #[arg(long, group = "direction")]
        outbound: bool,

        /// Weekday trains
        #[arg(long, group = "week")]
        weekday: bool,

        /// Weekend trains
        #[arg(long, group = "week")]
        weekend: bool,
    },

    /// Get all of the lines supported by the extra schedules endpoint
    Lines,

    /// Get all of the stations for a given line
    Stations {
        /// The Regional Rail Line Code (e.g, TRE)
        line: String,

        /// Trains going inbound
        #[arg(long, group = "direction")]
        inbound: bool,

        /// Trains going outbound
        #[arg(long, group = "direction")]
        outbound: bool,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut stations = StationsManager::new();
    let cli = Cli::parse();

    match cli.command {
        Commands::Next { from, to, count } => {
            let matching_from = stations
                .fuzzy_search(&from)
                .context("Invalid station, please use `tst stations` for all valid station names")?;
            let matching_to = stations
                .fuzzy_search(&to)
                .context("Invalid station, please use `tst stations` for all valid station names")?;
            let result = NextToArrive::get(&matching_from, &matching_to, count).context("Failed to get next trains")?;
            result.print();
        }
        Commands::Arrivals { station, count } => {
            let matching_station = stations
                .fuzzy_search(&station)
                .context("Invalid station, please use `tst stations` for all valid station names")?;
            let result = Arrivals::get(&matching_station, count).context("Failed to get arrivals")?;
            result.print();
        }
        Commands::Train { number } => {
            let result = TrainSchedule::get(&number).context("Failed to get train schedule")?;

            result.print();
        }
        Commands::Stations => {
            for station in stations.get_stations().iter() {
                println!("{station}");
            }
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "tst", &mut io::stdout());
        }
        Commands::Extra { command } => {
            let mut manager = SeptumMisc::new()?;

            match command {
                ExtraCommands::Schedule {
                    line,
                    orig,
                    dest,
                    inbound,
                    outbound,
                    weekday,
                    weekend,
                } => {
                    let direction = match (inbound, outbound) {
                        (true, _) => ScheduleDirection::Inbound,
                        (_, true) => ScheduleDirection::Outbound,
                        (_, _) => ScheduleDirection::Inbound,
                    };

                    let mode = match (weekday, weekend) {
                        (true, _) => ScheduleMode::Weekday,
                        (_, true) => ScheduleMode::Weekend,
                        (_, _) => ScheduleMode::Weekday,
                    };

                    let matching_orig = manager
                        .fuzzy_match_station_for_line(&line, &orig, &direction)
                        .context("Invalid station, please use `tst extra stations [LINE]` for all valid station names for a given line")?;
                    let matching_dest = manager
                        .fuzzy_match_station_for_line(&line, &dest, &direction)
                        .context("Invalid station, please use `tst extra stations [LINE]` for all valid station names for a given line")?;
                    let result = ScheduleOuter::get(&line, &direction, &matching_orig, &matching_dest)
                        .context("An error occurred while getting train schedule, please double check the direction")?;
                    result.print(&mode);
                }
                ExtraCommands::Lines => {
                    let result = manager
                        .get_lines()
                        .context("An error occurred while getting lines, please check your Septum URL")?;
                    result.print();
                }
                ExtraCommands::Stations {
                    line,
                    inbound,
                    outbound,
                } => {
                    let direction = match (inbound, outbound) {
                        (true, _) => ScheduleDirection::Inbound,
                        (_, true) => ScheduleDirection::Outbound,
                        (_, _) => ScheduleDirection::Inbound,
                    };
                    let stations = manager
                        .get_stations_for_line(&line, &direction)
                        .context("An error occurred while getting station, please check your Septum URL and inputs")?;
                    for station in stations.iter() {
                        println!("{station}");
                    }
                }
            }
        }
    }
    Ok(())
}
