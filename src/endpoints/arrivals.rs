use crate::traits::{Parse, PrettyPrint};
use crate::URL;
use anyhow::Result;
use colored::Colorize;
use reqwest::blocking::get;
use serde::Deserialize;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ArrivalsInner {
    direction: Option<String>,
    path: Option<String>,
    train_id: Option<String>,
    origin: Option<String>,
    destination: Option<String>,
    line: Option<String>,
    status: Option<String>,
    service_type: Option<String>,
    next_station: Option<String>,
    sched_time: Option<String>,
    depart_time: Option<String>,
    track: Option<String>,
    track_change: Option<String>,
    platform: Option<String>,
    platform_change: Option<String>,
}

#[derive(Deserialize)]
pub struct Arrivals(pub HashMap<String, Vec<HashMap<String, Vec<ArrivalsInner>>>>);

impl Arrivals {
    pub fn get(name: &str, num: u8) -> Result<Arrivals> {
        let request_url = format!("{}/Arrivals/index.php?station={}&results={}", URL, name, num,);
        let result: Arrivals = get(request_url)?.json()?;
        Ok(result)
    }
}

pub fn parse_time(time: Option<&str>) -> String {
    if time.is_none() {
        return "None".to_owned();
    }

    let time_vec: Vec<&str> = time.unwrap().split(" ").collect::<Vec<&str>>()[1].split(":").collect();
    let mut hour = time_vec[0].parse::<u8>().unwrap_or(0);
    let minute = time_vec[1].parse::<u8>().unwrap_or(0);
    let mut meridian = "AM";

    if hour > 12 {
        hour -= 12;
        meridian = "PM";
    }
    format!("{:02}:{:02} {}", hour, minute, meridian)
}

impl Parse for Arrivals {
    fn parse(&self) -> Vec<String> {
        let vec = self.0.values().next().unwrap();
        let north = &vec[0]["Northbound"];
        let south = &vec[1]["Southbound"];

        let mut trains: Vec<String> = north
            .into_iter()
            .map(|train| {
                format!(
                    "{:<13}{:<11}{:<27}{:<12}{:<10}{}",
                    "North",
                    train.train_id.as_ref().map_or("None", |orig| orig.as_str()),
                    train.next_station.as_ref().map_or("None", |orig| orig.as_str()),
                    parse_time(train.sched_time.as_ref().map(|s| s.as_str())),
                    train.status.as_ref().map_or("None", |orig| orig.as_str()),
                    train.destination.as_ref().map_or("None", |orig| orig.as_str())
                )
            })
            .collect();

        trains.append(
            &mut south
                .into_iter()
                .map(|train| {
                    format!(
                        "{:<13}{:<11}{:<27}{:<12}{:<10}{}",
                        "South",
                        train.train_id.as_ref().map_or("None", |orig| orig.as_str()),
                        train.next_station.as_ref().map_or("None", |orig| orig.as_str()),
                        parse_time(train.sched_time.as_ref().map(|s| s.as_str())),
                        train.status.as_ref().map_or("None", |orig| orig.as_str()),
                        train.destination.as_ref().map_or("None", |orig| orig.as_str())
                    )
                })
                .collect::<Vec<String>>(),
        );
        trains
    }
}

impl PrettyPrint for Arrivals {
    fn print(&self) {
        println!(
            "{:<13}{:<11}{:<27}{:<12}{:<10}{}",
            "Direction".blue(),
            "Train #".cyan(),
            "Next Station".green(),
            "Time".magenta(),
            "Status".red(),
            "Destination".yellow(),
        );
        for train in self.parse().iter() {
            println!("{train}");
        }
    }
}
