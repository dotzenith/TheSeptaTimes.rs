use std::error::Error;

use crate::data::{Arrival, NextToArrive, TrainSchedule};
use crate::parse::Parse;
use colored::Colorize;

pub trait Print {
    fn print(result: Result<Self, Box<dyn Error>>)
    where
        Self: Sized;
}

impl Print for Arrival {
    fn print(result: Result<Self, Box<dyn Error>>) {
        match result {
            Ok(arrival) => {
                println!(
                    "{:<13}{:<11}{:<27}{:<12}{:<10}{}",
                    "Direction".blue(),
                    "Train #".cyan(),
                    "Next Station".green(),
                    "Time".magenta(),
                    "Status".red(),
                    "Destination".yellow(),
                );
                for train in arrival.parse().iter() {
                    println!("{train}");
                }
                std::process::exit(0);
            }
            Err(err) => {
                eprintln!("An error occurred while getting arrivals: {}", err);
                std::process::exit(1);
            }
        }
    }
}

impl Print for NextToArrive {
    fn print(result: Result<Self, Box<dyn Error>>) {
        match result {
            Ok(next_to_arrive) => {
                println!(
                    "{:<11}{:<13}{:<11}{:<9}{}",
                    "Train #".cyan(),
                    "Departure".green(),
                    "Arrival".magenta(),
                    "Delay".red(),
                    "Line".yellow(),
                );
                for train in next_to_arrive.parse().iter() {
                    println!("{train}");
                }
                std::process::exit(0);
            }
            Err(err) => {
                eprintln!("An error occurred while getting next trains: {}", err);
                std::process::exit(1);
            }
        }
    }
}

impl Print for TrainSchedule {
    fn print(result: Result<Self, Box<dyn Error>>) {
        match result {
            Ok(train_schedule) => {
                println!(
                    "{:<27}{:<18}{}",
                    "Station".yellow(),
                    "Scheduled Time".cyan(),
                    "Actual Time".green(),
                );
                for train in train_schedule.parse().iter() {
                    println!("{train}");
                }
                std::process::exit(0);
            }
            Err(err) => {
                eprintln!("An error occurred while getting train schedule: {}", err);
                std::process::exit(1);
            }
        }
    }
}
