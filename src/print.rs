use crate::data::{Arrival, NextToArrive, TrainSchedule, BusAndTrolleyLocations};
use crate::parse::Parse;
use anyhow::Result;
use colored::Colorize;

pub trait Print {
    fn print(result: Result<Self>) -> Result<()>
    where
        Self: Sized;
}

impl Print for Arrival {
    fn print(result: Result<Self>) -> Result<()>{
        let arrival = result?;
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
        Ok(())
    }
}

impl Print for NextToArrive {
    fn print(result: Result<Self>) -> Result<()> {
        let next_to_arrive = result?;
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
        Ok(())
    }
}

impl Print for TrainSchedule {
    fn print(result: Result<Self>) -> Result<()>{
        let train_schedule = result?;
        println!(
            "{:<27}{:<18}{}",
            "Station".yellow(),
            "Scheduled Time".cyan(),
            "Actual Time".green(),
        );
        for train in train_schedule.parse().iter() {
            println!("{train}");
        }
        Ok(())
    }
}

impl Print for BusAndTrolleyLocations {
    fn print(result: Result<Self>) -> Result<()>{
        let train_schedule = result?;
        println!(
            "{:<5}{:<40}{:<15}{}",
            "Route".yellow(),
            "Next Stop".cyan(),
            "Direction".green(),
            "Destination".magenta()
        );
        for train in train_schedule.parse().iter() {
            println!("{train}");
        }
        Ok(())
    }
}
